use std::{fs, path::PathBuf};

use anyhow::Context;
use reqwest::{Client, header};
use serde_json::{Value, json};

pub async fn run(
    local_url: String,
    subgraph_url: String,
    auth_token: Option<String>,
    query_file: PathBuf,
    variables_file: Option<PathBuf>,
    operation_name: Option<String>,
    show_json: bool,
) -> anyhow::Result<()> {
    let query = fs::read_to_string(&query_file)
        .with_context(|| format!("failed to read query file {}", query_file.display()))?;
    let variables = read_variables(variables_file)?;

    let client = Client::new();
    let local = execute(
        &client,
        &local_url,
        None,
        &query,
        &variables,
        &operation_name,
    )
    .await?;
    let official = execute(
        &client,
        &subgraph_url,
        auth_token.as_deref(),
        &query,
        &variables,
        &operation_name,
    )
    .await?;

    if local != official {
        eprintln!("local response:\n{}", serde_json::to_string_pretty(&local)?);
        eprintln!(
            "official response:\n{}",
            serde_json::to_string_pretty(&official)?
        );
        anyhow::bail!("GraphQL responses differ");
    }

    if show_json {
        println!("{}", serde_json::to_string_pretty(&local)?);
    } else {
        println!("GraphQL responses match");
    }

    Ok(())
}

fn read_variables(path: Option<PathBuf>) -> anyhow::Result<Value> {
    match path {
        Some(path) => {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("failed to read variables file {}", path.display()))?;
            serde_json::from_str(&contents)
                .with_context(|| format!("failed to parse variables file {}", path.display()))
        }
        None => Ok(json!({})),
    }
}

async fn execute(
    client: &Client,
    url: &str,
    bearer: Option<&str>,
    query: &str,
    variables: &Value,
    operation_name: &Option<String>,
) -> anyhow::Result<Value> {
    let mut body = json!({
        "query": query,
        "variables": variables,
    });
    if let Some(operation_name) = operation_name
        .as_deref()
        .filter(|operation_name| !operation_name.trim().is_empty())
    {
        body["operationName"] = json!(operation_name);
    }

    let mut request = client
        .post(url)
        .header(header::USER_AGENT, "curl/8.0.0")
        .header(header::ACCEPT, "application/json")
        .json(&body);

    if let Some(token) = bearer.filter(|token| !token.trim().is_empty()) {
        request = request.bearer_auth(token);
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("failed to send GraphQL request to {url}"))?;
    let status = response.status();
    let body = response
        .text()
        .await
        .with_context(|| format!("failed to read GraphQL response from {url}"))?;

    if !status.is_success() {
        anyhow::bail!("GraphQL request to {url} failed with {status}: {body}");
    }

    serde_json::from_str(&body).with_context(|| format!("failed to parse JSON response from {url}"))
}
