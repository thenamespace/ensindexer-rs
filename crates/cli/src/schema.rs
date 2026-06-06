use std::{fs, path::PathBuf};

use anyhow::Context;
use api::build_schema_sdl;

mod local;
mod model;
mod official;

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
    let official_schema =
        official::fetch_official_schema(&subgraph_url, auth_token.as_deref()).await?;
    if let Some(path) = output {
        fs::write(&path, serde_json::to_string_pretty(&official_schema)?)
            .with_context(|| format!("failed to write schema to {}", path.display()))?;
    }

    let local = local::local_schema_summary();
    let official = official::schema_summary(&official_schema)?;
    let diff = local.diff(&official);
    diff.print();

    if diff.has_missing() {
        anyhow::bail!("local schema is missing official schema members");
    }

    Ok(())
}
