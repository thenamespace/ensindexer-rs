mod apply;
mod backfill;
mod live;
mod preload;
mod replay;

use alloy::providers::{Provider, ProviderBuilder};
use config::AppConfig;
use storage::Storage;

use crate::{
    archive::available_bounds,
    sources::{first_source_start_block, fixed_sources},
};

#[derive(Clone)]
pub struct IngestService {
    config: AppConfig,
    storage: Storage,
}

impl IngestService {
    pub fn new(config: AppConfig, storage: Storage) -> Self {
        Self { config, storage }
    }

    pub async fn run_configured_backfill(&self) -> anyhow::Result<()> {
        if self.config.backfill_source.is_raw() {
            self.replay_configured_archive(None).await
        } else {
            let (from_block, to_block) = self.resolve_db_backfill_range().await?;
            self.backfill_range(from_block, to_block).await
        }
    }

    pub(crate) async fn resolve_db_backfill_range(&self) -> anyhow::Result<(u64, u64)> {
        let from_block = self.next_checkpoint_block().await?;
        let to_block = self.latest_rpc_block().await?;
        anyhow::ensure!(
            from_block <= to_block,
            "database checkpoints are already at block {} but latest block is {}",
            from_block.saturating_sub(1),
            to_block
        );
        Ok((from_block, to_block))
    }

    pub(crate) async fn resolve_archive_replay_range(
        &self,
        archive_dir: &std::path::Path,
    ) -> anyhow::Result<(u64, u64)> {
        let (_, archived_to) = available_bounds(archive_dir, self.config.chain_id)?;
        let from_block = self.next_checkpoint_block().await?;
        anyhow::ensure!(
            from_block <= archived_to,
            "database checkpoints are already at block {} but raw archive covers through {}",
            from_block.saturating_sub(1),
            archived_to
        );
        Ok((from_block, archived_to))
    }

    async fn next_checkpoint_block(&self) -> anyhow::Result<u64> {
        let mut next_block = u64::MAX;
        for source in fixed_sources()? {
            let source_next = match self
                .storage
                .checkpoints()
                .find_by_source(source.checkpoint_name())
                .await?
            {
                Some(checkpoint) => checkpoint
                    .block_number
                    .try_into()
                    .map(|block: u64| block.saturating_add(1))
                    .map_err(|_| {
                        anyhow::anyhow!(
                            "checkpoint {} has negative block number {}",
                            checkpoint.source,
                            checkpoint.block_number
                        )
                    })?,
                None => source.start_block,
            };
            next_block = next_block.min(source_next);
        }
        if next_block == u64::MAX {
            first_source_start_block()
        } else {
            Ok(next_block)
        }
    }

    async fn latest_rpc_block(&self) -> anyhow::Result<u64> {
        let provider = ProviderBuilder::new()
            .connect(self.config.eth_rpc_url.as_str())
            .await?;
        Ok(provider.get_block_number().await?)
    }
}
