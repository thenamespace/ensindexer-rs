use std::{env, net::SocketAddr, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub eth_rpc_url: Url,
    pub envio_api_key: Option<String>,
    pub hypersync_url: Url,
    pub backfill_source: BackfillSource,
    pub raw_archive_dir: Option<PathBuf>,
    pub chain_id: u64,
    pub bind_address: SocketAddr,
    pub graphql_sandbox: bool,
    pub serve_indexer: bool,
    pub serve_backfill_from: Option<u64>,
    pub serve_backfill_to: Option<u64>,
    pub serve_backfill_source: BackfillSource,
    pub indexer_confirmation_depth: u64,
    pub backfill_batch_blocks: u64,
    pub live_poll_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let backfill_source = optional("BACKFILL_SOURCE", BackfillSource::Auto)?;
        let serve_backfill_source = optional("SERVE_BACKFILL_SOURCE", backfill_source)?;

        Ok(Self {
            database_url: required("DATABASE_URL")?,
            eth_rpc_url: required("ETH_RPC_URL")?.parse()?,
            envio_api_key: optional_secret_with_fallback("ENVIO_API_KEY", "ENVIO_API_TOKEN"),
            hypersync_url: optional(
                "HYPERSYNC_URL",
                "https://eth.hypersync.xyz"
                    .parse()
                    .expect("default hypersync url is valid"),
            )?,
            backfill_source,
            raw_archive_dir: optional_path_with_fallback("RAW_ARCHIVE_DIR", "RAW_LOG_ARCHIVE_DIR"),
            chain_id: optional("CHAIN_ID", 1)?,
            bind_address: optional(
                "BIND_ADDRESS",
                "127.0.0.1:8080"
                    .parse()
                    .expect("default bind address is valid"),
            )?,
            graphql_sandbox: optional_with_fallback("GRAPHQL_SANDBOX", "GRAPHQL_PLAYGROUND", true)?,
            serve_indexer: optional("SERVE_INDEXER", false)?,
            serve_backfill_from: optional_value("SERVE_BACKFILL_FROM")?,
            serve_backfill_to: optional_value("SERVE_BACKFILL_TO")?,
            serve_backfill_source,
            indexer_confirmation_depth: optional("INDEXER_CONFIRMATION_DEPTH", 12)?,
            backfill_batch_blocks: optional("BACKFILL_BATCH_BLOCKS", 1_000)?,
            live_poll_seconds: optional("LIVE_POLL_SECONDS", 12)?,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
pub enum BackfillSource {
    #[default]
    Auto,
    Hypersync,
    Rpc,
    Raw,
}

impl BackfillSource {
    pub fn use_hypersync(self, envio_api_key: Option<&str>) -> bool {
        match self {
            Self::Auto => envio_api_key.is_some_and(|key| !key.trim().is_empty()),
            Self::Hypersync => true,
            Self::Rpc | Self::Raw => false,
        }
    }

    pub fn is_raw(self) -> bool {
        matches!(self, Self::Raw)
    }
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

fn optional_value<T>(key: &str) -> Result<Option<T>, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key) {
        Ok(value) if value.trim().is_empty() => Ok(None),
        Ok(value) => value
            .parse()
            .map(Some)
            .map_err(|err| ConfigError::Invalid(format!("{key}: {err}"))),
        Err(_) => Ok(None),
    }
}

fn optional_with_fallback<T>(key: &str, fallback_key: &str, default: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key).or_else(|_| env::var(fallback_key)) {
        Ok(value) => value
            .parse()
            .map_err(|err| ConfigError::Invalid(format!("{key}: {err}"))),
        Err(_) => Ok(default),
    }
}

fn optional_secret_with_fallback(key: &str, fallback_key: &str) -> Option<String> {
    env::var(key)
        .or_else(|_| env::var(fallback_key))
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn optional_path_with_fallback(key: &str, fallback_key: &str) -> Option<PathBuf> {
    env::var(key)
        .or_else(|_| env::var(fallback_key))
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}

impl std::str::FromStr for BackfillSource {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "hypersync" | "envio" => Ok(Self::Hypersync),
            "rpc" => Ok(Self::Rpc),
            "raw" | "archive" | "json" => Ok(Self::Raw),
            _ => Err(ConfigError::Invalid(
                "BACKFILL_SOURCE: expected auto, hypersync, rpc, or raw".to_owned(),
            )),
        }
    }
}

impl From<url::ParseError> for ConfigError {
    fn from(value: url::ParseError) -> Self {
        Self::Invalid(format!("url parse error: {value}"))
    }
}
