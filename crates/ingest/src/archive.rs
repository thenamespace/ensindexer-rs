mod coverage;
mod manifest;
mod model;

use std::path::{Path, PathBuf};

pub use coverage::{ArchiveGap, ArchiveStatus, inspect_archive};
pub use manifest::ArchiveManifestRange;
pub(crate) use model::ArchivedRange;

use self::manifest::{
    upsert_manifest_range, verify_manifest_checksum, verify_manifest_range_checksum,
};

pub(crate) fn write_range(dir: &Path, range: &ArchivedRange) -> anyhow::Result<PathBuf> {
    let ranges_dir = dir.join("ranges");
    std::fs::create_dir_all(&ranges_dir)?;

    let path = range_path(dir, range.from_block, range.to_block);
    let tmp_path = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(range)?;
    let checksum = manifest::sha256_hex(&bytes);
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, &path)?;
    upsert_manifest_range(dir, range, &path, &checksum)?;
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
            verify_manifest_checksum(dir, &entry.path(), &bytes)?;
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

pub(crate) fn available_bounds(dir: &Path, expected_chain_id: u64) -> anyhow::Result<(u64, u64)> {
    let mut from = None::<u64>;
    let mut to = None::<u64>;

    for entry in std::fs::read_dir(dir.join("ranges"))? {
        let entry = entry?;
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let bytes = std::fs::read(entry.path())?;
        verify_manifest_checksum(dir, &entry.path(), &bytes)?;
        let range: ArchivedRange = serde_json::from_slice(&bytes)?;
        range.validate(expected_chain_id)?;
        from = Some(from.map_or(range.from_block, |current| current.min(range.from_block)));
        to = Some(to.map_or(range.to_block, |current| current.max(range.to_block)));
    }

    match (from, to) {
        (Some(from), Some(to)) => Ok((from, to)),
        _ => anyhow::bail!("raw archive contains no range files"),
    }
}

pub(crate) fn range_path(dir: &Path, from_block: u64, to_block: u64) -> PathBuf {
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

pub(crate) fn verify_manifest_range(
    dir: &Path,
    expected_chain_id: u64,
    entry: &ArchiveManifestRange,
) -> anyhow::Result<()> {
    verify_manifest_range_checksum(dir, expected_chain_id, entry)
}
