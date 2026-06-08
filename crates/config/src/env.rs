use std::{env, net::SocketAddr, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub eth_rpc_url: Url,
    pub eth_ws_url: Option<Url>,
    pub envio_api_key: Option<String>,
    pub hypersync_url: Url,
    pub enable_backfill: bool,
    pub enable_live_indexing: bool,
    pub backfill_source: BackfillSource,
    pub indexing_source: IndexingSource,
    pub archive_backfills: bool,
    pub raw_archive_dir: Option<PathBuf>,
    pub chain_id: u64,
    pub bind_address: SocketAddr,
    pub indexer_confirmation_depth: u64,
    pub backfill_batch_blocks: u64,
    pub live_poll_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: required("DATABASE_URL")?,
            eth_rpc_url: required("ETH_RPC_URL")?.parse()?,
            eth_ws_url: optional_url("ETH_WS_URL")?,
            envio_api_key: optional_secret("ENVIO_API_KEY"),
            hypersync_url: optional(
                "HYPERSYNC_URL",
                "https://eth.hypersync.xyz"
                    .parse()
                    .expect("default hypersync url is valid"),
            )?,
            enable_backfill: optional("ENABLE_BACKFILL", false)?,
            enable_live_indexing: optional("ENABLE_LIVE_INDEXING", false)?,
            backfill_source: optional("BACKFILL_SOURCE", BackfillSource::Rpc)?,
            indexing_source: optional("INDEXING_SOURCE", IndexingSource::HttpRpc)?,
            archive_backfills: optional("ARCHIVE_BACKFILLS", false)?,
            raw_archive_dir: optional_path("RAW_ARCHIVE_DIR"),
            chain_id: optional("CHAIN_ID", 1)?,
            bind_address: optional(
                "BIND_ADDRESS",
                "127.0.0.1:8080"
                    .parse()
                    .expect("default bind address is valid"),
            )?,
            indexer_confirmation_depth: optional("INDEXER_CONFIRMATION_DEPTH", 12)?,
            backfill_batch_blocks: optional("BACKFILL_BATCH_BLOCKS", 1_000)?,
            live_poll_seconds: optional("LIVE_POLL_SECONDS", 12)?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
pub enum BackfillSource {
    #[default]
    Rpc,
    Hypersync,
    Raw,
}

impl BackfillSource {
    pub fn is_raw(self) -> bool {
        matches!(self, Self::Raw)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
pub enum IndexingSource {
    #[default]
    HttpRpc,
    Wss,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    Missing(String),
    #[error("invalid environment variable value: {0}")]
    Invalid(String),
}

fn required(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::Missing(key.to_owned()))
}

fn optional<T>(key: &str, default: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key) {
        Ok(value) => value
            .parse()
            .map_err(|err| ConfigError::Invalid(format!("{key}: {err}"))),
        Err(_) => Ok(default),
    }
}

fn optional_secret(key: &str) -> Option<String> {
    env::var(key).ok().filter(|value| !value.trim().is_empty())
}

fn optional_path(key: &str) -> Option<PathBuf> {
    optional_secret(key).map(PathBuf::from)
}

fn optional_url(key: &str) -> Result<Option<Url>, ConfigError> {
    optional_secret(key)
        .map(|value| value.parse().map_err(ConfigError::from))
        .transpose()
}

impl std::str::FromStr for BackfillSource {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "rpc" => Ok(Self::Rpc),
            "hypersync" => Ok(Self::Hypersync),
            "raw" => Ok(Self::Raw),
            _ => Err(ConfigError::Invalid(
                "BACKFILL_SOURCE: expected rpc, hypersync, or raw".to_owned(),
            )),
        }
    }
}

impl std::str::FromStr for IndexingSource {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "http_rpc" => Ok(Self::HttpRpc),
            "wss" => Ok(Self::Wss),
            _ => Err(ConfigError::Invalid(
                "INDEXING_SOURCE: expected http_rpc or wss".to_owned(),
            )),
        }
    }
}

impl From<url::ParseError> for ConfigError {
    fn from(value: url::ParseError) -> Self {
        Self::Invalid(format!("url parse error: {value}"))
    }
}
