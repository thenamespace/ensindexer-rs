use config::AppConfig;
use ingest::IngestService;
use storage::Storage;

use crate::http;

pub async fn serve(config: AppConfig, storage: Storage) -> anyhow::Result<()> {
    if !config.enable_backfill && !config.enable_live_indexing {
        http::serve_http(config, storage).await
    } else {
        serve_with_indexing(config, storage).await
    }
}

async fn serve_with_indexing(config: AppConfig, storage: Storage) -> anyhow::Result<()> {
    tracing::info!(
        enable_backfill = config.enable_backfill,
        enable_live_indexing = config.enable_live_indexing,
        "starting unified HTTP and optional indexer service"
    );

    let indexer_config = config.clone();
    let indexer_storage = storage.clone();
    let mut indexer = tokio::spawn(async move {
        run_enabled_backfill(&indexer_config, &indexer_storage).await?;
        run_enabled_live_indexing(indexer_config, indexer_storage).await
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

async fn run_enabled_backfill(config: &AppConfig, storage: &Storage) -> anyhow::Result<()> {
    if !config.enable_backfill {
        return Ok(());
    }

    tracing::info!(
        from_block = ?config.backfill_from,
        to_block = ?config.backfill_to,
        source = ?config.backfill_source,
        archive_backfills = config.archive_backfills,
        "running configured startup backfill"
    );

    IngestService::new(config.clone(), storage.clone())
        .run_configured_backfill(config.backfill_from, config.backfill_to)
        .await
}

async fn run_enabled_live_indexing(config: AppConfig, storage: Storage) -> anyhow::Result<()> {
    if config.enable_live_indexing {
        IngestService::new(config, storage).run_live().await
    } else {
        std::future::pending().await
    }
}
