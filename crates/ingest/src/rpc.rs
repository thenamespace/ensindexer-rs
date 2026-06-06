use std::{cmp, collections::BTreeMap};

use alloy::{
    consensus::BlockHeader,
    network::{BlockResponse, Network},
    providers::Provider,
    rpc::types::{BlockNumberOrTag, Filter, Log},
};
use alloy_network_primitives::HeaderResponse;
use alloy_primitives::{Address, B256};
use contracts::{fixed_source_topic0s, resolver_topic0s};
use serde::{Deserialize, Serialize};

use crate::sources::{FixedSource, LogSource};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct BlockMeta {
    pub number: u64,
    pub hash: B256,
    pub parent_hash: B256,
    pub timestamp: u64,
}

impl BlockMeta {
    pub(crate) fn number_i64(&self) -> anyhow::Result<i64> {
        self.number
            .try_into()
            .map_err(|_| anyhow::anyhow!("block number does not fit i64: {}", self.number))
    }

    pub(crate) fn timestamp_i64(&self) -> anyhow::Result<i64> {
        self.timestamp
            .try_into()
            .map_err(|_| anyhow::anyhow!("block timestamp does not fit i64: {}", self.timestamp))
    }
}

pub(crate) async fn fetch_source_logs<P, N>(
    provider: &P,
    source: &FixedSource,
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<Vec<Log>>
where
    P: Provider<N>,
    N: Network,
{
    let from_block = cmp::max(from_block, source.start_block);
    if from_block > to_block {
        return Ok(Vec::new());
    }

    let filter = Filter::new()
        .address(source.address)
        .event_signature(fixed_source_topic0s(source.source))
        .from_block(BlockNumberOrTag::Number(from_block))
        .to_block(BlockNumberOrTag::Number(to_block));

    Ok(provider.get_logs(&filter).await?)
}

pub(crate) async fn fetch_resolver_logs<P, N>(
    provider: &P,
    addresses: &[Address],
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<Vec<Log>>
where
    P: Provider<N>,
    N: Network,
{
    const ADDRESS_CHUNK_SIZE: usize = 500;

    let mut logs = Vec::new();
    for chunk in addresses.chunks(ADDRESS_CHUNK_SIZE) {
        if chunk.is_empty() {
            continue;
        }

        let filter = Filter::new()
            .address(chunk.to_vec())
            .event_signature(resolver_topic0s())
            .from_block(BlockNumberOrTag::Number(from_block))
            .to_block(BlockNumberOrTag::Number(to_block));

        logs.extend(provider.get_logs(&filter).await?);
    }

    Ok(logs)
}

pub(crate) async fn fetch_block_metadata<P, N>(
    provider: &P,
    logs: &[(LogSource, Log)],
) -> anyhow::Result<BTreeMap<u64, BlockMeta>>
where
    P: Provider<N>,
    N: Network,
    N::BlockResponse: BlockResponse,
    <N::BlockResponse as BlockResponse>::Header: BlockHeader,
{
    let mut metas = BTreeMap::new();

    for (_, log) in logs {
        let block_number = log
            .block_number
            .ok_or_else(|| anyhow::anyhow!("log is missing required field block_number"))?;

        if metas.contains_key(&block_number) {
            continue;
        }

        metas.insert(
            block_number,
            fetch_block_meta_by_number(provider, block_number).await?,
        );
    }

    Ok(metas)
}

pub(crate) async fn fetch_block_meta_by_number<P, N>(
    provider: &P,
    block_number: u64,
) -> anyhow::Result<BlockMeta>
where
    P: Provider<N>,
    N: Network,
    N::BlockResponse: BlockResponse,
    <N::BlockResponse as BlockResponse>::Header: BlockHeader + HeaderResponse,
{
    let block = provider
        .get_block_by_number(BlockNumberOrTag::Number(block_number))
        .await?
        .ok_or_else(|| anyhow::anyhow!("block {block_number} was not found"))?;
    let header = block.header();

    Ok(BlockMeta {
        number: header.number(),
        hash: header.hash(),
        parent_hash: header.parent_hash(),
        timestamp: header.timestamp(),
    })
}
