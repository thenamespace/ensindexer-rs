use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use alloy::rpc::types::Log;
use serde::{Deserialize, Serialize};

use crate::{rpc::BlockMeta, sources::LogSource};

const ARCHIVE_VERSION: u32 = 1;

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

    fn validate(&self, expected_chain_id: u64) -> anyhow::Result<()> {
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

pub(crate) fn write_range(dir: &Path, range: &ArchivedRange) -> anyhow::Result<PathBuf> {
    let ranges_dir = dir.join("ranges");
    std::fs::create_dir_all(&ranges_dir)?;

    let path = range_path(dir, range.from_block, range.to_block);
    let tmp_path = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(range)?;
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, &path)?;
    Ok(path)
}

pub(crate) fn read_ranges(
    dir: &Path,
    expected_chain_id: u64,
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<Vec<ArchivedRange>> {
    anyhow::ensure!(
        from_block <= to_block,
        "from block must be less than or equal to to block"
    );

    let mut ranges = std::fs::read_dir(dir.join("ranges"))?
        .map(|entry| -> anyhow::Result<Option<ArchivedRange>> {
            let entry = entry?;
            if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
                return Ok(None);
            }

            let bytes = std::fs::read(entry.path())?;
            let range: ArchivedRange = serde_json::from_slice(&bytes)?;
            range.validate(expected_chain_id)?;
            if range.to_block < from_block || range.from_block > to_block {
                return Ok(None);
            }
            Ok(Some(range))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    ranges.sort_by_key(|range| range.from_block);
    validate_contiguous(&ranges, from_block, to_block)?;
    Ok(ranges)
}

fn range_path(dir: &Path, from_block: u64, to_block: u64) -> PathBuf {
    dir.join("ranges")
        .join(format!("{from_block:020}-{to_block:020}.json"))
}

fn validate_contiguous(
    ranges: &[ArchivedRange],
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<()> {
    let mut expected = from_block;
    for range in ranges {
        anyhow::ensure!(
            range.from_block == expected,
            "raw archive gap: expected range starting at {}, found {}..{}",
            expected,
            range.from_block,
            range.to_block
        );

        expected = range
            .to_block
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("raw archive range cannot end at u64::MAX"))?;
    }

    anyhow::ensure!(
        expected == to_block.saturating_add(1),
        "raw archive gap: missing range ending at {}",
        to_block
    );
    Ok(())
}
