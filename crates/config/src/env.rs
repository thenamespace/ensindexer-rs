use std::{env, net::SocketAddr};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub eth_rpc_url: Url,
    pub chain_id: u64,
    pub bind_address: SocketAddr,
    pub graphql_sandbox: bool,
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
            chain_id: optional("CHAIN_ID", 1)?,
            bind_address: optional(
                "BIND_ADDRESS",
                "127.0.0.1:8080"
                    .parse()
                    .expect("default bind address is valid"),
            )?,
            graphql_sandbox: optional_with_fallback("GRAPHQL_SANDBOX", "GRAPHQL_PLAYGROUND", true)?,
            indexer_confirmation_depth: optional("INDEXER_CONFIRMATION_DEPTH", 12)?,
            backfill_batch_blocks: optional("BACKFILL_BATCH_BLOCKS", 1_000)?,
            live_poll_seconds: optional("LIVE_POLL_SECONDS", 12)?,
        })
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

impl From<url::ParseError> for ConfigError {
    fn from(value: url::ParseError) -> Self {
        Self::Invalid(format!("ETH_RPC_URL: {value}"))
    }
}
