use config::AppConfig;
use ingest::IngestService;
use storage::Storage;

use crate::http;

pub async fn serve(config: AppConfig, storage: Storage) -> anyhow::Result<()> {
    if config.serve_indexer {
        serve_with_indexer(config, storage).await
    } else {
        http::serve_http(config, storage).await
    }
}

async fn serve_with_indexer(config: AppConfig, storage: Storage) -> anyhow::Result<()> {
    tracing::info!("starting unified HTTP and indexer service");

    let indexer_config = config.clone();
    let indexer_storage = storage.clone();
    let mut indexer = tokio::spawn(async move {
        run_startup_backfill(&indexer_config, &indexer_storage).await?;
        IngestService::new(indexer_config, indexer_storage)
            .run_live()
            .await
    });

    tokio::select! {
        http_result = http::serve_http(config, storage) => {
            indexer.abort();
            http_result
        }
        indexer_result = &mut indexer => {
            match indexer_result {
                Ok(result) => result,
                Err(error) if error.is_cancelled() => Ok(()),
                Err(error) => Err(error.into()),
            }
        }
    }
}

async fn run_startup_backfill(config: &AppConfig, storage: &Storage) -> anyhow::Result<()> {
    match (config.serve_backfill_from, config.serve_backfill_to) {
        (Some(from), Some(to)) => {
            tracing::info!(from_block = from, to_block = to, "running startup backfill");
            IngestService::new(config.clone(), storage.clone())
                .backfill_range(from, to)
                .await
        }
        (None, None) => Ok(()),
        _ => anyhow::bail!(
            "SERVE_BACKFILL_FROM and SERVE_BACKFILL_TO must both be set for startup backfill"
        ),
    }
}
