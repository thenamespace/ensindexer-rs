use std::{collections::BTreeMap, net::SocketAddr, path::PathBuf};

use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub postgres_db: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub eth_rpc_url: Url,
    pub envio_api_key: Option<String>,
    pub hypersync_url: Url,
    pub enable_backfill: bool,
    pub enable_live_indexing: bool,
    pub backfill_source: BackfillSource,
    pub archive_backfills: bool,
    pub raw_archive_dir: Option<PathBuf>,
    pub chain_id: u64,
    pub bind_address: SocketAddr,
    pub indexer_confirmation_depth: u64,
    pub backfill_live_gap_blocks: u64,
    pub backfill_batch_blocks: u64,
    pub live_poll_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let env = EnvConfig::load()?;
        let postgres_db = env.required("POSTGRES_DB")?;
        let postgres_user = env.required("POSTGRES_USER")?;
        let postgres_password = env.required("POSTGRES_PASSWORD")?;
        let postgres_host = env.required("POSTGRES_HOST")?;
        let postgres_port = env.optional("POSTGRES_PORT", 5432)?;

        Ok(Self {
            database_url: build_postgres_url(
                &postgres_db,
                &postgres_user,
                &postgres_password,
                &postgres_host,
                postgres_port,
            )?,
            postgres_db,
            postgres_user,
            postgres_password,
            postgres_host,
            postgres_port,
            eth_rpc_url: env.required("ETH_RPC_URL")?.parse()?,
            envio_api_key: env.optional_secret("ENVIO_API_KEY"),
            hypersync_url: env.optional(
                "HYPERSYNC_URL",
                "https://eth.hypersync.xyz"
                    .parse()
                    .expect("default hypersync url is valid"),
            )?,
            enable_backfill: env.optional("ENABLE_BACKFILL", false)?,
            enable_live_indexing: env.optional("ENABLE_LIVE_INDEXING", false)?,
            backfill_source: env.optional("BACKFILL_SOURCE", BackfillSource::Rpc)?,
            archive_backfills: env.optional("ARCHIVE_BACKFILLS", false)?,
            raw_archive_dir: env.optional_path("RAW_ARCHIVE_DIR"),
            chain_id: env.optional("CHAIN_ID", 1)?,
            bind_address: env.optional(
                "BIND_ADDRESS",
                "127.0.0.1:8080"
                    .parse()
                    .expect("default bind address is valid"),
            )?,
            indexer_confirmation_depth: env.optional("INDEXER_CONFIRMATION_DEPTH", 12)?,
            backfill_live_gap_blocks: env.optional("BACKFILL_LIVE_GAP_BLOCKS", 10)?,
            backfill_batch_blocks: env.optional("BACKFILL_BATCH_BLOCKS", 1_000)?,
            live_poll_seconds: env.optional("LIVE_POLL_SECONDS", 12)?,
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

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    Missing(String),
    #[error("invalid environment variable value: {0}")]
    Invalid(String),
    #[error("failed to load environment configuration: {0}")]
    Figment(#[from] figment::Error),
}

struct EnvConfig {
    values: BTreeMap<String, serde_json::Value>,
}

impl EnvConfig {
    fn load() -> Result<Self, ConfigError> {
        let values = Figment::new().merge(Env::raw()).extract()?;
        Ok(Self { values })
    }

    fn required(&self, key: &str) -> Result<String, ConfigError> {
        self.optional_secret(key)
            .ok_or_else(|| ConfigError::Missing(key.to_owned()))
    }

    fn optional<T>(&self, key: &str, default: T) -> Result<T, ConfigError>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        match self.optional_secret(key) {
            Some(value) => value
                .parse()
                .map_err(|err| ConfigError::Invalid(format!("{key}: {err}"))),
            None => Ok(default),
        }
    }

    fn optional_secret(&self, key: &str) -> Option<String> {
        self.values
            .get(key)
            .or_else(|| self.values.get(&key.to_ascii_lowercase()))
            .or_else(|| self.values.get(&key.to_ascii_uppercase()))
            .and_then(env_value_to_string)
            .filter(|value| !value.is_empty())
    }

    fn optional_path(&self, key: &str) -> Option<PathBuf> {
        self.optional_secret(key).map(PathBuf::from)
    }
}

fn env_value_to_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::String(value) => Some(value.trim().to_owned()),
        serde_json::Value::Bool(value) => Some(value.to_string()),
        serde_json::Value::Number(value) => Some(value.to_string()),
        other => Some(other.to_string()),
    }
}

fn build_postgres_url(
    db: &str,
    user: &str,
    password: &str,
    host: &str,
    port: u16,
) -> Result<String, ConfigError> {
    let mut url =
        Url::parse("postgres://localhost/postgres").expect("static postgres url is valid");
    url.set_username(user).map_err(|_| {
        ConfigError::Invalid("POSTGRES_USER cannot be encoded as URL username".into())
    })?;
    url.set_password(Some(password)).map_err(|_| {
        ConfigError::Invalid("POSTGRES_PASSWORD cannot be encoded as URL password".into())
    })?;
    url.set_host(Some(host))
        .map_err(|err| ConfigError::Invalid(format!("POSTGRES_HOST: {err}")))?;
    url.set_port(Some(port))
        .map_err(|_| ConfigError::Invalid("POSTGRES_PORT is invalid".into()))?;
    url.set_path(db);
    Ok(url.to_string())
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

impl From<url::ParseError> for ConfigError {
    fn from(value: url::ParseError) -> Self {
        Self::Invalid(format!("url parse error: {value}"))
    }
}
