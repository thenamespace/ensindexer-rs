use std::{cmp, collections::BTreeSet, str::FromStr};

use alloy::{providers::ProviderBuilder, rpc::types::Log};
use alloy_primitives::Address;
use config::BackfillSource;

use super::IngestService;
use crate::{
    archive::{ArchivedRange, add_resolver_from_log, write_range},
    hypersync::{HypersyncBackfillClient, LogBatch},
    rpc::{
        fetch_block_meta_by_number, fetch_block_metadata, fetch_resolver_logs, fetch_source_logs,
    },
    sources::{LogSource, fixed_sources},
};

impl IngestService {
    pub async fn backfill_range(&self, from_block: u64, to_block: u64) -> anyhow::Result<()> {
        anyhow::ensure!(
            from_block <= to_block,
            "from block must be less than or equal to to block"
        );
        anyhow::ensure!(
            !self.config.backfill_source.is_raw(),
            "BACKFILL_SOURCE=raw is only valid for archive replay"
        );

        tracing::info!(
            chain_id = self.config.chain_id,
            from_block,
            to_block,
            batch_blocks = self.config.backfill_batch_blocks,
            backfill_source = ?self.config.backfill_source,
            "starting fixed-source backfill"
        );

        let provider = ProviderBuilder::new()
            .connect(self.config.eth_rpc_url.as_str())
            .await?;
        let hypersync = self.hypersync_backfill_client()?;
        let sources = fixed_sources()?;
        let mut range_start = from_block;

        while range_start <= to_block {
            let range_end = cmp::min(
                to_block,
                range_start.saturating_add(self.config.backfill_batch_blocks.saturating_sub(1)),
            );

            let mut batch = LogBatch::default();
            for source in &sources {
                let source_batch = match &hypersync {
                    Some(client) => {
                        client
                            .fetch_source_logs(source, range_start, range_end)
                            .await?
                    }
                    None => {
                        let logs =
                            fetch_source_logs(&provider, source, range_start, range_end).await?;
                        LogBatch {
                            raw_logs: logs
                                .into_iter()
                                .map(|log| (LogSource::Fixed(source.source), log))
                                .collect(),
                            block_meta: Default::default(),
                        }
                    }
                };
                tracing::debug!(
                    ?source,
                    from_block = range_start,
                    to_block = range_end,
                    logs = source_batch.raw_logs.len(),
                    "fetched logs"
                );

                batch.extend(source_batch);
            }

            let resolver_addresses = self
                .resolver_addresses_for_batch(&batch.raw_logs, range_start)
                .await?;
            let resolver_batch = match &hypersync {
                Some(client) => {
                    client
                        .fetch_resolver_logs(&resolver_addresses, range_start, range_end)
                        .await?
                }
                None => {
                    let logs =
                        fetch_resolver_logs(&provider, &resolver_addresses, range_start, range_end)
                            .await?;
                    LogBatch {
                        raw_logs: logs
                            .into_iter()
                            .map(|log| (LogSource::Resolver, log))
                            .collect(),
                        block_meta: Default::default(),
                    }
                }
            };
            tracing::debug!(
                from_block = range_start,
                to_block = range_end,
                addresses = resolver_addresses.len(),
                logs = resolver_batch.raw_logs.len(),
                "fetched resolver logs"
            );
            batch.extend(resolver_batch);

            let mut block_meta = batch.block_meta;
            if hypersync.is_none() {
                block_meta.extend(fetch_block_metadata(&provider, &batch.raw_logs).await?);
            }
            let active_sources = sources
                .iter()
                .filter(|source| range_end >= source.start_block)
                .collect::<Vec<_>>();
            let checkpoint_sources = active_sources
                .iter()
                .map(|source| source.checkpoint_name().to_owned())
                .collect::<Vec<_>>();
            if !active_sources.is_empty() && !block_meta.contains_key(&range_end) {
                let meta = match &hypersync {
                    Some(client) => client.fetch_block_meta_by_number(range_end).await?,
                    None => fetch_block_meta_by_number(&provider, range_end).await?,
                };
                block_meta.insert(range_end, meta);
            }

            if self.config.archive_backfills {
                let dir = self.config.raw_archive_dir.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("RAW_ARCHIVE_DIR is required when ARCHIVE_BACKFILLS=true")
                })?;
                let archive = ArchivedRange::new(
                    self.config.chain_id,
                    range_start,
                    range_end,
                    batch.raw_logs.clone(),
                    block_meta.clone(),
                    checkpoint_sources.clone(),
                );
                let path = write_range(dir, &archive)?;
                tracing::info!(path = %path.display(), "wrote raw archive range");
            }

            self.apply_raw_range_transactional(
                range_end,
                batch.raw_logs,
                block_meta,
                checkpoint_sources,
            )
            .await?;

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

    fn hypersync_backfill_client(&self) -> anyhow::Result<Option<HypersyncBackfillClient>> {
        if self.config.backfill_source != BackfillSource::Hypersync {
            return Ok(None);
        }

        let api_key = self
            .config
            .envio_api_key
            .as_deref()
            .filter(|key| !key.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("BACKFILL_SOURCE=hypersync requires ENVIO_API_KEY"))?;

        Ok(Some(HypersyncBackfillClient::new(
            self.config.hypersync_url.to_string(),
            api_key.to_owned(),
        )?))
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

        add_discovered_resolvers(&mut addresses, raw_logs)?;

        Ok(addresses.into_iter().collect())
    }
}

fn add_discovered_resolvers(
    addresses: &mut BTreeSet<Address>,
    raw_logs: &[(LogSource, Log)],
) -> anyhow::Result<()> {
    for (source, log) in raw_logs {
        add_resolver_from_log(addresses, *source, log)?;
    }
    Ok(())
}
