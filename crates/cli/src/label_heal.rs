use std::{fs, path::Path, time::Instant};

use anyhow::{Context, bail};
use storage::Storage;

#[derive(Debug, Clone)]
pub(crate) struct HealOptions {
    pub labelhashes: Vec<String>,
    pub limit: i64,
    pub repair_passes: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct ImportOptions {
    pub input: std::path::PathBuf,
    pub chunk_size: usize,
}

pub(crate) async fn import(storage: Storage, options: ImportOptions) -> anyhow::Result<()> {
    let started = Instant::now();
    let records = read_records(&options.input)?;
    let mut imported = 0usize;
    for chunk in records.chunks(options.chunk_size.max(1)) {
        storage.label_preimages().upsert_many(chunk).await?;
        imported += chunk.len();
        tracing::info!(
            imported,
            total = records.len(),
            "imported local label records"
        );
    }

    println!(
        "label import complete: records={} elapsed_ms={}",
        records.len(),
        started.elapsed().as_millis()
    );
    Ok(())
}

pub(crate) async fn run(storage: Storage, options: HealOptions) -> anyhow::Result<()> {
    let started = Instant::now();
    let labelhashes = if options.labelhashes.is_empty() {
        storage
            .label_preimages()
            .repairable_labelhashes(options.limit)
            .await?
    } else {
        options.labelhashes
    };
    if labelhashes.is_empty() {
        println!("label repair candidates: 0");
        return Ok(());
    }

    let repaired = storage
        .label_preimages()
        .repair_domain_names_for_labelhashes(&labelhashes, options.repair_passes)
        .await?;

    println!(
        "label repair complete: labelhashes={} repaired_rows={} elapsed_ms={}",
        labelhashes.len(),
        repaired,
        started.elapsed().as_millis()
    );
    Ok(())
}

fn read_records(path: &Path) -> anyhow::Result<Vec<(String, String)>> {
    let bytes = fs::read(path).with_context(|| format!("read label file {}", path.display()))?;
    if path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("ensrainbow"))
    {
        return parse_ensrainbow(&bytes);
    }

    parse_tsv(&bytes)
}

fn parse_tsv(bytes: &[u8]) -> anyhow::Result<Vec<(String, String)>> {
    let input = std::str::from_utf8(bytes)?;
    let mut records = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let Some((labelhash, label)) = line.split_once('\t') else {
            bail!(
                "invalid label row {}: expected labelhash<TAB>label",
                index + 1
            );
        };
        records.push((normalize_labelhash(labelhash)?, label.to_owned()));
    }
    Ok(records)
}

fn parse_ensrainbow(bytes: &[u8]) -> anyhow::Result<Vec<(String, String)>> {
    let mut cursor = bytes;
    let header_bytes = read_delimited_message(&mut cursor).context("read .ensrainbow header")?;
    validate_ensrainbow_header(header_bytes)?;

    let mut records = Vec::new();
    while !cursor.is_empty() {
        let record_bytes = read_delimited_message(&mut cursor)
            .with_context(|| format!("read .ensrainbow record {}", records.len() + 1))?;
        records.push(parse_record(record_bytes)?);
    }
    Ok(records)
}

fn validate_ensrainbow_header(bytes: &[u8]) -> anyhow::Result<()> {
    let mut format_identifier = None;
    let mut file_format_version = None;
    for (field, value) in parse_message(bytes)? {
        match (field, value) {
            (1, WireValue::LengthDelimited(value)) => {
                format_identifier = Some(std::str::from_utf8(value)?.to_owned());
            }
            (2, WireValue::Varint(value)) => file_format_version = Some(value),
            _ => {}
        }
    }

    let format_identifier = format_identifier.context(".ensrainbow header missing format")?;
    if format_identifier != "ensrainbow" {
        bail!("invalid .ensrainbow format identifier {format_identifier}");
    }
    let file_format_version = file_format_version.context(".ensrainbow header missing version")?;
    if file_format_version != 1 {
        bail!("unsupported .ensrainbow file format version {file_format_version}");
    }
    Ok(())
}

fn parse_record(bytes: &[u8]) -> anyhow::Result<(String, String)> {
    let mut labelhash = None;
    let mut label = None;
    for (field, value) in parse_message(bytes)? {
        match (field, value) {
            (1, WireValue::LengthDelimited(value)) => labelhash = Some(format_labelhash(value)?),
            (2, WireValue::LengthDelimited(value)) => {
                label = Some(std::str::from_utf8(value)?.to_owned());
            }
            _ => {}
        }
    }

    Ok((
        labelhash.context("rainbow record missing labelhash")?,
        label.context("rainbow record missing label")?,
    ))
}

fn normalize_labelhash(value: &str) -> anyhow::Result<String> {
    let trimmed = value.trim();
    let hex = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    if hex.len() != 64 || !hex.as_bytes().iter().all(u8::is_ascii_hexdigit) {
        bail!("invalid labelhash {value}");
    }
    Ok(format!("0x{}", hex.to_ascii_lowercase()))
}

fn format_labelhash(bytes: &[u8]) -> anyhow::Result<String> {
    if bytes.len() != 32 {
        bail!("invalid labelhash byte length {}", bytes.len());
    }
    let mut out = String::with_capacity(66);
    out.push_str("0x");
    for byte in bytes {
        use std::fmt::Write;
        write!(&mut out, "{byte:02x}")?;
    }
    Ok(out)
}

fn parse_message(mut bytes: &[u8]) -> anyhow::Result<Vec<(u64, WireValue<'_>)>> {
    let mut fields = Vec::new();
    while !bytes.is_empty() {
        let key = read_varint(&mut bytes)?;
        let field = key >> 3;
        let wire_type = key & 0x07;
        let value = match wire_type {
            0 => {
                let value = read_varint(&mut bytes)?;
                WireValue::Varint(value)
            }
            2 => {
                let len = read_varint(&mut bytes)? as usize;
                if bytes.len() < len {
                    bail!("truncated protobuf length-delimited field");
                }
                let (field_bytes, rest) = bytes.split_at(len);
                bytes = rest;
                WireValue::LengthDelimited(field_bytes)
            }
            _ => bail!("unsupported protobuf wire type {wire_type}"),
        };
        fields.push((field, value));
    }
    Ok(fields)
}

fn read_delimited_message<'a>(bytes: &mut &'a [u8]) -> anyhow::Result<&'a [u8]> {
    let len = read_varint(bytes)? as usize;
    if bytes.len() < len {
        bail!("truncated protobuf delimited message");
    }
    let (message, rest) = bytes.split_at(len);
    *bytes = rest;
    Ok(message)
}

fn read_varint(bytes: &mut &[u8]) -> anyhow::Result<u64> {
    let mut value = 0u64;
    for shift in (0..64).step_by(7) {
        let Some((&byte, rest)) = bytes.split_first() else {
            bail!("truncated protobuf varint");
        };
        *bytes = rest;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    bail!("protobuf varint overflow")
}

enum WireValue<'a> {
    Varint(u64),
    LengthDelimited(&'a [u8]),
}
