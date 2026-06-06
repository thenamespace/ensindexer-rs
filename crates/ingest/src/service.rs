use std::{cmp, collections::BTreeSet, str::FromStr, time::Duration};

use alloy::{
    consensus::BlockHeader,
    network::{BlockResponse, Network},
    providers::{Provider, ProviderBuilder},
    rpc::types::Log,
};
use alloy_network_primitives::HeaderResponse;
use alloy_primitives::Address;
use config::AppConfig;
use contracts::{EnsEvent, decode_fixed_source_log};
use storage::{BlockInsert, Storage};
use tokio::time::sleep;

use crate::{
    decode::decode_log,
    rpc::{
        fetch_block_meta_by_number, fetch_block_metadata, fetch_resolver_logs, fetch_source_logs,
    },
    sources::{LogSource, fixed_sources},
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

    pub async fn backfill_range(&self, from_block: u64, to_block: u64) -> anyhow::Result<()> {
        anyhow::ensure!(
            from_block <= to_block,
            "from block must be less than or equal to to block"
        );

        tracing::info!(
            chain_id = self.config.chain_id,
            from_block,
            to_block,
            batch_blocks = self.config.backfill_batch_blocks,
            "starting fixed-source backfill"
        );

        let provider = ProviderBuilder::new()
            .connect(self.config.eth_rpc_url.as_str())
            .await?;
        let sources = fixed_sources()?;
        let mut range_start = from_block;

        while range_start <= to_block {
            let range_end = cmp::min(
                to_block,
                range_start.saturating_add(self.config.backfill_batch_blocks.saturating_sub(1)),
            );

            let mut raw_logs = Vec::new();
            for source in &sources {
                let logs = fetch_source_logs(&provider, source, range_start, range_end).await?;
                tracing::debug!(
                    ?source,
                    from_block = range_start,
                    to_block = range_end,
                    logs = logs.len(),
                    "fetched logs"
                );

                raw_logs.extend(
                    logs.into_iter()
                        .map(|log| (LogSource::Fixed(source.source), log)),
                );
            }

            let resolver_addresses = self
                .resolver_addresses_for_batch(&raw_logs, range_start)
                .await?;
            let resolver_logs =
                fetch_resolver_logs(&provider, &resolver_addresses, range_start, range_end).await?;
            tracing::debug!(
                from_block = range_start,
                to_block = range_end,
                addresses = resolver_addresses.len(),
                logs = resolver_logs.len(),
                "fetched resolver logs"
            );
            raw_logs.extend(
                resolver_logs
                    .into_iter()
                    .map(|log| (LogSource::Resolver, log)),
            );

            let mut block_meta = fetch_block_metadata(&provider, &raw_logs).await?;
            let active_sources = sources
                .iter()
                .filter(|source| range_end >= source.start_block)
                .collect::<Vec<_>>();
            if !active_sources.is_empty() && !block_meta.contains_key(&range_end) {
                block_meta.insert(
                    range_end,
                    fetch_block_meta_by_number(&provider, range_end).await?,
                );
            }

            for meta in block_meta.values() {
                self.storage
                    .blocks()
                    .upsert(BlockInsert {
                        number: meta.number_i64()?,
                        hash: types::hex_b256(meta.hash),
                        parent_hash: Some(types::hex_b256(meta.parent_hash)),
                        timestamp: meta.timestamp_i64()?,
                    })
                    .await?;
            }

            let mut decoded = Vec::new();
            for (source, log) in raw_logs {
                match decode_log(source, log, &block_meta) {
                    Ok(indexed) => decoded.push(indexed),
                    Err(error) => tracing::warn!(?source, %error, "skipping undecodable log"),
                }
            }

            decoded.sort_by_key(|event| {
                (
                    event.ctx.block_number,
                    event.ctx.transaction_index,
                    event.ctx.log_index,
                )
            });

            for event in decoded {
                projection::apply_event(&self.storage, event).await?;
            }

            for source in active_sources {
                if let Some(meta) = block_meta.get(&range_end) {
                    self.storage
                        .checkpoints()
                        .upsert(
                            source.checkpoint_name(),
                            meta.number_i64()?,
                            &types::hex_b256(meta.hash),
                        )
                        .await?;
                }
            }

            tracing::info!(
                from_block = range_start,
                to_block = range_end,
                "applied fixed-source backfill range"
            );

            if range_end == u64::MAX {
                break;
            }
            range_start = range_end + 1;
        }

        Ok(())
    }

    async fn resolver_addresses_for_batch(
        &self,
        raw_logs: &[(LogSource, Log)],
        range_start: u64,
    ) -> anyhow::Result<Vec<Address>> {
        let mut addresses = BTreeSet::new();

        if range_start
            > fixed_sources()?
                .first()
                .map_or(0, |source| source.start_block)
        {
            for address in self.storage.resolvers().list_distinct_addresses().await? {
                addresses.insert(Address::from_str(&address)?);
            }
        }

        for (source, log) in raw_logs {
            if !matches!(source, LogSource::Fixed(_)) {
                continue;
            }

            let Ok(EnsEvent::RegistryNewResolver { resolver, .. }) =
                decode_fixed_source_log(source.fixed_source()?, log)
            else {
                continue;
            };

            if resolver != Address::ZERO {
                addresses.insert(resolver);
            }
        }

        Ok(addresses.into_iter().collect())
    }

    pub async fn run_live(&self) -> anyhow::Result<()> {
        tracing::info!(
            chain_id = self.config.chain_id,
            confirmation_depth = self.config.indexer_confirmation_depth,
            poll_seconds = self.config.live_poll_seconds,
            "live indexer requested"
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
            self.backfill_range(next_block, range_end).await?;
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
