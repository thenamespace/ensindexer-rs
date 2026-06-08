use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context;
use config::AppConfig;
use reqwest::{Client, Url};
use serde::Deserialize;
use storage::Storage;
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};

#[derive(Debug, Clone)]
pub(crate) struct HealOptions {
    pub labelhashes: Vec<String>,
    pub limit: i64,
    pub concurrency: usize,
    pub repair_passes: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
enum HealResponse {
    Success {
        label: String,
    },
    Error {
        #[serde(rename = "errorCode")]
        error_code: i32,
    },
}

#[derive(Debug)]
enum HealOutcome {
    Found { labelhash: String, label: String },
    Missing { labelhash: String },
}

pub(crate) async fn run(
    config: AppConfig,
    storage: Storage,
    options: HealOptions,
) -> anyhow::Result<()> {
    let labelhashes = if options.labelhashes.is_empty() {
        storage
            .label_preimages()
            .candidate_labelhashes(options.limit)
            .await?
    } else {
        options.labelhashes
    };
    if labelhashes.is_empty() {
        println!("label heal candidates: 0");
        return Ok(());
    }

    println!(
        "label heal candidates: {} concurrency: {}",
        labelhashes.len(),
        options.concurrency
    );

    let client = Client::builder().timeout(Duration::from_secs(20)).build()?;
    let semaphore = Arc::new(Semaphore::new(options.concurrency.max(1)));
    let mut tasks = JoinSet::new();

    for labelhash in labelhashes {
        let permit = Arc::clone(&semaphore).acquire_owned().await?;
        let client = client.clone();
        let base_url = config.ensrainbow_url.clone();
        tasks.spawn(async move {
            let result = heal_labelhash(&client, &base_url, labelhash).await;
            drop(permit);
            result
        });
    }

    let mut found = Vec::new();
    let mut missing = Vec::new();
    while let Some(result) = tasks.join_next().await {
        match result?? {
            HealOutcome::Found { labelhash, label } => {
                if types::validate_label(&label).is_ok() {
                    found.push((labelhash, label));
                } else {
                    missing.push(labelhash);
                }
            }
            HealOutcome::Missing { labelhash } => missing.push(labelhash),
        }
    }

    storage.label_preimages().upsert_many(&found).await?;
    storage.label_preimages().record_misses(&missing).await?;
    let repair_start = Instant::now();
    let found_labelhashes = found
        .iter()
        .map(|(labelhash, _)| labelhash.clone())
        .collect::<Vec<_>>();
    let repaired = storage
        .label_preimages()
        .repair_domain_names_for_labelhashes(&found_labelhashes, options.repair_passes)
        .await?;

    println!(
        "label heal complete: found={} missing={} repaired_rows={} repair_ms={}",
        found.len(),
        missing.len(),
        repaired,
        repair_start.elapsed().as_millis()
    );
    Ok(())
}

async fn heal_labelhash(
    client: &Client,
    base_url: &Url,
    labelhash: String,
) -> anyhow::Result<HealOutcome> {
    let mut last_error = None;
    for attempt in 1..=4 {
        match heal_once(client, base_url, &labelhash).await {
            Ok(outcome) => return Ok(outcome),
            Err(err) => {
                last_error = Some(err);
                if attempt < 4 {
                    sleep(Duration::from_millis(250 * attempt)).await;
                }
            }
        }
    }

    Err(last_error.expect("retry loop records an error"))
}

async fn heal_once(
    client: &Client,
    base_url: &Url,
    labelhash: &str,
) -> anyhow::Result<HealOutcome> {
    let url = base_url.join(&format!("/v1/heal/{labelhash}"))?;
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("ENSRainbow request failed for {labelhash}"))?;
    let body = response.json::<HealResponse>().await?;

    match body {
        HealResponse::Success { label } => Ok(HealOutcome::Found {
            labelhash: labelhash.to_owned(),
            label,
        }),
        HealResponse::Error { error_code: 404 } => Ok(HealOutcome::Missing {
            labelhash: labelhash.to_owned(),
        }),
        HealResponse::Error { error_code } => {
            anyhow::bail!("ENSRainbow error for {labelhash}: {error_code}")
        }
    }
}
