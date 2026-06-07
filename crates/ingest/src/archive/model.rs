use std::collections::BTreeMap;

use alloy::rpc::types::Log;
use serde::{Deserialize, Serialize};

use crate::{rpc::BlockMeta, sources::LogSource};

pub(crate) const ARCHIVE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ArchivedRange {
    pub(crate) version: u32,
    pub(crate) chain_id: u64,
    pub(crate) from_block: u64,
    pub(crate) to_block: u64,
    pub(crate) logs: Vec<ArchivedLog>,
    pub(crate) block_meta: BTreeMap<u64, BlockMeta>,
    pub(crate) checkpoint_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ArchivedLog {
    pub(crate) source: LogSource,
    pub(crate) log: Log,
}

impl ArchivedRange {
    pub(crate) fn new(
        chain_id: u64,
        from_block: u64,
        to_block: u64,
        raw_logs: Vec<(LogSource, Log)>,
        block_meta: BTreeMap<u64, BlockMeta>,
        checkpoint_sources: Vec<String>,
    ) -> Self {
        Self {
            version: ARCHIVE_VERSION,
            chain_id,
            from_block,
            to_block,
            logs: raw_logs
                .into_iter()
                .map(|(source, log)| ArchivedLog { source, log })
                .collect(),
            block_meta,
            checkpoint_sources,
        }
    }

    pub(crate) fn into_parts(self) -> (Vec<(LogSource, Log)>, BTreeMap<u64, BlockMeta>) {
        (
            self.logs
                .into_iter()
                .map(|entry| (entry.source, entry.log))
                .collect(),
            self.block_meta,
        )
    }

    pub(crate) fn validate(&self, expected_chain_id: u64) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.version == ARCHIVE_VERSION,
            "unsupported raw archive version {}",
            self.version
        );
        anyhow::ensure!(
            self.chain_id == expected_chain_id,
            "raw archive chain_id {} does not match configured chain_id {}",
            self.chain_id,
            expected_chain_id
        );
        anyhow::ensure!(
            self.from_block <= self.to_block,
            "raw archive has invalid range {}..{}",
            self.from_block,
            self.to_block
        );
        Ok(())
    }
}
