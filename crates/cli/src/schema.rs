use std::{collections::BTreeSet, fs, path::PathBuf};

use anyhow::Context;
use api::build_schema_sdl;
use reqwest::{Client, header};
use serde_json::{Value, json};

const INTROSPECTION_QUERY: &str = r#"
query IntrospectionQuery {
  __schema {
    queryType {
      fields {
        name
      }
    }
    types {
      kind
      name
    }
  }
}
"#;

pub async fn print_local_sdl(output: Option<PathBuf>) -> anyhow::Result<()> {
    let sdl = build_schema_sdl();
    match output {
        Some(path) => {
            fs::write(&path, sdl)
                .with_context(|| format!("failed to write SDL to {}", path.display()))?;
        }
        None => println!("{sdl}"),
    }
    Ok(())
}

pub async fn diff_official(
    subgraph_url: String,
    auth_token: Option<String>,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let official = fetch_official_schema(&subgraph_url, auth_token.as_deref()).await?;
    if let Some(path) = output {
        fs::write(&path, serde_json::to_string_pretty(&official)?)
            .with_context(|| format!("failed to write schema to {}", path.display()))?;
    }

    let local = local_schema_summary();
    let official = schema_summary(&official)?;
    let diff = local.diff(&official);
    diff.print();

    if diff.has_missing() {
        anyhow::bail!("local schema is missing official schema members");
    }

    Ok(())
}

async fn fetch_official_schema(url: &str, bearer: Option<&str>) -> anyhow::Result<Value> {
    let mut request = Client::new()
        .post(url)
        .header(header::USER_AGENT, "curl/8.0.0")
        .header(header::ACCEPT, "application/json")
        .json(&json!({
            "query": INTROSPECTION_QUERY,
            "operationName": "IntrospectionQuery",
            "variables": {},
        }));

    if let Some(token) = bearer.filter(|token| !token.trim().is_empty()) {
        request = request.bearer_auth(token);
    }

    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        anyhow::bail!("official schema request failed with {status}: {body}");
    }

    let body: Value = serde_json::from_str(&body)?;
    if let Some(errors) = body.get("errors") {
        anyhow::bail!("official schema request returned errors: {errors}");
    }
    body.pointer("/data/__schema")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("official schema response did not contain __schema"))
}

fn local_schema_summary() -> SchemaSummary {
    let sdl = build_schema_sdl();
    let query_fields = parse_query_fields(&sdl);
    let mut input_types = BTreeSet::new();
    let mut enum_types = BTreeSet::new();
    for line in sdl.lines() {
        if let Some(name) = line.strip_prefix("input ").and_then(first_token) {
            input_types.insert(name.to_owned());
        } else if let Some(name) = line.strip_prefix("enum ").and_then(first_token) {
            enum_types.insert(name.to_owned());
        }
    }

    SchemaSummary {
        query_fields,
        input_types,
        enum_types,
    }
}

fn parse_query_fields(sdl: &str) -> BTreeSet<String> {
    let mut fields = BTreeSet::new();
    let mut in_query = false;
    for line in sdl.lines() {
        let trimmed = line.trim();
        if trimmed == "type QueryRoot {" {
            in_query = true;
            continue;
        }
        if in_query && trimmed == "}" {
            break;
        }
        if !in_query || trimmed.is_empty() {
            continue;
        }
        let Some(name) = trimmed.split(['(', ':']).next() else {
            continue;
        };
        fields.insert(name.to_owned());
    }
    fields
}

fn first_token(value: &str) -> Option<&str> {
    value
        .split([' ', '{'])
        .next()
        .filter(|name| !name.trim().is_empty())
}

fn schema_summary(schema: &Value) -> anyhow::Result<SchemaSummary> {
    let query_fields = schema
        .pointer("/queryType/fields")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow::anyhow!("official schema has no query fields"))?
        .iter()
        .filter_map(|field| field.get("name").and_then(Value::as_str))
        .map(str::to_owned)
        .collect();

    let mut input_types = BTreeSet::new();
    let mut enum_types = BTreeSet::new();
    for ty in schema
        .get("types")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow::anyhow!("official schema has no types"))?
    {
        let Some(name) = ty.get("name").and_then(Value::as_str) else {
            continue;
        };
        match ty.get("kind").and_then(Value::as_str) {
            Some("INPUT_OBJECT") => {
                input_types.insert(name.to_owned());
            }
            Some("ENUM") => {
                enum_types.insert(name.to_owned());
            }
            _ => {}
        }
    }

    Ok(SchemaSummary {
        query_fields,
        input_types,
        enum_types,
    })
}

struct SchemaSummary {
    query_fields: BTreeSet<String>,
    input_types: BTreeSet<String>,
    enum_types: BTreeSet<String>,
}

impl SchemaSummary {
    fn diff(&self, official: &Self) -> SchemaDiff {
        SchemaDiff {
            missing_query_fields: difference(&official.query_fields, &self.query_fields),
            extra_query_fields: difference(&self.query_fields, &official.query_fields),
            missing_input_types: difference(&official.input_types, &self.input_types),
            extra_input_types: difference(&self.input_types, &official.input_types),
            missing_enum_types: difference(&official.enum_types, &self.enum_types),
            extra_enum_types: difference(&self.enum_types, &official.enum_types),
        }
    }
}

struct SchemaDiff {
    missing_query_fields: Vec<String>,
    extra_query_fields: Vec<String>,
    missing_input_types: Vec<String>,
    extra_input_types: Vec<String>,
    missing_enum_types: Vec<String>,
    extra_enum_types: Vec<String>,
}

impl SchemaDiff {
    fn has_missing(&self) -> bool {
        !self.missing_query_fields.is_empty()
            || !self.missing_input_types.is_empty()
            || !self.missing_enum_types.is_empty()
    }

    fn print(&self) {
        print_section("missing query fields", &self.missing_query_fields);
        print_section("extra query fields", &self.extra_query_fields);
        print_section("missing input types", &self.missing_input_types);
        print_section("extra input types", &self.extra_input_types);
        print_section("missing enum types", &self.missing_enum_types);
        print_section("extra enum types", &self.extra_enum_types);
    }
}

fn difference(left: &BTreeSet<String>, right: &BTreeSet<String>) -> Vec<String> {
    left.difference(right).cloned().collect()
}

fn print_section(label: &str, values: &[String]) {
    println!("{label}: {}", values.len());
    for value in values.iter().take(40) {
        println!("  {value}");
    }
    if values.len() > 40 {
        println!("  ... {} more", values.len() - 40);
    }
}
