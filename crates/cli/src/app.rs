use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::AppConfig;
use ingest::IngestService;
use storage::Storage;
use tracing_subscriber::{EnvFilter, fmt};

use crate::compare;

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
    Compare {
        #[arg(long, default_value = "http://127.0.0.1:8080/subgraph")]
        local_url: String,
        #[arg(long, env = "SUBGRAPH_URL")]
        subgraph_url: String,
        #[arg(long, env = "SUBGRAPH_AUTH_TOKEN")]
        auth_token: Option<String>,
        #[arg(long)]
        query_file: PathBuf,
        #[arg(long)]
        variables_file: Option<PathBuf>,
        #[arg(long)]
        show_json: bool,
    },
}

pub async fn run() -> anyhow::Result<()> {
    init_tracing();

    let cli = Cli::parse();
    match cli.command {
        Command::Serve => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            server::serve(config, storage).await?;
        }
        Command::Migrate => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            tracing::info!("migrations complete");
        }
        Command::Status => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            print_status(&storage).await?;
        }
        Command::Reset { yes } => {
            if !yes {
                anyhow::bail!("reset deletes all indexed data; rerun with --yes to confirm");
            }

            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            storage.maintenance().reset_indexed_data().await?;
            tracing::warn!("indexed data reset complete");
        }
        Command::Backfill { from, to } => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
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
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            IngestService::new(config, storage)
                .replay_archive_range(from, to, archive_dir)
                .await?;
        }
        Command::Index => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            IngestService::new(config, storage).run_live().await?;
        }
        Command::Compare {
            local_url,
            subgraph_url,
            auth_token,
            query_file,
            variables_file,
            show_json,
        } => {
            compare::run(
                local_url,
                subgraph_url,
                auth_token,
                query_file,
                variables_file,
                show_json,
            )
            .await?;
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
