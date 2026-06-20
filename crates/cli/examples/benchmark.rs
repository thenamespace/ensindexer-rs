use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use figment::{Figment, providers::Env};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{Value, json};

const WARMUPS: usize = 5;
const ITERATIONS: usize = 25;
const TIMEOUT_MS: u64 = 100_000;
const QUERY_DIR: &str = "benchmarks/queries";
const OUTPUT_PATH: &str = "BENCHMARK.md";
const NETWORK_BASELINE_QUERY: &str = "{ _meta { block { number } } }";
const USE_COMPUTE_ONLY_TIMING: bool = false;
const LOG_QUERY_ERRORS: bool = true;
const TIMING_MODE: TimingMode = timing_mode();

const ENSNODE_OVERRIDES: &[QueryOverride] = &[
    QueryOverride {
        from: "$expiry: String!",
        to: "$expiry: BigInt!",
    },
    QueryOverride {
        from: "$ethNode: ID!",
        to: "$ethNode: String!",
    },
    QueryOverride {
        from: "_contains_nocase",
        to: "_contains",
    },
    QueryOverride {
        from: "_starts_with_nocase",
        to: "_starts_with",
    },
    QueryOverride {
        from: "_ends_with_nocase",
        to: "_ends_with",
    },
];

const ENTRIES: &[BenchmarkEntry] = &[
    BenchmarkEntry {
        name: "ensindexer-rs (Local)",
        url: "http://127.0.0.1:8080/subgraph",
        auth_bearer_env: None,
        overrides: &[],
    },
    // BenchmarkEntry {
    //     name: "ensindexer-rs (Hosted)",
    //     url: "https://ensindexer-rs.namespace.ninja/subgraph",
    //     auth_bearer_env: None,
    //     overrides: &[],
    // },
    BenchmarkEntry {
        name: "ensnode",
        url: "https://api.alpha.ensnode.io/subgraph",
        auth_bearer_env: None,
        overrides: ENSNODE_OVERRIDES,
    },
    BenchmarkEntry {
        name: "the graph indexer",
        url: "https://gateway.thegraph.com/api/subgraphs/id/5XqPmWe6gjyrJtFn9cLy237i4cWw2j9HcUJEXsP5qGtH",
        auth_bearer_env: Some("SUBGRAPH_AUTH_TOKEN"),
        overrides: &[],
    },
];

#[derive(Debug, Clone, Copy)]
enum TimingMode {
    Raw,
    ComputeOnly,
}

const fn timing_mode() -> TimingMode {
    if USE_COMPUTE_ONLY_TIMING {
        TimingMode::ComputeOnly
    } else {
        TimingMode::Raw
    }
}

#[derive(Debug, Clone, Copy)]
struct BenchmarkEntry {
    name: &'static str,
    url: &'static str,
    auth_bearer_env: Option<&'static str>,
    overrides: &'static [QueryOverride],
}

#[derive(Debug, Clone, Copy)]
struct QueryOverride {
    from: &'static str,
    to: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse()?;
    dotenvy::dotenv().ok();
    let env = EnvConfig::load()?;
    let entries = active_entries(args.only.as_deref())?;
    let queries = read_queries(QUERY_DIR)?;
    let client = Client::builder()
        .timeout(Duration::from_millis(TIMEOUT_MS))
        .build()?;

    let baselines = run_baselines(&client, &env, &entries).await?;

    let mut rows = Vec::new();
    for query in queries {
        let cells = run_query_for_all_entries(&client, &env, &entries, &baselines, &query).await?;
        rows.push(OperationRow {
            operation: query.id,
            cells,
        });
    }

    let report = render_report(&entries, &baselines, &rows);
    fs::write(OUTPUT_PATH, &report)?;
    println!("Wrote {OUTPUT_PATH}");
    println!("{report}");

    Ok(())
}

