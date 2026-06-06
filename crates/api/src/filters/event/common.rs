use async_graphql::InputObject;
use storage::EventFilter as StorageEventFilter;

use crate::filters::BlockChangedFilter;

#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_gt: Option<String>,
    pub id_lt: Option<String>,
    pub id_gte: Option<String>,
    pub id_lte: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub parent_id: Option<String>,
    pub block_number: Option<i32>,
    pub block_number_not: Option<i32>,
    pub block_number_gt: Option<i32>,
    pub block_number_lt: Option<i32>,
    pub block_number_gte: Option<i32>,
    pub block_number_lte: Option<i32>,
    pub block_number_in: Option<Vec<i32>>,
    pub block_number_not_in: Option<Vec<i32>>,
    pub transaction_id: Option<String>,
    pub transaction_id_not: Option<String>,
    pub transaction_id_gt: Option<String>,
    pub transaction_id_lt: Option<String>,
    pub transaction_id_gte: Option<String>,
    pub transaction_id_lte: Option<String>,
    pub transaction_id_in: Option<Vec<String>>,
    pub transaction_id_not_in: Option<Vec<String>>,
    pub transaction_id_contains: Option<String>,
    pub transaction_id_not_contains: Option<String>,
    pub owner_id: Option<String>,
    pub parent_domain_id: Option<String>,
    pub resolver_id: Option<String>,
    pub registrant_id: Option<String>,
    pub new_owner_id: Option<String>,
    pub addr_id: Option<String>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
    pub name_contains_nocase: Option<String>,
    pub fuses: Option<i32>,
    pub fuses_gt: Option<i32>,
    pub fuses_lt: Option<i32>,
    pub fuses_gte: Option<i32>,
    pub fuses_lte: Option<i32>,
    pub ttl: Option<String>,
    pub ttl_gt: Option<String>,
    pub ttl_lt: Option<String>,
    pub ttl_gte: Option<String>,
    pub ttl_lte: Option<String>,
    pub expiry_date: Option<String>,
    pub expiry_date_gt: Option<String>,
    pub expiry_date_lt: Option<String>,
    pub expiry_date_gte: Option<String>,
    pub expiry_date_lte: Option<String>,
    pub coin_type: Option<String>,
    pub coin_type_gt: Option<String>,
    pub coin_type_lt: Option<String>,
    pub content_type: Option<String>,
    pub content_type_gt: Option<String>,
    pub content_type_lt: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
    pub key: Option<String>,
    pub key_contains: Option<String>,
    pub value: Option<String>,
    pub value_contains: Option<String>,
    pub hash: Option<String>,
    pub interface_id: Option<String>,
    pub implementer: Option<String>,
    pub target: Option<String>,
    pub is_authorized: Option<bool>,
    pub version: Option<String>,
    pub version_gt: Option<String>,
    pub version_lt: Option<String>,
}

impl EventFilter {
    pub(crate) fn into_domain_filter(self) -> StorageEventFilter {
        let parent_id = self.parent_id.clone();
        self.into_storage_filter(parent_id)
    }

    pub(crate) fn into_registration_filter(self) -> StorageEventFilter {
        let parent_id = self.parent_id.clone();
        self.into_storage_filter(parent_id)
    }

    pub(crate) fn into_resolver_filter(self) -> StorageEventFilter {
        let parent_id = self.parent_id.clone();
        self.into_storage_filter(parent_id)
    }

