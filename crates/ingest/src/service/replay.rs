use std::{path::PathBuf, time::Instant};

use super::IngestService;
use crate::archive::{range_entries, read_range_entry};

impl IngestService {
    pub async fn replay_archive_range(
        &self,
        from_block: u64,
        to_block: u64,
        archive_dir: Option<PathBuf>,
    ) -> anyhow::Result<()> {
        let archive_dir = archive_dir
            .or_else(|| self.config.raw_archive_dir.clone())
            .ok_or_else(|| {
                anyhow::anyhow!("RAW_ARCHIVE_DIR or --archive-dir is required for replay")
            })?;

        tracing::info!(
            chain_id = self.config.chain_id,
            from_block,
            to_block,
            archive_dir = %archive_dir.display(),
            "starting raw archive replay"
        );

        let entries = range_entries(&archive_dir, self.config.chain_id, from_block, to_block)?;
        let total_ranges = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            let started_at = Instant::now();
            tracing::info!(
                archive_file = %entry.file,
                from_block = entry.from_block,
                to_block = entry.to_block,
                logs = entry.logs,
                bytes = entry.bytes,
                range_index = index + 1,
                total_ranges,
                "replaying raw archive range"
            );

            let range = read_range_entry(&archive_dir, self.config.chain_id, entry)?;
            let range_start = range.from_block;
            let range_end = range.to_block;
            let checkpoint_sources = range.checkpoint_sources.clone();
            let (raw_logs, block_meta) = range.into_parts();

            self.apply_raw_range_transactional(range_end, raw_logs, block_meta, checkpoint_sources)
                .await?;

            let elapsed = started_at.elapsed();
            tracing::info!(
                archive_file = %entry.file,
                from_block = range_start,
                to_block = range_end,
                range_index = index + 1,
                total_ranges,
                elapsed_ms = elapsed.as_millis(),
                "replayed raw archive range"
            );
        }

        Ok(())
    }
}