async fn run_baselines(
    client: &Client,
    env: &EnvConfig,
    entries: &[&'static BenchmarkEntry],
) -> Result<Vec<EndpointBaseline>> {
    let mut tasks = tokio::task::JoinSet::new();
    for (index, entry) in entries.iter().enumerate() {
        let entry = *entry;
        let client = client.clone();
        let env = env.clone();
        tasks.spawn(async move { Ok((index, run_baseline(&client, &env, entry).await?)) });
    }
    collect_indexed(tasks, entries.len()).await
}

async fn run_baseline(
    client: &Client,
    env: &EnvConfig,
    entry: &'static BenchmarkEntry,
) -> Result<EndpointBaseline> {
    let query = QueryFixture {
        id: "_baseline".to_string(),
        query: NETWORK_BASELINE_QUERY.to_string(),
        variables: Value::Object(Default::default()),
    };
    let mut samples = Vec::with_capacity(ITERATIONS);
    for iteration in 0..(WARMUPS + ITERATIONS) {
        if let Sample::Ok(sample) = post_graphql(client, env, entry, &query).await {
            if iteration >= WARMUPS {
                samples.push(sample.raw_ms);
            }
        }
    }

    Ok(EndpointBaseline {
        name: entry.name,
        url: entry.url,
        median_ms: median(&samples),
        p95_ms: percentile(&samples, 95.0),
    })
}

async fn run_query_for_all_entries(
    client: &Client,
    env: &EnvConfig,
    entries: &[&'static BenchmarkEntry],
    baselines: &[EndpointBaseline],
    query: &QueryFixture,
) -> Result<Vec<BenchmarkCell>> {
    let mut tasks = tokio::task::JoinSet::new();
    for (index, (entry, baseline)) in entries.iter().zip(baselines.iter()).enumerate() {
        let entry = *entry;
        let client = client.clone();
        let env = env.clone();
        let query = query.clone();
        let baseline = baseline.clone();
        tasks.spawn(async move {
            Ok((
                index,
                run_query(&client, &env, entry, &baseline, &query).await,
            ))
        });
    }
    collect_indexed(tasks, entries.len()).await
}

async fn run_query(
    client: &Client,
    env: &EnvConfig,
    entry: &'static BenchmarkEntry,
    baseline: &EndpointBaseline,
    query: &QueryFixture,
) -> BenchmarkCell {
    let mut samples = Vec::with_capacity(ITERATIONS);
    let mut last_status = CellStatus::Error("no successful samples".to_string());

    for iteration in 0..(WARMUPS + ITERATIONS) {
        match post_graphql(client, env, entry, query).await {
            Sample::Ok(sample) => {
                if iteration >= WARMUPS {
                    samples.push(match TIMING_MODE {
                        TimingMode::Raw => sample.raw_ms,
                        TimingMode::ComputeOnly => (sample.raw_ms - baseline.median_ms).max(0.0),
                    });
                }
            }
            Sample::Unsupported(reason) => {
                log_query_error(entry, query, "unsupported", &reason);
                return BenchmarkCell::unsupported();
            }
            Sample::Timeout => last_status = CellStatus::Timeout,
            Sample::Error(reason) => last_status = CellStatus::Error(reason),
        }
    }

    if samples.is_empty() {
        log_cell_status(entry, query, &last_status);
        return BenchmarkCell {
            status: last_status,
            median_ms: None,
        };
    }

    BenchmarkCell {
        status: CellStatus::Ok,
        median_ms: Some(median(&samples)),
    }
}

async fn post_graphql(
    client: &Client,
    env: &EnvConfig,
    entry: &BenchmarkEntry,
    query: &QueryFixture,
) -> Sample {
    let headers = match headers(env, entry) {
        Ok(headers) => headers,
        Err(error) => return Sample::Error(error.to_string()),
    };
    let started = Instant::now();
    let response = client
        .post(entry.url)
        .headers(headers)
        .json(&json!({
            "query": entry.query(&query.query),
            "variables": query.variables,
        }))
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(error) if error.is_timeout() => return Sample::Timeout,
        Err(error) => return Sample::Error(error.to_string()),
    };

    let raw_ms = started.elapsed().as_secs_f64() * 1000.0;
    let status = response.status();
    let text = match response.text().await {
        Ok(text) => text,
        Err(error) => return Sample::Error(error.to_string()),
    };

    if !status.is_success() {
        return Sample::Error(format!(
            "http {status}: {}",
            text.chars().take(160).collect::<String>()
        ));
    }

    let body: Value = match serde_json::from_str(&text) {
        Ok(body) => body,
        Err(error) => return Sample::Error(error.to_string()),
    };

    if let Some(errors) = body.get("errors").and_then(Value::as_array) {
        let messages = errors
            .iter()
            .filter_map(|error| error.get("message").and_then(Value::as_str))
            .collect::<Vec<_>>();
        let reason = messages.join("; ");
        if messages.iter().all(|message| is_unsupported_error(message)) {
            return Sample::Unsupported(reason);
        }
        return Sample::Error(reason);
    }

    Sample::Ok(HttpSample { raw_ms })
}

fn headers(env: &EnvConfig, entry: &BenchmarkEntry) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    if let Some(env_name) = entry.auth_bearer_env {
        let token = env
            .required(env_name)
            .with_context(|| format!("{} requires {env_name}", entry.name))?;
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {token}"))?,
        );
    }
    Ok(headers)
}

impl BenchmarkEntry {
    fn query(&self, query: &str) -> String {
        let mut rewritten = query.to_string();
        for override_rule in self.overrides {
            rewritten = rewritten.replace(override_rule.from, override_rule.to);
        }
        rewritten
    }
}

