use std::{net::SocketAddr, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use config::{AppConfig, BackfillSource, IndexingSource};
use storage::Storage;
use tracing_subscriber::{EnvFilter, fmt};
use url::Url;

#[derive(Debug, Parser)]
#[command(name = "ensindexer", about = "Production Rust ENS indexer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start HTTP, GraphQL, Playground, and optional indexing workers.
    Start(Box<StartArgs>),
    /// Print latest indexed block and source checkpoints.
    Status,
}

#[derive(Debug, Args, Default)]
struct StartArgs {
    #[arg(long)]
    database_url: Option<String>,
    #[arg(long)]
    eth_rpc_url: Option<Url>,
    #[arg(long)]
    eth_ws_url: Option<Url>,
    #[arg(long)]
    envio_api_key: Option<String>,
    #[arg(long)]
    hypersync_url: Option<Url>,
    #[arg(long)]
    raw_archive_dir: Option<PathBuf>,
    #[arg(long)]
    chain_id: Option<u64>,
    #[arg(long)]
    bind_address: Option<SocketAddr>,

    #[arg(long)]
    enable_backfill: Option<bool>,
    #[arg(long)]
    backfill_source: Option<BackfillSource>,
    #[arg(long)]
    enable_live_indexing: Option<bool>,
    #[arg(long)]
    live_indexing_source: Option<IndexingSource>,
    #[arg(long)]
    archive_backfills: Option<bool>,

    #[arg(long)]
    indexer_confirmation_depth: Option<u64>,
    #[arg(long)]
    backfill_batch_blocks: Option<u64>,
    #[arg(long)]
    live_poll_seconds: Option<u64>,
}

pub async fn run() -> anyhow::Result<()> {
    init_tracing();
    dotenvy::dotenv().ok();

    match Cli::parse().command {
        Command::Start(args) => start(*args).await,
        Command::Status => status().await,
    }
}

async fn start(args: StartArgs) -> anyhow::Result<()> {
    let mut config = AppConfig::from_env()?;
    args.apply(&mut config);
    validate_start_config(&config)?;

    let storage = if config.backfill_source.is_raw() {
        Storage::connect_with_max_connections(&config.database_url, 1).await?
    } else {
        Storage::connect(&config.database_url).await?
    };
    storage.run_migrations().await?;
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
    if config.enable_live_indexing && config.live_indexing_source == IndexingSource::Wss {
        anyhow::ensure!(
            config.eth_ws_url.is_some(),
            "LIVE_INDEXING_SOURCE=wss requires ETH_WS_URL"
        );
    }
    if config.backfill_source == BackfillSource::Hypersync {
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

impl StartArgs {
    fn apply(self, config: &mut AppConfig) {
        override_if_some(&mut config.database_url, self.database_url);
        override_if_some(&mut config.eth_rpc_url, self.eth_rpc_url);
        set_option_if_some(&mut config.eth_ws_url, self.eth_ws_url);
        set_option_if_some(&mut config.envio_api_key, self.envio_api_key);
        override_if_some(&mut config.hypersync_url, self.hypersync_url);
        set_option_if_some(&mut config.raw_archive_dir, self.raw_archive_dir);
        override_if_some(&mut config.chain_id, self.chain_id);
        override_if_some(&mut config.bind_address, self.bind_address);
        override_if_some(&mut config.enable_backfill, self.enable_backfill);
        override_if_some(&mut config.backfill_source, self.backfill_source);
        override_if_some(&mut config.enable_live_indexing, self.enable_live_indexing);
        override_if_some(&mut config.live_indexing_source, self.live_indexing_source);
        override_if_some(&mut config.archive_backfills, self.archive_backfills);
        override_if_some(
            &mut config.indexer_confirmation_depth,
            self.indexer_confirmation_depth,
        );
        override_if_some(
            &mut config.backfill_batch_blocks,
            self.backfill_batch_blocks,
        );
        override_if_some(&mut config.live_poll_seconds, self.live_poll_seconds);
    }
}

fn override_if_some<T>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

fn set_option_if_some<T>(target: &mut Option<T>, value: Option<T>) {
    if let Some(value) = value {
        *target = Some(value);
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
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();
}
