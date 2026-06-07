use std::{collections::BTreeMap, time::Instant};

use alloy::rpc::types::Log;
use storage::BlockInsert;

use super::IngestService;
use crate::{decode::decode_log, rpc::BlockMeta, sources::LogSource};

impl IngestService {
    pub(super) async fn apply_raw_range_transactional(
        &self,
        range_end: u64,
        raw_logs: Vec<(LogSource, Log)>,
        block_meta: BTreeMap<u64, BlockMeta>,
        checkpoint_sources: Vec<String>,
    ) -> anyhow::Result<()> {
        sqlx::query("begin").execute(self.storage.pool()).await?;
        sqlx::query("set local synchronous_commit = off")
            .execute(self.storage.pool())
            .await?;
        if let Err(error) = self.storage.begin_change_buffer() {
            if let Err(rollback_error) = sqlx::query("rollback").execute(self.storage.pool()).await
            {
                tracing::error!(%rollback_error, "failed to roll back raw replay range");
            }
            return Err(error.into());
        }
        if let Err(error) = self.storage.begin_event_buffer() {
            if let Err(clear_error) = self.storage.clear_change_buffer() {
                tracing::error!(%clear_error, "failed to clear replay change buffer");
            }
            if let Err(rollback_error) = sqlx::query("rollback").execute(self.storage.pool()).await
            {
                tracing::error!(%rollback_error, "failed to roll back raw replay range");
            }
            return Err(error.into());
        }
        if let Err(error) = self.storage.begin_entity_cache() {
            if let Err(clear_error) = self.storage.clear_change_buffer() {
                tracing::error!(%clear_error, "failed to clear replay change buffer");
            }
            if let Err(clear_error) = self.storage.clear_event_buffer() {
                tracing::error!(%clear_error, "failed to clear replay event buffer");
            }
            if let Err(rollback_error) = sqlx::query("rollback").execute(self.storage.pool()).await
            {
                tracing::error!(%rollback_error, "failed to roll back raw replay range");
            }
            return Err(error.into());
        }

        let result = self
            .apply_raw_range_buffered(range_end, raw_logs, block_meta, checkpoint_sources)
            .await;

        match result {
            Ok(()) => {
                self.storage.clear_change_buffer()?;
                self.storage.clear_event_buffer()?;
                self.storage.clear_entity_cache()?;
                let started = Instant::now();
                sqlx::query("commit").execute(self.storage.pool()).await?;
                tracing::debug!(
                    elapsed_ms = started.elapsed().as_millis(),
                    "committed raw replay transaction"
                );
                Ok(())
            }
            Err(error) => {
                if let Err(clear_error) = self.storage.clear_change_buffer() {
                    tracing::error!(%clear_error, "failed to clear replay change buffer");
                }
                if let Err(clear_error) = self.storage.clear_event_buffer() {
                    tracing::error!(%clear_error, "failed to clear replay event buffer");
                }
                if let Err(clear_error) = self.storage.clear_entity_cache() {
                    tracing::error!(%clear_error, "failed to clear replay entity cache");
                }
                if let Err(rollback_error) =
                    sqlx::query("rollback").execute(self.storage.pool()).await
                {
                    tracing::error!(%rollback_error, "failed to roll back raw replay range");
                }
                Err(error)
            }
        }
    }

    async fn apply_raw_range_buffered(
        &self,
        range_end: u64,
        raw_logs: Vec<(LogSource, Log)>,
        block_meta: BTreeMap<u64, BlockMeta>,
        checkpoint_sources: Vec<String>,
    ) -> anyhow::Result<()> {
        self.apply_raw_range_inner(range_end, raw_logs, block_meta, checkpoint_sources, true)
            .await
    }

