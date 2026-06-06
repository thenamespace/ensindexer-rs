use std::collections::{BTreeMap, BTreeSet};

use reqwest::{Client, header};
use serde_json::{Value, json};

use super::model::SchemaSummary;

pub(super) const INTROSPECTION_QUERY: &str = r#"
query IntrospectionQuery {
    __schema {
      queryType {
        fields {
          name
          args {
            name
            type {
              kind
              name
              ofType {
                kind
                name
                ofType {
                  kind
                  name
                  ofType {
                    kind
                    name
                    ofType {
                      kind
                      name
                    }
                  }
                }
              }
            }
          }
        }
      }
      types {
        kind
        name
        inputFields {
          name
        }
        enumValues {
          name
        }
      }
    }
  }
"#;

pub(super) async fn fetch_official_schema(
    url: &str,
    bearer: Option<&str>,
) -> anyhow::Result<Value> {
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

pub(super) fn schema_summary(schema: &Value) -> anyhow::Result<SchemaSummary> {
    let mut query_arg_types = BTreeMap::new();
    let query_fields = schema
        .pointer("/queryType/fields")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow::anyhow!("official schema has no query fields"))?
        .iter()
        .filter_map(|field| {
            let name = field.get("name").and_then(Value::as_str)?;
            let mut args = BTreeSet::new();
            for arg in field
                .get("args")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let Some(arg_name) = arg.get("name").and_then(Value::as_str) else {
                    continue;
                };
                args.insert(arg_name.to_owned());
                if let Some(ty) = arg.get("type") {
                    query_arg_types.insert(format!("{name}.{arg_name}"), format_type_ref(ty));
                }
            }
            Some((name.to_owned(), args))
        })
        .collect();

    let mut input_types = BTreeMap::new();
    let mut enum_types = BTreeMap::new();
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
                input_types.insert(name.to_owned(), type_member_names(ty, "inputFields"));
            }
            Some("ENUM") => {
                enum_types.insert(name.to_owned(), type_member_names(ty, "enumValues"));
            }
            _ => {}
        }
    }

    Ok(SchemaSummary {
        query_fields,
        query_arg_types,
        input_types,
        enum_types,
    })
}

fn format_type_ref(ty: &Value) -> String {
    match ty.get("kind").and_then(Value::as_str) {
        Some("NON_NULL") => format!(
            "{}!",
            ty.get("ofType").map(format_type_ref).unwrap_or_default()
        ),
        Some("LIST") => format!(
            "[{}]",
            ty.get("ofType").map(format_type_ref).unwrap_or_default()
        ),
        _ => ty
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
    }
}

fn type_member_names(ty: &Value, key: &str) -> BTreeSet<String> {
    ty.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|field| field.get("name").and_then(Value::as_str))
        .map(str::to_owned)
        .collect()
}
