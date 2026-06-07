mod coverage;
mod manifest;
mod model;
mod resolvers;

use std::path::{Path, PathBuf};

pub use coverage::{ArchiveGap, ArchiveStatus, inspect_archive};
pub use manifest::ArchiveManifestRange;
pub(crate) use model::ArchivedRange;
pub use resolvers::{ResolverCacheStatus, rebuild_resolver_cache};
pub(crate) use resolvers::{add_resolver_from_log, load_resolver_cache, write_resolver_cache};

use self::manifest::{load_manifest, upsert_manifest_range, verify_manifest_range_checksum};

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

pub(crate) fn range_entries(
    dir: &Path,
    expected_chain_id: u64,
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<Vec<ArchiveManifestRange>> {
    anyhow::ensure!(
        from_block <= to_block,
        "from block must be less than or equal to to block"
    );

    let manifest = load_manifest(dir)?;
    anyhow::ensure!(
        manifest.chain_id == 0 || manifest.chain_id == expected_chain_id,
        "archive manifest chain_id {} does not match configured chain_id {}",
        manifest.chain_id,
        expected_chain_id
    );
    let mut ranges = manifest
        .ranges
        .into_iter()
        .filter(|range| range.to_block >= from_block && range.from_block <= to_block)
        .collect::<Vec<_>>();
    ranges.sort_by_key(|range| range.from_block);
    validate_contiguous_entries(&ranges, from_block, to_block)?;
    Ok(ranges)
}

pub(crate) fn read_range_entry(
    dir: &Path,
    expected_chain_id: u64,
    entry: &ArchiveManifestRange,
) -> anyhow::Result<ArchivedRange> {
    verify_manifest_range_checksum(dir, expected_chain_id, entry)?;
    let bytes = std::fs::read(dir.join(&entry.file))?;
    let range: ArchivedRange = serde_json::from_slice(&bytes)?;
    range.validate(expected_chain_id)?;
    Ok(range)
}

pub(crate) fn available_bounds(dir: &Path, expected_chain_id: u64) -> anyhow::Result<(u64, u64)> {
    let manifest = load_manifest(dir)?;
    anyhow::ensure!(
        manifest.chain_id == 0 || manifest.chain_id == expected_chain_id,
        "archive manifest chain_id {} does not match configured chain_id {}",
        manifest.chain_id,
        expected_chain_id
    );
    let from = manifest.ranges.iter().map(|range| range.from_block).min();
    let to = manifest.ranges.iter().map(|range| range.to_block).max();

    match (from, to) {
        (Some(from), Some(to)) => Ok((from, to)),
        _ => anyhow::bail!("raw archive contains no range files"),
    }
}

pub(crate) fn range_path(dir: &Path, from_block: u64, to_block: u64) -> PathBuf {
    dir.join("ranges")
        .join(format!("{from_block:020}-{to_block:020}.json"))
}

fn validate_contiguous_entries(
    ranges: &[ArchiveManifestRange],
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
