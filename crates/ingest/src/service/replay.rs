use std::path::PathBuf;

use super::IngestService;
use crate::archive::read_ranges;

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

        for range in read_ranges(&archive_dir, self.config.chain_id, from_block, to_block)? {
            let range_start = range.from_block;
            let range_end = range.to_block;
            let checkpoint_sources = range.checkpoint_sources.clone();
            let (raw_logs, block_meta) = range.into_parts();

            self.apply_raw_range(range_end, raw_logs, block_meta, checkpoint_sources)
                .await?;

            tracing::info!(
                from_block = range_start,
                to_block = range_end,
                "replayed raw archive range"
            );
        }

        Ok(())
    }
}
