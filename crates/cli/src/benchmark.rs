use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use anyhow::Context;
use api::build_schema;
use reqwest::{Client, header};
use serde::Serialize;
use serde_json::{Value, json};
use storage::Storage;

#[derive(Debug, Clone)]
pub struct BenchmarkOptions {
    pub query_dir: PathBuf,
    pub iterations: usize,
    pub warmup: usize,
    pub local_compute: bool,
    pub local_url: Option<String>,
    pub official_url: Option<String>,
    pub official_auth_token: Option<String>,
    pub ensnode_url: Option<String>,
    pub ensnode_auth_token: Option<String>,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct BenchmarkReport {
    iterations: usize,
    warmup: usize,
    note: &'static str,
    results: Vec<QueryReport>,
}

#[derive(Debug, Serialize)]
struct QueryReport {
    name: String,
    source: String,
    response_bytes: usize,
    compute_ms: Option<Stats>,
    wall_ms: Option<Stats>,
    provider_ms: Option<Stats>,
}

#[derive(Debug, Serialize)]
struct Stats {
    min: f64,
    median: f64,
    p95: f64,
    max: f64,
    mean: f64,
}

struct QueryCase {
    name: String,
    query: String,
    variables: Value,
}

struct Endpoint {
    name: &'static str,
    url: String,
    auth_token: Option<String>,
}

pub async fn run(storage: Storage, options: BenchmarkOptions) -> anyhow::Result<()> {
    anyhow::ensure!(
        options.iterations > 0,
        "--iterations must be greater than zero"
    );
    let cases = read_query_cases(&options.query_dir)?;
    anyhow::ensure!(
        !cases.is_empty(),
        "no .graphql benchmark files found in {}",
        options.query_dir.display()
    );

    let client = Client::new();
    let schema = if options.local_compute {
        Some(build_schema(storage))
    } else {
        None
    };
    let endpoints = endpoints(&options);
    let mut results = Vec::new();

    for case in &cases {
        if let Some(schema) = &schema {
            results
                .push(run_local_compute(schema, case, options.iterations, options.warmup).await?);
        }
        for endpoint in &endpoints {
            results.push(
                run_endpoint(&client, endpoint, case, options.iterations, options.warmup).await?,
            );
        }
    }

    let report = BenchmarkReport {
        iterations: options.iterations,
        warmup: options.warmup,
        note: "compute_ms is only emitted for in-process local execution. endpoint wall_ms includes HTTP and network time. provider_ms is recorded only when a provider exposes timing in GraphQL extensions.",
        results,
    };

    let json = serde_json::to_string_pretty(&report)?;
    if let Some(output) = options.output {
        fs::write(&output, json)
            .with_context(|| format!("failed to write benchmark report {}", output.display()))?;
        println!("benchmark report written: {}", output.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

fn endpoints(options: &BenchmarkOptions) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();
    if let Some(url) = options.local_url.clone().filter(|url| !url.is_empty()) {
        endpoints.push(Endpoint {
            name: "local-http",
            url,
            auth_token: None,
        });
    }
    if let Some(url) = options.official_url.clone().filter(|url| !url.is_empty()) {
        endpoints.push(Endpoint {
            name: "official",
            url,
            auth_token: options.official_auth_token.clone(),
        });
    }
    if let Some(url) = options.ensnode_url.clone().filter(|url| !url.is_empty()) {
        endpoints.push(Endpoint {
            name: "ensnode",
            url,
            auth_token: options.ensnode_auth_token.clone(),
        });
    }
    endpoints
}

fn read_query_cases(dir: &Path) -> anyhow::Result<Vec<QueryCase>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let path = entry?.path();
        if path
            .extension()
            .is_some_and(|extension| extension == "graphql")
        {
            files.push(path);
        }
    }
    files.sort();

    files
        .into_iter()
        .map(|path| {
            let name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .context("invalid benchmark file name")?
                .to_owned();
            let query = fs::read_to_string(&path)
                .with_context(|| format!("failed to read query {}", path.display()))?;
            let variables = read_variables(&path.with_extension("variables.json"))?;
            Ok(QueryCase {
                name,
                query,
                variables,
            })
        })
        .collect()
}

fn read_variables(path: &Path) -> anyhow::Result<Value> {
    if !path.exists() {
        return Ok(json!({}));
    }
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read variables {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse variables {}", path.display()))
}

async fn run_local_compute(
    schema: &api::EnsSchema,
    case: &QueryCase,
    iterations: usize,
    warmup: usize,
) -> anyhow::Result<QueryReport> {
    for _ in 0..warmup {
        execute_local(schema, case).await?;
    }

    let mut elapsed = Vec::with_capacity(iterations);
    let mut response_bytes = 0;
    for _ in 0..iterations {
        let start = Instant::now();
        let response = execute_local(schema, case).await?;
        elapsed.push(start.elapsed());
        response_bytes = response.to_string().len();
    }

    Ok(QueryReport {
        name: case.name.clone(),
        source: "local-compute".to_owned(),
        response_bytes,
        compute_ms: Some(stats(elapsed)),
        wall_ms: None,
        provider_ms: None,
    })
}

async fn execute_local(schema: &api::EnsSchema, case: &QueryCase) -> anyhow::Result<Value> {
    let request = async_graphql::Request::new(case.query.clone())
        .variables(async_graphql::Variables::from_json(case.variables.clone()));
    let response = schema.execute(request).await;
    let value = serde_json::to_value(response)?;
    if value.get("errors").is_some() {
        anyhow::bail!(
            "local benchmark query {} returned errors: {}",
            case.name,
            serde_json::to_string_pretty(&value)?
        );
    }
    Ok(value)
}

async fn run_endpoint(
    client: &Client,
    endpoint: &Endpoint,
    case: &QueryCase,
    iterations: usize,
    warmup: usize,
) -> anyhow::Result<QueryReport> {
    for _ in 0..warmup {
        execute_endpoint(client, endpoint, case).await?;
    }

    let mut wall = Vec::with_capacity(iterations);
    let mut provider = Vec::new();
    let mut response_bytes = 0;
    for _ in 0..iterations {
        let start = Instant::now();
        let response = execute_endpoint(client, endpoint, case).await?;
        wall.push(start.elapsed());
        response_bytes = response.body_bytes;
        if let Some(provider_ms) = response.provider_ms {
            provider.push(Duration::from_secs_f64(provider_ms / 1000.0));
        }
    }

    Ok(QueryReport {
        name: case.name.clone(),
        source: endpoint.name.to_owned(),
        response_bytes,
        compute_ms: None,
        wall_ms: Some(stats(wall)),
        provider_ms: (!provider.is_empty()).then(|| stats(provider)),
    })
}

struct EndpointResponse {
    body_bytes: usize,
    provider_ms: Option<f64>,
}

async fn execute_endpoint(
    client: &Client,
    endpoint: &Endpoint,
    case: &QueryCase,
) -> anyhow::Result<EndpointResponse> {
    let mut request = client
        .post(&endpoint.url)
        .header(header::USER_AGENT, "ensindexer-benchmark/0.1")
        .header(header::ACCEPT, "application/json")
        .json(&json!({
            "query": case.query,
            "variables": case.variables,
        }));

    if let Some(token) = endpoint
        .auth_token
        .as_deref()
        .filter(|token| !token.is_empty())
    {
        request = request.bearer_auth(token);
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("failed to send {} to {}", case.name, endpoint.name))?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        anyhow::bail!(
            "{} query {} failed with {status}: {body}",
            endpoint.name,
            case.name
        );
    }
    let value: Value = serde_json::from_str(&body).with_context(|| {
        format!(
            "failed to parse {} response for {}",
            endpoint.name, case.name
        )
    })?;
    if value.get("errors").is_some() {
        anyhow::bail!(
            "{} query {} returned errors: {}",
            endpoint.name,
            case.name,
            serde_json::to_string_pretty(&value)?
        );
    }

    Ok(EndpointResponse {
        body_bytes: body.len(),
        provider_ms: provider_ms(&value),
    })
}

fn provider_ms(value: &Value) -> Option<f64> {
    let extensions = value.get("extensions")?;
    extensions
        .pointer("/tracing/duration")
        .and_then(Value::as_f64)
        .map(|nanoseconds| nanoseconds / 1_000_000.0)
        .or_else(|| extensions.get("durationMs").and_then(Value::as_f64))
        .or_else(|| extensions.get("executionMs").and_then(Value::as_f64))
}

fn stats(samples: Vec<Duration>) -> Stats {
    let mut values: Vec<_> = samples
        .into_iter()
        .map(|sample| sample.as_secs_f64() * 1000.0)
        .collect();
    values.sort_by(f64::total_cmp);
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Stats {
        min: values[0],
        median: percentile(&values, 0.50),
        p95: percentile(&values, 0.95),
        max: values[values.len() - 1],
        mean,
    }
}

fn percentile(values: &[f64], percentile: f64) -> f64 {
    let index = ((values.len() - 1) as f64 * percentile).round() as usize;
    values[index]
}
