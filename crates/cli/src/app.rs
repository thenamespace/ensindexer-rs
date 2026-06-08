use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::AppConfig;
use ingest::IngestService;
use storage::Storage;
use tracing_subscriber::{EnvFilter, fmt};

use crate::{benchmark, compare, label_heal, schema};

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
    Backfill,
    Archive {
        #[arg(long)]
        archive_dir: Option<PathBuf>,
    },
    Replay {
        #[arg(long)]
        archive_dir: Option<PathBuf>,
    },
    ArchiveStatus {
        #[arg(long)]
        archive_dir: Option<PathBuf>,
        #[arg(long)]
        verify: bool,
    },
    LabelsHeal {
        #[arg(long)]
        labelhash: Vec<String>,
        #[arg(long, default_value_t = 1_000)]
        limit: i64,
        #[arg(long, default_value_t = 16)]
        repair_passes: usize,
    },
    LabelsImport {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value_t = 10_000)]
        chunk_size: usize,
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
        operation_name: Option<String>,
        #[arg(long)]
        show_json: bool,
    },
    Benchmark {
        #[arg(long, default_value = "benchmarks/queries")]
        query_dir: PathBuf,
        #[arg(long, default_value_t = 20)]
        iterations: usize,
        #[arg(long, default_value_t = 3)]
        warmup: usize,
        #[arg(long, default_value_t = true)]
        local_compute: bool,
        #[arg(long)]
        local_url: Option<String>,
        #[arg(long)]
        official_url: Option<String>,
        #[arg(long)]
        official_auth_token: Option<String>,
        #[arg(long)]
        ensnode_url: Option<String>,
        #[arg(long)]
        ensnode_auth_token: Option<String>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    SchemaLocal {
        #[arg(long)]
        output: Option<PathBuf>,
    },
    SchemaDiff {
        #[arg(long, env = "SUBGRAPH_URL")]
        subgraph_url: String,
        #[arg(long, env = "SUBGRAPH_AUTH_TOKEN")]
        auth_token: Option<String>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

pub async fn run() -> anyhow::Result<()> {
    init_tracing();
    dotenvy::dotenv().ok();

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
        Command::Backfill => {
            let config = AppConfig::from_env()?;
            let storage = replay_storage(&config).await?;
            storage.run_migrations().await?;
            IngestService::new(config, storage)
                .run_configured_backfill()
                .await?;
        }
        Command::Archive { archive_dir } => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            IngestService::new(config, storage)
                .run_configured_archive(archive_dir)
                .await?;
        }
        Command::Replay { archive_dir } => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect_with_max_connections(&config.database_url, 1).await?;
            storage.run_migrations().await?;
            IngestService::new(config, storage)
                .replay_configured_archive(archive_dir)
                .await?;
        }
        Command::ArchiveStatus {
            archive_dir,
            verify,
        } => {
            let config = AppConfig::from_env()?;
            let archive_dir = archive_dir
                .or(config.raw_archive_dir)
                .ok_or_else(|| anyhow::anyhow!("RAW_ARCHIVE_DIR or --archive-dir is required"))?;
            let status =
                ingest::inspect_archive(&archive_dir, config.chain_id, None, None, verify)?;
            print_archive_status(status);
        }
        Command::LabelsHeal {
            labelhash,
            limit,
            repair_passes,
        } => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            label_heal::run(
                storage,
                label_heal::HealOptions {
                    labelhashes: labelhash,
                    limit,
                    repair_passes,
                },
            )
            .await?;
        }
        Command::LabelsImport { input, chunk_size } => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            storage.run_migrations().await?;
            label_heal::import(storage, label_heal::ImportOptions { input, chunk_size }).await?;
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
            operation_name,
            show_json,
        } => {
            compare::run(
                local_url,
                subgraph_url,
                auth_token,
                query_file,
                variables_file,
                operation_name,
                show_json,
            )
            .await?;
        }
        Command::Benchmark {
            query_dir,
            iterations,
            warmup,
            local_compute,
            local_url,
            official_url,
            official_auth_token,
            ensnode_url,
            ensnode_auth_token,
            output,
        } => {
            let config = AppConfig::from_env()?;
            let storage = Storage::connect(&config.database_url).await?;
            benchmark::run(
                storage,
                benchmark::BenchmarkOptions {
                    query_dir,
                    iterations,
                    warmup,
                    local_compute,
                    local_url,
                    official_url,
                    official_auth_token,
                    ensnode_url,
                    ensnode_auth_token,
                    output,
                },
            )
            .await?;
        }
        Command::SchemaLocal { output } => {
            schema::print_local_sdl(output).await?;
        }
        Command::SchemaDiff {
            subgraph_url,
            auth_token,
            output,
        } => {
            schema::diff_official(subgraph_url, auth_token, output).await?;
        }
    }

    Ok(())
}

fn print_archive_status(status: ingest::ArchiveStatus) {
    println!("archive chain_id: {}", status.chain_id);
    println!("archive ranges: {}", status.ranges.len());
    if let (Some(first), Some(last)) = (status.ranges.first(), status.ranges.last()) {
        println!("archive coverage: {}..{}", first.from_block, last.to_block);
    }
    let total_bytes: u64 = status.ranges.iter().map(|range| range.bytes).sum();
    let total_logs: usize = status.ranges.iter().map(|range| range.logs).sum();
    println!("archive bytes: {total_bytes}");
    println!("archive logs: {total_logs}");
    println!("archive verified: {}", status.verified);

    if status.is_contiguous() {
        println!("archive gaps: none");
    } else {
        println!("archive gaps:");
        for gap in status.gaps {
            println!("  {}..{}", gap.from_block, gap.to_block);
        }
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

async fn replay_storage(config: &AppConfig) -> anyhow::Result<Storage> {
    if config.backfill_source.is_raw() {
        Ok(Storage::connect_with_max_connections(&config.database_url, 1).await?)
    } else {
        Ok(Storage::connect(&config.database_url).await?)
    }
}
