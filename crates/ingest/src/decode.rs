use std::collections::BTreeMap;

use alloy::rpc::types::Log;
use alloy_primitives::B256;
use contracts::{IndexedEvent, decode_fixed_source_log, decode_resolver_log};
use types::LogContext;

use crate::{rpc::BlockMeta, sources::LogSource};

pub(crate) fn decode_log(
    source: LogSource,
    log: Log,
    block_meta: &BTreeMap<u64, BlockMeta>,
) -> anyhow::Result<IndexedEvent> {
    let event = match source {
        LogSource::Fixed(source) => decode_fixed_source_log(source, &log)?,
        LogSource::Resolver => decode_resolver_log(&log)?,
    };
    let ctx = log_context(&log, block_meta)?;
    Ok(IndexedEvent { ctx, event })
}

fn log_context(log: &Log, block_meta: &BTreeMap<u64, BlockMeta>) -> anyhow::Result<LogContext> {
    let block_number = log
        .block_number
        .ok_or_else(|| anyhow::anyhow!("log is missing required field block_number"))?;
    let meta = block_meta.get(&block_number);

    Ok(LogContext {
        block_number: u64_to_i64(block_number, "block_number")?,
        block_timestamp: match (log.block_timestamp, meta) {
            (Some(timestamp), _) => u64_to_i64(timestamp, "block_timestamp")?,
            (None, Some(meta)) => meta.timestamp_i64()?,
            (None, None) => 0,
        },
        block_hash: log
            .block_hash
            .or_else(|| meta.map(|meta| meta.hash))
            .ok_or_else(|| anyhow::anyhow!("log is missing required field block_hash"))?,
        transaction_hash: required_b256(log.transaction_hash, "transaction_hash")?,
        transaction_index: required_u64_to_i64(log.transaction_index, "transaction_index")?,
        log_index: required_u64_to_i64(log.log_index, "log_index")?,
        contract_address: log.address(),
    })
}

fn required_b256(value: Option<B256>, field: &'static str) -> anyhow::Result<B256> {
    value.ok_or_else(|| anyhow::anyhow!("log is missing required field {field}"))
}

fn required_u64_to_i64(value: Option<u64>, field: &'static str) -> anyhow::Result<i64> {
    let value = value.ok_or_else(|| anyhow::anyhow!("log is missing required field {field}"))?;
    u64_to_i64(value, field)
}

fn u64_to_i64(value: u64, field: &'static str) -> anyhow::Result<i64> {
    value
        .try_into()
        .map_err(|_| anyhow::anyhow!("log field {field} does not fit i64: {value}"))
}