#[derive(Debug, Default)]
struct Args {
    only: Option<String>,
}

impl Args {
    fn parse() -> Result<Self> {
        let mut args = std::env::args().skip(1);
        let mut parsed = Args::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--only" => {
                    parsed.only = Some(
                        args.next()
                            .context("--only requires an endpoint name substring")?,
                    );
                }
                "--help" | "-h" => {
                    println!(
                        "Usage: cargo run -p cli --release --example benchmark -- [--only <name-substring>]"
                    );
                    std::process::exit(0);
                }
                other => bail!("unknown argument {other}"),
            }
        }
        Ok(parsed)
    }
}

fn active_entries(only: Option<&str>) -> Result<Vec<&'static BenchmarkEntry>> {
    let entries = ENTRIES
        .iter()
        .filter(|entry| {
            only.is_none_or(|only| entry.name.to_lowercase().contains(&only.to_lowercase()))
        })
        .collect::<Vec<_>>();
    if entries.is_empty() {
        bail!("no benchmark entries matched --only {:?}", only);
    }
    Ok(entries)
}

async fn collect_indexed<T: 'static>(
    mut tasks: tokio::task::JoinSet<Result<(usize, T)>>,
    len: usize,
) -> Result<Vec<T>> {
    let mut values = (0..len).map(|_| None).collect::<Vec<_>>();
    while let Some(result) = tasks.join_next().await {
        let (index, value) = result??;
        values[index] = Some(value);
    }
    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| value.with_context(|| format!("missing benchmark result {index}")))
        .collect()
}

fn read_queries(dir: &str) -> Result<Vec<QueryFixture>> {
    let mut paths = fs::read_dir(dir)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.sort();

    let queries = paths
        .into_iter()
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "graphql")
        })
        .map(read_query)
        .collect::<Result<Vec<_>>>()?;
    if queries.is_empty() {
        bail!("no .graphql files found in {dir}");
    }
    Ok(queries)
}

fn read_query(path: PathBuf) -> Result<QueryFixture> {
    let id = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .with_context(|| format!("invalid query filename {}", path.display()))?
        .to_string();
    let query = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let variables = read_variables(&path.with_extension("variables.json"))?;
    Ok(QueryFixture {
        id,
        query,
        variables,
    })
}

fn read_variables(path: &Path) -> Result<Value> {
    match fs::read_to_string(path) {
        Ok(raw) => serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(Value::Object(Default::default()))
        }
        Err(error) => Err(error).with_context(|| format!("read {}", path.display())),
    }
}

fn render_report(
    entries: &[&'static BenchmarkEntry],
    baselines: &[EndpointBaseline],
    rows: &[OperationRow],
) -> String {
    let mut lines = Vec::new();
    lines.push("# ENS Indexer Benchmark".to_string());
    lines.push(String::new());
    lines.push(format!("Generated at `{}`.", timestamp()));
    lines.push(String::new());
    lines.push(format!(
        "Settings: `{WARMUPS}` warmups, `{ITERATIONS}` measured iterations, `{TIMEOUT_MS}`ms timeout, `{}` timing.",
        timing_label(TIMING_MODE)
    ));
    lines.push(String::new());
    lines.push("| endpoint | URL | network baseline median | network baseline p95 |".to_string());
    lines.push("| --- | --- | ---: | ---: |".to_string());
    for baseline in baselines {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            escape_cell(baseline.name),
            escape_cell(baseline.url),
            format_ms(baseline.median_ms),
            format_ms(baseline.p95_ms)
        ));
    }
    lines.push(String::new());
    lines.push(render_table(entries, rows));
    lines.push(String::new());
    lines.push(
        "Relative speed is calculated against the slowest supported numeric result in each row. Unsupported, timeout, and error cells are excluded from the numeric baseline."
            .to_string(),
    );
    lines.join("\n")
}

fn render_table(entries: &[&'static BenchmarkEntry], rows: &[OperationRow]) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "| operation | {} |",
        entries
            .iter()
            .map(|entry| escape_cell(entry.name))
            .collect::<Vec<_>>()
            .join(" | ")
    ));
    lines.push(format!(
        "| --- | {} |",
        entries
            .iter()
            .map(|_| "---:")
            .collect::<Vec<_>>()
            .join(" | ")
    ));

    for row in rows {
        let slowest = row
            .cells
            .iter()
            .filter_map(|cell| cell.median_ms)
            .reduce(f64::max);
        let cells = row
            .cells
            .iter()
            .map(|cell| render_cell(cell, slowest))
            .collect::<Vec<_>>()
            .join(" | ");
        lines.push(format!("| `{}` | {} |", row.operation, cells));
    }
    lines.join("\n")
}

