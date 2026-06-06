use std::{cmp, collections::BTreeMap};

use alloy::rpc::types::Log as RpcLog;
use alloy_primitives::{Address, B256, Bytes, Log as PrimitiveLog};
use contracts::{fixed_source_topic0s, resolver_topic0s};
use hypersync_client::{
    Client, ClientConfig,
    format::{Data, Hash, Quantity, UInt},
    net_types::{
        LogFilter, Query, block::BlockField, log::LogField, transaction::TransactionField,
    },
    simple_types::{Block, Event, Log},
};

use crate::{
    rpc::BlockMeta,
    sources::{FixedSource, LogSource},
};

#[derive(Debug, Clone)]
pub(crate) struct HypersyncBackfillClient {
    client: Client,
}

impl HypersyncBackfillClient {
    pub(crate) fn new(
        url: impl Into<String>,
        api_token: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let client = Client::new(ClientConfig {
            url: url.into(),
            api_token: api_token.into(),
            ..Default::default()
        })?;

        Ok(Self { client })
    }

    pub(crate) async fn fetch_source_logs(
        &self,
        source: &FixedSource,
        from_block: u64,
        to_block: u64,
    ) -> anyhow::Result<LogBatch> {
        let from_block = cmp::max(from_block, source.start_block);
        if from_block > to_block {
            return Ok(LogBatch::default());
        }

        let address = source.address.to_string();
        let topics = fixed_source_topic0s(source.source)
            .into_iter()
            .map(|topic| topic.to_string())
            .collect::<Vec<_>>();
        let query = logs_query(from_block, to_block)?.where_logs(
            LogFilter::all()
                .and_address([address])?
                .and_topic0(topics)?,
        );

        self.fetch_logs(query, LogSource::Fixed(source.source))
            .await
    }

    pub(crate) async fn fetch_resolver_logs(
        &self,
        addresses: &[Address],
        from_block: u64,
        to_block: u64,
    ) -> anyhow::Result<LogBatch> {
        const ADDRESS_CHUNK_SIZE: usize = 1_000;

        let mut batch = LogBatch::default();
        let topics = resolver_topic0s()
            .into_iter()
            .map(|topic| topic.to_string())
            .collect::<Vec<_>>();

        for chunk in addresses.chunks(ADDRESS_CHUNK_SIZE) {
            if chunk.is_empty() {
                continue;
            }

            let address_strings = chunk.iter().map(ToString::to_string).collect::<Vec<_>>();
            let query = logs_query(from_block, to_block)?.where_logs(
                LogFilter::all()
                    .and_address(address_strings)?
                    .and_topic0(topics.clone())?,
            );

            batch.extend(self.fetch_logs(query, LogSource::Resolver).await?);
        }

        Ok(batch)
    }

    pub(crate) async fn fetch_block_meta_by_number(
        &self,
        block_number: u64,
    ) -> anyhow::Result<BlockMeta> {
        let query = Query::new()
            .from_block(block_number)
            .to_block_excl(block_number.saturating_add(1))
            .include_all_blocks()
            .select_block_fields(block_fields());
        let response = self.client.get(&query).await?;

        for blocks in response.data.blocks {
            if let Some(block) = blocks.into_iter().next() {
                return meta_from_block(&block);
            }
        }

        anyhow::bail!("block {block_number} was not found by hypersync")
    }

    async fn fetch_logs(&self, mut query: Query, source: LogSource) -> anyhow::Result<LogBatch> {
        let mut batch = LogBatch::default();
        let target_block = query.to_block.unwrap_or(u64::MAX);

        loop {
            let response = self.client.get_events(query.clone()).await?;
            let next_block = response.next_block;

            for event in response.data {
                let Some(log) = convert_event(event, &mut batch.block_meta)? else {
                    continue;
                };
                batch.raw_logs.push((source, log));
            }

            if next_block >= target_block {
                break;
            }
            anyhow::ensure!(
                next_block > query.from_block,
                "hypersync pagination did not advance from block {}",
                query.from_block
            );
            query.from_block = next_block;
        }

        Ok(batch)
    }
}

