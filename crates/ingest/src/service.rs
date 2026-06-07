mod apply;
mod backfill;
mod live;
mod replay;

use alloy::providers::{Provider, ProviderBuilder};
use config::AppConfig;
use storage::Storage;

use crate::{archive::available_bounds, sources::first_source_start_block};

#[derive(Clone)]
pub struct IngestService {
    config: AppConfig,
    storage: Storage,
}

impl IngestService {
    pub fn new(config: AppConfig, storage: Storage) -> Self {
        Self { config, storage }
    }

    pub async fn run_configured_backfill(
        &self,
        from_block: Option<u64>,
        to_block: Option<u64>,
    ) -> anyhow::Result<()> {
        let (from_block, to_block) = self.resolve_backfill_range(from_block, to_block).await?;

        if self.config.backfill_source.is_raw() {
            self.replay_archive_range(from_block, to_block, None).await
        } else {
            self.backfill_range(from_block, to_block).await
        }
    }

    pub async fn run_configured_archive(
        &self,
        from_block: Option<u64>,
        to_block: Option<u64>,
        archive_dir: Option<std::path::PathBuf>,
    ) -> anyhow::Result<()> {
        let (from_block, to_block) = self.resolve_backfill_range(from_block, to_block).await?;
        self.archive_range(from_block, to_block, archive_dir).await
    }

    async fn resolve_backfill_range(
        &self,
        from_block: Option<u64>,
        to_block: Option<u64>,
    ) -> anyhow::Result<(u64, u64)> {
        let from_block = match from_block {
            Some(from_block) => from_block,
            None if self.config.backfill_source.is_raw() => self.raw_archive_bounds()?.0,
            None => first_source_start_block()?,
        };
        let to_block = match to_block {
            Some(to_block) => to_block,
            None if self.config.backfill_source.is_raw() => self.raw_archive_bounds()?.1,
            None => self.latest_rpc_block().await?,
        };
        anyhow::ensure!(
            from_block <= to_block,
            "from block must be less than or equal to to block"
        );
        Ok((from_block, to_block))
    }

    fn raw_archive_bounds(&self) -> anyhow::Result<(u64, u64)> {
        let archive_dir = self.config.raw_archive_dir.as_ref().ok_or_else(|| {
            anyhow::anyhow!("RAW_ARCHIVE_DIR is required when BACKFILL_SOURCE=raw")
        })?;
        available_bounds(archive_dir, self.config.chain_id)
    }

    async fn latest_rpc_block(&self) -> anyhow::Result<u64> {
        let provider = ProviderBuilder::new()
            .connect(self.config.eth_rpc_url.as_str())
            .await?;
        Ok(provider.get_block_number().await?)
    }
}
