use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::model::ArchivedRange;

const MANIFEST_VERSION: u32 = 1;
const MANIFEST_FILE: &str = "manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct ArchiveManifest {
    pub(crate) version: u32,
    pub(crate) chain_id: u64,
    pub(crate) ranges: Vec<ArchiveManifestRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveManifestRange {
    pub from_block: u64,
    pub to_block: u64,
    pub file: String,
    pub sha256: String,
    pub bytes: u64,
    pub logs: usize,
}

pub(crate) fn load_manifest(dir: &Path) -> anyhow::Result<ArchiveManifest> {
    let path = manifest_path(dir);
    if !path.exists() {
        return build_manifest_from_ranges(dir);
    }

    let bytes = std::fs::read(&path)?;
    let manifest: ArchiveManifest = serde_json::from_slice(&bytes)?;
    anyhow::ensure!(
        manifest.version == MANIFEST_VERSION,
        "unsupported archive manifest version {}",
        manifest.version
    );
    Ok(manifest)
}

pub(crate) fn upsert_manifest_range(
    dir: &Path,
    range: &ArchivedRange,
    path: &Path,
    checksum: &str,
) -> anyhow::Result<()> {
    let file = relative_archive_path(dir, path)?;
    let bytes = std::fs::metadata(path)?.len();
    let mut manifest = load_manifest(dir).unwrap_or_else(|_| ArchiveManifest {
        version: MANIFEST_VERSION,
        chain_id: range.chain_id,
        ranges: Vec::new(),
    });
    manifest.version = MANIFEST_VERSION;
    manifest.chain_id = range.chain_id;

    manifest.ranges.retain(|entry| {
        !(entry.from_block == range.from_block && entry.to_block == range.to_block)
    });
    manifest.ranges.push(ArchiveManifestRange {
        from_block: range.from_block,
        to_block: range.to_block,
        file,
        sha256: checksum.to_owned(),
        bytes,
        logs: range.logs.len(),
    });
    manifest.ranges.sort_by_key(|entry| entry.from_block);
    write_manifest(dir, &manifest)
}

pub(crate) fn verify_manifest_range_checksum(
    dir: &Path,
    expected_chain_id: u64,
    entry: &ArchiveManifestRange,
) -> anyhow::Result<()> {
    let path = dir.join(&entry.file);
    let bytes = std::fs::read(&path)?;
    let actual = sha256_hex(&bytes);
    anyhow::ensure!(
        actual == entry.sha256,
        "archive checksum mismatch for {}: manifest={} actual={}",
        entry.file,
        entry.sha256,
        actual
    );
    anyhow::ensure!(
        bytes.len() as u64 == entry.bytes,
        "archive byte length mismatch for {}: manifest={} actual={}",
        entry.file,
        entry.bytes,
        bytes.len()
    );
    let range: ArchivedRange = serde_json::from_slice(&bytes)?;
    range.validate(expected_chain_id)?;
    anyhow::ensure!(
        range.from_block == entry.from_block && range.to_block == entry.to_block,
        "archive range mismatch for {}",
        entry.file
    );
    Ok(())
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn manifest_path(dir: &Path) -> PathBuf {
    dir.join(MANIFEST_FILE)
}

fn write_manifest(dir: &Path, manifest: &ArchiveManifest) -> anyhow::Result<()> {
    let path = manifest_path(dir);
    let tmp_path = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(manifest)?;
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, &path)?;
    Ok(())
}

fn build_manifest_from_ranges(dir: &Path) -> anyhow::Result<ArchiveManifest> {
    let mut ranges = Vec::new();
    let mut chain_id = None;
    let ranges_dir = dir.join("ranges");
    if !ranges_dir.exists() {
        return Ok(ArchiveManifest {
            version: MANIFEST_VERSION,
            chain_id: 0,
            ranges,
        });
    }

    for entry in std::fs::read_dir(ranges_dir)? {
        let entry = entry?;
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let bytes = std::fs::read(entry.path())?;
        let range: ArchivedRange = serde_json::from_slice(&bytes)?;
        chain_id = Some(chain_id.unwrap_or(range.chain_id));
        let file = relative_archive_path(dir, &entry.path())?;
        ranges.push(ArchiveManifestRange {
            from_block: range.from_block,
            to_block: range.to_block,
            file,
            sha256: sha256_hex(&bytes),
            bytes: bytes.len().try_into()?,
            logs: range.logs.len(),
        });
    }
    ranges.sort_by_key(|range| range.from_block);
    Ok(ArchiveManifest {
        version: MANIFEST_VERSION,
        chain_id: chain_id.unwrap_or_default(),
        ranges,
    })
}

fn relative_archive_path(dir: &Path, path: &Path) -> anyhow::Result<String> {
    Ok(path
        .strip_prefix(dir)?
        .to_string_lossy()
        .trim_start_matches('/')
        .to_owned())
}