#[derive(Debug, Default)]
pub(crate) struct LogBatch {
    pub(crate) raw_logs: Vec<(LogSource, RpcLog)>,
    pub(crate) block_meta: BTreeMap<u64, BlockMeta>,
}

impl LogBatch {
    pub(crate) fn extend(&mut self, other: Self) {
        self.raw_logs.extend(other.raw_logs);
        self.block_meta.extend(other.block_meta);
    }
}

fn logs_query(from_block: u64, to_block: u64) -> anyhow::Result<Query> {
    let to_block_excl = to_block
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("to_block cannot be u64::MAX for hypersync queries"))?;

    Ok(Query::new()
        .from_block(from_block)
        .to_block_excl(to_block_excl)
        .select_block_fields(block_fields())
        .select_transaction_fields([TransactionField::Hash])
        .select_log_fields([
            LogField::Address,
            LogField::Data,
            LogField::Topic0,
            LogField::Topic1,
            LogField::Topic2,
            LogField::Topic3,
            LogField::BlockHash,
            LogField::BlockNumber,
            LogField::TransactionHash,
            LogField::TransactionIndex,
            LogField::LogIndex,
            LogField::Removed,
        ]))
}

fn block_fields() -> [BlockField; 4] {
    [
        BlockField::Number,
        BlockField::Hash,
        BlockField::ParentHash,
        BlockField::Timestamp,
    ]
}

fn convert_event(
    event: Event,
    block_meta: &mut BTreeMap<u64, BlockMeta>,
) -> anyhow::Result<Option<RpcLog>> {
    if let Some(block) = event.block.as_deref()
        && let Ok(meta) = meta_from_block(block)
    {
        block_meta.entry(meta.number).or_insert(meta);
    }

    convert_log(event.log).map(Some)
}

fn convert_log(log: Log) -> anyhow::Result<RpcLog> {
    let address = log
        .address
        .as_ref()
        .map(address)
        .ok_or_else(|| anyhow::anyhow!("hypersync log missing address"))?;
    let topics = log
        .topics
        .iter()
        .filter_map(|topic| topic.as_ref().map(b256))
        .collect::<Vec<_>>();
    let data = log.data.as_ref().map(bytes).unwrap_or_default();

    Ok(RpcLog {
        inner: PrimitiveLog::new_unchecked(address, topics, data),
        block_hash: log.block_hash.as_ref().map(b256),
        block_number: log.block_number.map(uint),
        block_timestamp: None,
        transaction_hash: log.transaction_hash.as_ref().map(b256),
        transaction_index: log.transaction_index.map(uint),
        log_index: log.log_index.map(uint),
        removed: log.removed.unwrap_or(false),
    })
}

fn meta_from_block(block: &Block) -> anyhow::Result<BlockMeta> {
    Ok(BlockMeta {
        number: block
            .number
            .ok_or_else(|| anyhow::anyhow!("hypersync block missing number"))?,
        hash: block
            .hash
            .as_ref()
            .map(b256)
            .ok_or_else(|| anyhow::anyhow!("hypersync block missing hash"))?,
        parent_hash: block
            .parent_hash
            .as_ref()
            .map(b256)
            .ok_or_else(|| anyhow::anyhow!("hypersync block missing parent hash"))?,
        timestamp: block
            .timestamp
            .as_ref()
            .map(quantity)
            .transpose()?
            .ok_or_else(|| anyhow::anyhow!("hypersync block missing timestamp"))?,
    })
}

fn b256(value: &Hash) -> B256 {
    B256::from_slice(value.as_ref())
}

fn address(value: &hypersync_client::format::Address) -> Address {
    Address::from_slice(value.as_ref())
}

fn bytes(value: &Data) -> Bytes {
    Bytes::copy_from_slice(value.as_ref())
}

fn uint(value: UInt) -> u64 {
    *value
}

fn quantity(value: &Quantity) -> anyhow::Result<u64> {
    let bytes = value.as_ref();
    anyhow::ensure!(
        bytes.len() <= 8,
        "hypersync quantity does not fit u64: {value:?}"
    );

    let mut padded = [0_u8; 8];
    padded[8 - bytes.len()..].copy_from_slice(bytes);
    Ok(u64::from_be_bytes(padded))
}
