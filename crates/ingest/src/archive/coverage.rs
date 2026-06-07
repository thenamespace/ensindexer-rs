use std::path::Path;

use super::{
    manifest::{ArchiveManifestRange, load_manifest},
    verify_manifest_range,
};

#[derive(Debug, Clone)]
pub struct ArchiveStatus {
    pub chain_id: u64,
    pub ranges: Vec<ArchiveManifestRange>,
    pub gaps: Vec<ArchiveGap>,
}

#[derive(Debug, Clone)]
pub struct ArchiveGap {
    pub from_block: u64,
    pub to_block: u64,
}

impl ArchiveStatus {
    pub fn is_contiguous(&self) -> bool {
        self.gaps.is_empty()
    }
}

pub fn inspect_archive(
    dir: &Path,
    expected_chain_id: u64,
    from_block: Option<u64>,
    to_block: Option<u64>,
) -> anyhow::Result<ArchiveStatus> {
    let mut ranges = load_manifest(dir)?
        .ranges
        .into_iter()
        .filter(|range| {
            from_block.is_none_or(|from| range.to_block >= from)
                && to_block.is_none_or(|to| range.from_block <= to)
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|range| range.from_block);
    for range in &ranges {
        verify_manifest_range(dir, expected_chain_id, range)?;
    }

    Ok(ArchiveStatus {
        chain_id: expected_chain_id,
        gaps: coverage_gaps(&ranges, from_block, to_block),
        ranges,
    })
}

fn coverage_gaps(
    ranges: &[ArchiveManifestRange],
    from_block: Option<u64>,
    to_block: Option<u64>,
) -> Vec<ArchiveGap> {
    let Some(mut expected) = from_block.or_else(|| ranges.first().map(|range| range.from_block))
    else {
        return Vec::new();
    };
    let target = to_block.unwrap_or_else(|| ranges.last().map_or(expected, |range| range.to_block));
    let mut gaps = Vec::new();

    for range in ranges {
        if range.to_block < expected {
            continue;
        }
        if range.from_block > expected {
            gaps.push(ArchiveGap {
                from_block: expected,
                to_block: range.from_block.saturating_sub(1),
            });
        }
        expected = range.to_block.saturating_add(1);
        if expected > target {
            break;
        }
    }

    if expected <= target {
        gaps.push(ArchiveGap {
            from_block: expected,
            to_block: target,
        });
    }

    gaps
}
