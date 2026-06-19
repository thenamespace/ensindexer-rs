mod apply;
mod backfill;
mod live;
mod preload;
mod replay;

use alloy::providers::{Provider, ProviderBuilder};
use config::AppConfig;
use storage::Storage;

use crate::{
    archive::{available_bounds, available_end_at_or_before},
    sources::{first_source_start_block, fixed_sources},
};

pub(crate) const BULK_INDEX_DROP_BLOCK_THRESHOLD: u64 = 500_000;

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
            match self.resolve_db_backfill_range().await? {
                Some((from_block, to_block)) => {
                    if self.config.backfill_source == config::BackfillSource::Hypersync {
                        self.backfill_range_with_bulk_indexes(from_block, to_block)
                            .await
                    } else {
                        self.backfill_range(from_block, to_block).await
                    }
                }
                None => Ok(()),
            }
        }
    }

    pub(crate) async fn resolve_db_backfill_range(&self) -> anyhow::Result<Option<(u64, u64)>> {
        let from_block = self.next_checkpoint_block().await?;
        let to_block = self.backfill_target_block(self.latest_rpc_block().await?);
        if from_block > to_block {
            tracing::info!(
                from_block,
                to_block,
                enable_live_indexing = self.config.enable_live_indexing,
                confirmation_depth = self.config.indexer_confirmation_depth,
                backfill_live_gap_blocks = self.config.backfill_live_gap_blocks,
                "startup backfill has no confirmed historical range to process"
            );
            return Ok(None);
        }
        Ok(Some((from_block, to_block)))
    }

    pub(crate) async fn resolve_archive_replay_range(
        &self,
        archive_dir: &std::path::Path,
    ) -> anyhow::Result<Option<(u64, u64)>> {
        let (_, archived_to) = available_bounds(archive_dir, self.config.chain_id)?;
        let target = self.backfill_target_block(archived_to);
        let Some(to_block) = available_end_at_or_before(archive_dir, self.config.chain_id, target)?
        else {
            tracing::info!(
                archived_to,
                target,
                "raw archive has no complete range at or before configured backfill target"
            );
            return Ok(None);
        };
        let from_block = self.next_checkpoint_block().await?;
        if from_block > to_block {
            tracing::info!(
                from_block,
                to_block,
                archived_to,
                "raw archive replay has no range to process"
            );
            return Ok(None);
        }
        Ok(Some((from_block, to_block)))
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

    fn backfill_target_block(&self, latest_block: u64) -> u64 {
        if self.config.enable_live_indexing {
            latest_block.saturating_sub(
                self.config
                    .indexer_confirmation_depth
                    .saturating_add(self.config.backfill_live_gap_blocks),
            )
        } else {
            latest_block
        }
    }

    async fn backfill_range_with_bulk_indexes(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> anyhow::Result<()> {
        if !should_drop_bulk_indexes(from_block, to_block) {
            tracing::info!(
                from_block,
                to_block,
                block_span = backfill_block_span(from_block, to_block),
                threshold = BULK_INDEX_DROP_BLOCK_THRESHOLD,
                "keeping secondary indexes for small hypersync backfill"
            );
            return self.backfill_range(from_block, to_block).await;
        }

        tracing::info!("dropping secondary indexes for bulk hypersync backfill");
        self.storage
            .maintenance()
            .drop_bulk_replay_indexes()
            .await?;

        let result = self.backfill_range(from_block, to_block).await;

        tracing::info!("recreating secondary indexes after bulk hypersync backfill");
        let recreate_result = self
            .storage
            .maintenance()
            .recreate_bulk_replay_indexes()
            .await;
        match (result, recreate_result) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(error), Ok(())) => Err(error),
            (Ok(()), Err(error)) => Err(error.into()),
            (Err(error), Err(recreate_error)) => {
                tracing::error!(%recreate_error, "failed to recreate secondary indexes");
                Err(error)
            }
        }
    }
}

pub(crate) fn should_drop_bulk_indexes(from_block: u64, to_block: u64) -> bool {
    backfill_block_span(from_block, to_block) > BULK_INDEX_DROP_BLOCK_THRESHOLD
}

pub(crate) fn backfill_block_span(from_block: u64, to_block: u64) -> u64 {
    to_block.saturating_sub(from_block).saturating_add(1)
}

#[cfg(test)]
mod tests {
    use super::{BULK_INDEX_DROP_BLOCK_THRESHOLD, should_drop_bulk_indexes};

    #[test]
    fn bulk_index_drop_requires_more_than_threshold_blocks() {
        assert!(!should_drop_bulk_indexes(
            1,
            BULK_INDEX_DROP_BLOCK_THRESHOLD
        ));
        assert!(should_drop_bulk_indexes(
            1,
            BULK_INDEX_DROP_BLOCK_THRESHOLD + 1
        ));
    }
}
