use std::{cmp, time::Duration};

use alloy::{
    consensus::BlockHeader,
    network::{BlockResponse, Network},
    providers::{Provider, ProviderBuilder},
};
use alloy_network_primitives::HeaderResponse;
use tokio::time::sleep;

use super::IngestService;
use crate::{rpc::fetch_block_meta_by_number, sources::fixed_sources};

impl IngestService {
    pub async fn run_live(&self) -> anyhow::Result<()> {
        tracing::info!(
            chain_id = self.config.chain_id,
            confirmation_depth = self.config.indexer_confirmation_depth,
            poll_seconds = self.config.live_poll_seconds,
            "live indexer requested with HTTP RPC polling"
        );

        let provider = ProviderBuilder::new()
            .connect(self.config.eth_rpc_url.as_str())
            .await?;

        loop {
            let latest = provider.get_block_number().await?;
            let safe_head = latest.saturating_sub(self.config.indexer_confirmation_depth);
            let next_block = self.next_live_block().await?;

            if next_block > safe_head {
                tracing::debug!(
                    latest,
                    safe_head,
                    next_block,
                    "live indexer is waiting for confirmed blocks"
                );
                sleep(Duration::from_secs(self.config.live_poll_seconds)).await;
                continue;
            }

            if !self.verify_parent_hash(&provider, next_block).await? {
                tracing::warn!(
                    next_block,
                    "reorg detected; clearing indexed state for canonical rebuild"
                );
                self.storage.maintenance().reset_indexed_data().await?;
                continue;
            }

            let range_end = cmp::min(
                safe_head,
                next_block.saturating_add(self.config.backfill_batch_blocks.saturating_sub(1)),
            );

            tracing::info!(
                latest,
                safe_head,
                from_block = next_block,
                to_block = range_end,
                "indexing live confirmed range"
            );
            self.live_range(next_block, range_end).await?;
        }
    }

    async fn next_live_block(&self) -> anyhow::Result<u64> {
        let sources = fixed_sources()?;
        let mut next_block = u64::MAX;

        for source in sources {
            let next_for_source = match self
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

            next_block = cmp::min(next_block, next_for_source);
        }

        anyhow::ensure!(next_block != u64::MAX, "no ingest sources configured");
        Ok(next_block)
    }

    async fn verify_parent_hash<P, N>(&self, provider: &P, next_block: u64) -> anyhow::Result<bool>
    where
        P: Provider<N>,
        N: Network,
        N::BlockResponse: BlockResponse,
        <N::BlockResponse as BlockResponse>::Header: BlockHeader + HeaderResponse,
    {
        if next_block == 0 {
            return Ok(true);
        }

        let previous_number: i64 = next_block
            .saturating_sub(1)
            .try_into()
            .map_err(|_| anyhow::anyhow!("block number does not fit i64: {}", next_block - 1))?;
        let Some(previous) = self
            .storage
            .blocks()
            .find_by_number(previous_number)
            .await?
        else {
            return Ok(true);
        };

        let next_meta = fetch_block_meta_by_number(provider, next_block).await?;
        let expected_parent = types::hex_b256(next_meta.parent_hash);
        if previous.hash != expected_parent {
            tracing::warn!(
                next_block,
                previous_number = previous.number,
                local_hash = previous.hash,
                remote_parent_hash = expected_parent,
                "parent hash mismatch"
            );
            return Ok(false);
        }

        Ok(true)
    }
}