    fn into_storage_filter(self, parent_id: Option<String>) -> StorageEventFilter {
        StorageEventFilter {
            id: self.id,
            id_not: self.id_not,
            id_gt: self.id_gt,
            id_lt: self.id_lt,
            id_gte: self.id_gte,
            id_lte: self.id_lte,
            id_in: self.id_in,
            id_not_in: self.id_not_in,
            parent_id,
            block_number: self.block_number,
            block_number_not: self.block_number_not,
            block_number_gt: self.block_number_gt,
            block_number_lt: self.block_number_lt,
            block_number_gte: self.block_number_gte,
            block_number_lte: self.block_number_lte,
            block_number_in: self.block_number_in,
            block_number_not_in: self.block_number_not_in,
            transaction_id: self.transaction_id,
            transaction_id_not: self.transaction_id_not,
            transaction_id_gt: self.transaction_id_gt,
            transaction_id_lt: self.transaction_id_lt,
            transaction_id_gte: self.transaction_id_gte,
            transaction_id_lte: self.transaction_id_lte,
            transaction_id_in: self.transaction_id_in,
            transaction_id_not_in: self.transaction_id_not_in,
            transaction_id_contains: self.transaction_id_contains,
            transaction_id_not_contains: self.transaction_id_not_contains,
            owner_id: self.owner_id,
            parent_domain_id: self.parent_domain_id,
            resolver_id: self.resolver_id,
            registrant_id: self.registrant_id,
            new_owner_id: self.new_owner_id,
            addr_id: self.addr_id,
            name: self.name,
            name_contains: self.name_contains,
            name_contains_nocase: self.name_contains_nocase,
            fuses: self.fuses,
            fuses_gt: self.fuses_gt,
            fuses_lt: self.fuses_lt,
            fuses_gte: self.fuses_gte,
            fuses_lte: self.fuses_lte,
            ttl: self.ttl,
            ttl_gt: self.ttl_gt,
            ttl_lt: self.ttl_lt,
            ttl_gte: self.ttl_gte,
            ttl_lte: self.ttl_lte,
            expiry_date: self.expiry_date,
            expiry_date_gt: self.expiry_date_gt,
            expiry_date_lt: self.expiry_date_lt,
            expiry_date_gte: self.expiry_date_gte,
            expiry_date_lte: self.expiry_date_lte,
            coin_type: self.coin_type,
            coin_type_gt: self.coin_type_gt,
            coin_type_lt: self.coin_type_lt,
            content_type: self.content_type,
            content_type_gt: self.content_type_gt,
            content_type_lt: self.content_type_lt,
            x: self.x,
            y: self.y,
            key: self.key,
            key_contains: self.key_contains,
            value: self.value,
            value_contains: self.value_contains,
            hash: self.hash,
            interface_id: self.interface_id,
            implementer: self.implementer,
            target: self.target,
            is_authorized: self.is_authorized,
            version: self.version,
            version_gt: self.version_gt,
            version_lt: self.version_lt,
        }
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct BaseEventFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_gt")]
    pub id_gt: Option<String>,
    #[graphql(name = "id_lt")]
    pub id_lt: Option<String>,
    #[graphql(name = "id_gte")]
    pub id_gte: Option<String>,
    #[graphql(name = "id_lte")]
    pub id_lte: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    #[graphql(name = "blockNumber")]
    pub block_number: Option<i32>,
    #[graphql(name = "blockNumber_not")]
    pub block_number_not: Option<i32>,
    #[graphql(name = "blockNumber_gt")]
    pub block_number_gt: Option<i32>,
    #[graphql(name = "blockNumber_lt")]
    pub block_number_lt: Option<i32>,
    #[graphql(name = "blockNumber_gte")]
    pub block_number_gte: Option<i32>,
    #[graphql(name = "blockNumber_lte")]
    pub block_number_lte: Option<i32>,
    #[graphql(name = "blockNumber_in")]
    pub block_number_in: Option<Vec<i32>>,
    #[graphql(name = "blockNumber_not_in")]
    pub block_number_not_in: Option<Vec<i32>>,
    #[graphql(name = "transactionID")]
    pub transaction_id: Option<String>,
    #[graphql(name = "transactionID_not")]
    pub transaction_id_not: Option<String>,
    #[graphql(name = "transactionID_gt")]
    pub transaction_id_gt: Option<String>,
    #[graphql(name = "transactionID_lt")]
    pub transaction_id_lt: Option<String>,
    #[graphql(name = "transactionID_gte")]
    pub transaction_id_gte: Option<String>,
    #[graphql(name = "transactionID_lte")]
    pub transaction_id_lte: Option<String>,
    #[graphql(name = "transactionID_in")]
    pub transaction_id_in: Option<Vec<String>>,
    #[graphql(name = "transactionID_not_in")]
    pub transaction_id_not_in: Option<Vec<String>>,
    #[graphql(name = "transactionID_contains")]
    pub transaction_id_contains: Option<String>,
    #[graphql(name = "transactionID_not_contains")]
    pub transaction_id_not_contains: Option<String>,
    #[graphql(name = "_change_block")]
    pub change_block: Option<BlockChangedFilter>,
}

impl BaseEventFilter {
    pub(crate) fn apply(self, filter: &mut EventFilter) {
        filter.id = self.id;
        filter.id_not = self.id_not;
        filter.id_gt = self.id_gt;
        filter.id_lt = self.id_lt;
        filter.id_gte = self.id_gte;
        filter.id_lte = self.id_lte;
        filter.id_in = self.id_in;
        filter.id_not_in = self.id_not_in;
        filter.block_number = self.block_number;
        filter.block_number_not = self.block_number_not;
        filter.block_number_gt = self.block_number_gt;
        filter.block_number_lt = self.block_number_lt;
        filter.block_number_gte = self.block_number_gte;
        filter.block_number_lte = self.block_number_lte;
        filter.block_number_in = self.block_number_in;
        filter.block_number_not_in = self.block_number_not_in;
        filter.transaction_id = self.transaction_id;
        filter.transaction_id_not = self.transaction_id_not;
        filter.transaction_id_gt = self.transaction_id_gt;
        filter.transaction_id_lt = self.transaction_id_lt;
        filter.transaction_id_gte = self.transaction_id_gte;
        filter.transaction_id_lte = self.transaction_id_lte;
        filter.transaction_id_in = self.transaction_id_in;
        filter.transaction_id_not_in = self.transaction_id_not_in;
        filter.transaction_id_contains = self.transaction_id_contains;
        filter.transaction_id_not_contains = self.transaction_id_not_contains;
    }
}

pub(crate) trait ApplyEventFilter {
    fn apply(self, filter: &mut EventFilter);
}
