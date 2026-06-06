use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::AppConfig;
use ingest::IngestService;
use storage::Storage;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Debug, Parser)]
#[command(name = "cli", about = "Custom Rust ENS indexer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Serve,
    Migrate,
    Status,
    Reset {
        #[arg(long)]
        yes: bool,
    },
    Backfill {
        #[arg(long)]
        from: u64,
        #[arg(long)]
        to: u64,
    },
    Replay {
        #[arg(long)]
        from: u64,
        #[arg(long)]
        to: u64,
        #[arg(long)]
        archive_dir: Option<PathBuf>,
    },
    Index,
}

pub async fn run() -> anyhow::Result<()> {
    init_tracing();

    let cli = Cli::parse();
    let config = AppConfig::from_env()?;
    let storage = Storage::connect(&config.database_url).await?;

    match cli.command {
        Command::Serve => {
            storage.run_migrations().await?;
            server::serve(config, storage).await?;
        }
        Command::Migrate => {
            storage.run_migrations().await?;
            tracing::info!("migrations complete");
        }
        Command::Status => {
            print_status(&storage).await?;
        }
        Command::Reset { yes } => {
            if !yes {
                anyhow::bail!("reset deletes all indexed data; rerun with --yes to confirm");
            }

            storage.run_migrations().await?;
            storage.maintenance().reset_indexed_data().await?;
            tracing::warn!("indexed data reset complete");
        }
        Command::Backfill { from, to } => {
            storage.run_migrations().await?;
            IngestService::new(config, storage)
                .backfill_range(from, to)
                .await?;
        }
        Command::Replay {
            from,
            to,
            archive_dir,
        } => {
            storage.run_migrations().await?;
            IngestService::new(config, storage)
                .replay_archive_range(from, to, archive_dir)
                .await?;
        }
        Command::Index => {
            storage.run_migrations().await?;
            IngestService::new(config, storage).run_live().await?;
        }
    }

    Ok(())
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
