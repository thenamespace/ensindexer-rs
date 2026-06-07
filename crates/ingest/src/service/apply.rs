use std::collections::BTreeMap;

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

        let result = self
            .apply_raw_range(range_end, raw_logs, block_meta, checkpoint_sources)
            .await;

        match result {
            Ok(()) => {
                sqlx::query("commit").execute(self.storage.pool()).await?;
                Ok(())
            }
            Err(error) => {
                if let Err(rollback_error) =
                    sqlx::query("rollback").execute(self.storage.pool()).await
                {
                    tracing::error!(%rollback_error, "failed to roll back raw replay range");
                }
                Err(error)
            }
        }
    }

    pub(super) async fn apply_raw_range(
        &self,
        range_end: u64,
        raw_logs: Vec<(LogSource, Log)>,
        block_meta: BTreeMap<u64, BlockMeta>,
        checkpoint_sources: Vec<String>,
    ) -> anyhow::Result<()> {
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

        if let Some(meta) = block_meta.get(&range_end) {
            for source in checkpoint_sources {
                self.storage
                    .checkpoints()
                    .upsert(&source, meta.number_i64()?, &types::hex_b256(meta.hash))
                    .await?;
            }
        }

        Ok(())
    }
}