    async fn apply_raw_range_inner(
        &self,
        range_end: u64,
        raw_logs: Vec<(LogSource, Log)>,
        block_meta: BTreeMap<u64, BlockMeta>,
        checkpoint_sources: Vec<String>,
        flush_changes_by_block: bool,
    ) -> anyhow::Result<()> {
        let range_started = Instant::now();
        let block_started = Instant::now();
        let blocks = block_meta
            .values()
            .map(|meta| {
                Ok(BlockInsert {
                    number: meta.number_i64()?,
                    hash: types::hex_b256(meta.hash),
                    parent_hash: Some(types::hex_b256(meta.parent_hash)),
                    timestamp: meta.timestamp_i64()?,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        self.storage.blocks().upsert_many(blocks).await?;
        let block_write_ms = block_started.elapsed().as_millis();

        let decode_started = Instant::now();
        let mut decoded = Vec::new();
        let raw_log_count = raw_logs.len();
        for (source, log) in raw_logs {
            match decode_log(source, log, &block_meta) {
                Ok(indexed) => decoded.push(indexed),
                Err(error) => tracing::warn!(?source, %error, "skipping undecodable log"),
            }
        }
        let decode_ms = decode_started.elapsed().as_millis();

        let sort_started = Instant::now();
        decoded.sort_by_key(|event| {
            (
                event.ctx.block_number,
                event.ctx.transaction_index,
                event.ctx.log_index,
            )
        });
        let sort_ms = sort_started.elapsed().as_millis();

        let projection_started = Instant::now();
        let mut current_block = None;
        let mut changed_rows = 0;
        let mut current_flush_rows = 0;
        let mut current_flush_ms = 0;
        let mut change_flush_ms = 0;
        for event in decoded {
            if flush_changes_by_block
                && current_block.is_some_and(|block| block != event.ctx.block_number)
            {
                let current_flush_started = Instant::now();
                let current_stats = self.storage.flush_entity_cache().await?;
                current_flush_ms += current_flush_started.elapsed().as_millis();
                current_flush_rows += current_stats.rows;
                let change_flush_started = Instant::now();
                let flushed = self.storage.flush_change_buffer().await?;
                change_flush_ms += change_flush_started.elapsed().as_millis();
                changed_rows += flushed;
                tracing::debug!(flushed_changes = flushed, "flushed replay change buffer");
            }
            current_block = Some(event.ctx.block_number);
            projection::apply_event(&self.storage, event).await?;
        }
        if flush_changes_by_block {
            let current_flush_started = Instant::now();
            let current_stats = self.storage.flush_entity_cache().await?;
            current_flush_ms += current_flush_started.elapsed().as_millis();
            current_flush_rows += current_stats.rows;
            let change_flush_started = Instant::now();
            let flushed = self.storage.flush_change_buffer().await?;
            change_flush_ms += change_flush_started.elapsed().as_millis();
            changed_rows += flushed;
            tracing::debug!(flushed_changes = flushed, "flushed replay change buffer");
        }
        let projection_ms = projection_started.elapsed().as_millis();

        let event_flush_started = Instant::now();
        let event_rows = if flush_changes_by_block {
            let stats = self.storage.flush_event_buffer().await?;
            stats.rows
        } else {
            0
        };
        let event_flush_ms = event_flush_started.elapsed().as_millis();

        let checkpoint_started = Instant::now();
        if let Some(meta) = block_meta.get(&range_end) {
            for source in checkpoint_sources {
                self.storage
                    .checkpoints()
                    .upsert(&source, meta.number_i64()?, &types::hex_b256(meta.hash))
                    .await?;
            }
        }
        let checkpoint_ms = checkpoint_started.elapsed().as_millis();

        tracing::info!(
            raw_logs = raw_log_count,
            event_rows,
            current_flush_rows,
            changed_rows,
            block_write_ms,
            decode_ms,
            sort_ms,
            projection_ms,
            current_flush_ms,
            change_flush_ms,
            event_flush_ms,
            checkpoint_ms,
            elapsed_ms = range_started.elapsed().as_millis(),
            "applied raw replay range"
        );

        Ok(())
    }
}
