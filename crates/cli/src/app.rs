use alloy::providers::{Provider, ProviderBuilder};
use clap::{Parser, Subcommand};
use config::{AppConfig, BackfillSource};
use storage::Storage;
use tracing_subscriber::{EnvFilter, fmt};

const STARTUP_MAINTENANCE_LAG_THRESHOLD: u64 = 500_000;

#[derive(Debug, Parser)]
#[command(name = "ensindexer", about = "Production Rust ENS indexer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start HTTP, GraphQL, Playground, and optional indexing workers.
    Start,
    /// Print latest indexed block and source checkpoints.
    Status,
}

pub async fn run() -> anyhow::Result<()> {
    init_tracing();
    dotenvy::dotenv().ok();

    match Cli::parse().command {
        Command::Start => start().await,
        Command::Status => status().await,
    }
}

async fn start() -> anyhow::Result<()> {
    let config = AppConfig::from_env()?;
    validate_start_config(&config)?;

    let storage = if config.backfill_source.is_raw() {
        Storage::connect_with_max_connections(&config.database_url, 1).await?
    } else {
        Storage::connect(&config.database_url).await?
    };
    run_startup_maintenance(&config, &storage).await?;
    server::serve(config, storage).await
}

async fn status() -> anyhow::Result<()> {
    let config = AppConfig::from_env()?;
    let storage = Storage::connect(&config.database_url).await?;
    print_status(&storage).await
}

fn validate_start_config(config: &AppConfig) -> anyhow::Result<()> {
    if config.enable_backfill && config.backfill_source.is_raw() {
        anyhow::ensure!(
            config.raw_archive_dir.is_some(),
            "BACKFILL_SOURCE=raw requires RAW_ARCHIVE_DIR"
        );
    }
    if config.archive_backfills {
        anyhow::ensure!(
            !config.backfill_source.is_raw(),
            "ARCHIVE_BACKFILLS=true is only valid with BACKFILL_SOURCE=rpc or hypersync"
        );
        anyhow::ensure!(
            config.raw_archive_dir.is_some(),
            "ARCHIVE_BACKFILLS=true requires RAW_ARCHIVE_DIR"
        );
    }
    if config.enable_backfill && config.backfill_source == BackfillSource::Hypersync {
        anyhow::ensure!(
            config
                .envio_api_key
                .as_deref()
                .is_some_and(|key| !key.trim().is_empty()),
            "BACKFILL_SOURCE=hypersync requires ENVIO_API_KEY"
        );
    }
    Ok(())
}

async fn run_startup_maintenance(config: &AppConfig, storage: &Storage) -> anyhow::Result<()> {
    if should_skip_startup_maintenance(config, storage).await? {
        return Ok(());
    }

    storage.run_migrations().await?;
    storage.maintenance().ensure_bulk_replay_indexes().await?;
    Ok(())
}

async fn should_skip_startup_maintenance(
    config: &AppConfig,
    storage: &Storage,
) -> anyhow::Result<bool> {
    if !config.enable_backfill {
        return Ok(false);
    }

    if !storage.schema_is_initialized().await? {
        tracing::info!("database schema is not initialized; running startup migrations");
        return Ok(false);
    }

    let Some(next_block) = next_checkpoint_block(storage).await? else {
        tracing::info!(
            threshold = STARTUP_MAINTENANCE_LAG_THRESHOLD,
            "skipping startup migration/index repair before initial backfill checkpoints exist"
        );
        return Ok(true);
    };

    let latest_block = latest_rpc_block(config).await?;
    let target_block = backfill_target_block(config, latest_block);
    let lag_blocks = target_block.saturating_sub(next_block);

    if lag_blocks > STARTUP_MAINTENANCE_LAG_THRESHOLD {
        tracing::info!(
            next_block,
            target_block,
            lag_blocks,
            threshold = STARTUP_MAINTENANCE_LAG_THRESHOLD,
            "skipping startup migration/index repair while backfill is far behind"
        );
        return Ok(true);
    }

    tracing::info!(
        next_block,
        target_block,
        lag_blocks,
        threshold = STARTUP_MAINTENANCE_LAG_THRESHOLD,
        "running startup migration/index repair because backfill is near caught up"
    );
    Ok(false)
}

async fn next_checkpoint_block(storage: &Storage) -> anyhow::Result<Option<u64>> {
    let checkpoints = storage.checkpoints().list().await?;
    let next_block = checkpoints
        .into_iter()
        .map(|checkpoint| {
            checkpoint
                .block_number
                .try_into()
                .map(|block: u64| block.saturating_add(1))
                .map_err(|_| {
                    anyhow::anyhow!(
                        "checkpoint {} has negative block number {}",
                        checkpoint.source,
                        checkpoint.block_number
                    )
                })
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .min();
    Ok(next_block)
}

async fn latest_rpc_block(config: &AppConfig) -> anyhow::Result<u64> {
    let provider = ProviderBuilder::new()
        .connect(config.eth_rpc_url.as_str())
        .await?;
    Ok(provider.get_block_number().await?)
}

fn backfill_target_block(config: &AppConfig, latest_block: u64) -> u64 {
    if config.enable_live_indexing {
        latest_block.saturating_sub(
            config
                .indexer_confirmation_depth
                .saturating_add(config.backfill_live_gap_blocks),
        )
    } else {
        latest_block
    }
}

async fn print_status(storage: &Storage) -> anyhow::Result<()> {
    match storage.blocks().find_latest().await? {
        Some(block) => {
            let parent_hash = block.parent_hash.as_deref().unwrap_or("none");
            println!(
                "latest block: number={} hash={} parent_hash={} timestamp={}",
                block.number, block.hash, parent_hash, block.timestamp
            );
        }
        None => println!("latest block: none"),
    }

    let checkpoints = storage.checkpoints().list().await?;
    if checkpoints.is_empty() {
        println!("checkpoints: none");
    } else {
        println!("checkpoints:");
        for checkpoint in checkpoints {
            println!(
                "  {}: number={} hash={}",
                checkpoint.source, checkpoint.block_number, checkpoint.block_hash
            );
        }
    }

    Ok(())
}

fn init_tracing() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=error"));
    fmt().with_env_filter(filter).init();
}