fn render_cell(cell: &BenchmarkCell, slowest: Option<f64>) -> String {
    match &cell.status {
        CellStatus::Ok => {
            let value = cell.median_ms.unwrap_or_default();
            let Some(slowest) = slowest else {
                return format_ms(value);
            };
            if slowest <= 0.0 || (value - slowest).abs() < f64::EPSILON {
                return format!("{} (slowest)", format_ms(value));
            }
            if value <= 0.0 {
                return format!("{} (near-baseline/noisy)", format_ms(value));
            }
            let faster = slowest / value;
            format!("{} ({faster:.1}x)", format_ms(value))
        }
        CellStatus::Unsupported(reason) => {
            let _ = reason;
            "unsupported".to_string()
        }
        CellStatus::Timeout => "timeout".to_string(),
        CellStatus::Error(reason) => {
            let _ = reason;
            "error".to_string()
        }
    }
}

fn log_cell_status(entry: &BenchmarkEntry, query: &QueryFixture, status: &CellStatus) {
    match status {
        CellStatus::Unsupported(reason) => log_query_error(entry, query, "unsupported", reason),
        CellStatus::Timeout => log_query_error(entry, query, "timeout", "request timed out"),
        CellStatus::Error(reason) => log_query_error(entry, query, "error", reason),
        CellStatus::Ok => {}
    }
}

fn log_query_error(entry: &BenchmarkEntry, query: &QueryFixture, status: &str, reason: &str) {
    if LOG_QUERY_ERRORS {
        eprintln!(
            "[benchmark:{status}] endpoint={} operation={} reason={}",
            entry.name, query.id, reason
        );
    }
}

fn is_unsupported_error(message: &str) -> bool {
    [
        "Cannot query field",
        "Unknown argument",
        "Unknown type",
        "is not defined",
        "Expected value of type",
        "used in position expecting type",
    ]
    .iter()
    .any(|needle| message.contains(needle))
}

fn median(values: &[f64]) -> f64 {
    percentile(values, 50.0)
}

fn percentile(values: &[f64], percentile: f64) -> f64 {
    let mut sorted = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();
    if sorted.is_empty() {
        return 0.0;
    }
    sorted.sort_by(|left, right| left.total_cmp(right));
    let index = ((percentile / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[index.saturating_sub(1).min(sorted.len() - 1)]
}

fn timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}s since Unix epoch", duration.as_secs()),
        Err(_) => "before Unix epoch".to_string(),
    }
}

fn timing_label(mode: TimingMode) -> &'static str {
    match mode {
        TimingMode::Raw => "raw",
        TimingMode::ComputeOnly => "computeOnly",
    }
}

fn format_ms(value: f64) -> String {
    format!("{value:.3}ms")
}

fn escape_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

#[derive(Debug, Clone)]
struct EnvConfig {
    values: BTreeMap<String, Value>,
}

impl EnvConfig {
    fn load() -> Result<Self> {
        let values = Figment::new().merge(Env::raw()).extract()?;
        Ok(Self { values })
    }

    fn required(&self, key: &str) -> Result<String> {
        self.optional(key)
            .with_context(|| format!("missing required environment variable {key}"))
    }

    fn optional(&self, key: &str) -> Option<String> {
        self.values
            .get(key)
            .or_else(|| self.values.get(&key.to_ascii_lowercase()))
            .or_else(|| self.values.get(&key.to_ascii_uppercase()))
            .and_then(env_value_to_string)
            .filter(|value| !value.is_empty())
    }
}

fn env_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(value) => Some(value.trim().to_owned()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        other => Some(other.to_string()),
    }
}

#[derive(Debug, Clone)]
struct EndpointBaseline {
    name: &'static str,
    url: &'static str,
    median_ms: f64,
    p95_ms: f64,
}

#[derive(Debug)]
struct OperationRow {
    operation: String,
    cells: Vec<BenchmarkCell>,
}

#[derive(Debug)]
struct BenchmarkCell {
    status: CellStatus,
    median_ms: Option<f64>,
}

impl BenchmarkCell {
    fn unsupported() -> Self {
        Self {
            status: CellStatus::Unsupported(String::new()),
            median_ms: None,
        }
    }
}

#[derive(Debug)]
enum CellStatus {
    Ok,
    Unsupported(String),
    Timeout,
    Error(String),
}

#[derive(Debug, Clone)]
struct QueryFixture {
    id: String,
    query: String,
    variables: Value,
}

#[derive(Debug)]
enum Sample {
    Ok(HttpSample),
    Unsupported(String),
    Timeout,
    Error(String),
}

#[derive(Debug)]
struct HttpSample {
    raw_ms: f64,
}
