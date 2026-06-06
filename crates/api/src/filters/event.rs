use async_graphql::InputObject;
use storage::EventFilter as StorageEventFilter;

#[derive(Debug, Clone, InputObject, Default)]
pub struct EventFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    pub domain: Option<String>,
    pub registration: Option<String>,
    pub resolver: Option<String>,
    #[graphql(name = "blockNumber")]
    pub block_number: Option<i32>,
    #[graphql(name = "blockNumber_gt")]
    pub block_number_gt: Option<i32>,
    #[graphql(name = "blockNumber_lt")]
    pub block_number_lt: Option<i32>,
    #[graphql(name = "blockNumber_gte")]
    pub block_number_gte: Option<i32>,
    #[graphql(name = "blockNumber_lte")]
    pub block_number_lte: Option<i32>,
    #[graphql(name = "transactionID")]
    pub transaction_id: Option<String>,
    #[graphql(name = "transactionID_not")]
    pub transaction_id_not: Option<String>,
    #[graphql(name = "transactionID_in")]
    pub transaction_id_in: Option<Vec<String>>,
    #[graphql(name = "transactionID_not_in")]
    pub transaction_id_not_in: Option<Vec<String>>,
    pub owner: Option<String>,
    #[graphql(name = "parentDomain")]
    pub parent_domain: Option<String>,
    #[graphql(name = "newOwner")]
    pub new_owner: Option<String>,
    pub registrant: Option<String>,
    pub addr: Option<String>,
    pub name: Option<String>,
    #[graphql(name = "name_contains")]
    pub name_contains: Option<String>,
    #[graphql(name = "name_contains_nocase")]
    pub name_contains_nocase: Option<String>,
    pub fuses: Option<i32>,
    #[graphql(name = "fuses_gt")]
    pub fuses_gt: Option<i32>,
    #[graphql(name = "fuses_lt")]
    pub fuses_lt: Option<i32>,
    #[graphql(name = "fuses_gte")]
    pub fuses_gte: Option<i32>,
    #[graphql(name = "fuses_lte")]
    pub fuses_lte: Option<i32>,
    pub ttl: Option<String>,
    #[graphql(name = "ttl_gt")]
    pub ttl_gt: Option<String>,
    #[graphql(name = "ttl_lt")]
    pub ttl_lt: Option<String>,
    #[graphql(name = "ttl_gte")]
    pub ttl_gte: Option<String>,
    #[graphql(name = "ttl_lte")]
    pub ttl_lte: Option<String>,
    #[graphql(name = "expiryDate")]
    pub expiry_date: Option<String>,
    #[graphql(name = "expiryDate_gt")]
    pub expiry_date_gt: Option<String>,
    #[graphql(name = "expiryDate_lt")]
    pub expiry_date_lt: Option<String>,
    #[graphql(name = "expiryDate_gte")]
    pub expiry_date_gte: Option<String>,
    #[graphql(name = "expiryDate_lte")]
    pub expiry_date_lte: Option<String>,
    #[graphql(name = "coinType")]
    pub coin_type: Option<String>,
    #[graphql(name = "coinType_gt")]
    pub coin_type_gt: Option<String>,
    #[graphql(name = "coinType_lt")]
    pub coin_type_lt: Option<String>,
    #[graphql(name = "contentType")]
    pub content_type: Option<String>,
    #[graphql(name = "contentType_gt")]
    pub content_type_gt: Option<String>,
    #[graphql(name = "contentType_lt")]
    pub content_type_lt: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
    pub key: Option<String>,
    #[graphql(name = "key_contains")]
    pub key_contains: Option<String>,
    pub value: Option<String>,
    #[graphql(name = "value_contains")]
    pub value_contains: Option<String>,
    pub hash: Option<String>,
    #[graphql(name = "interfaceID")]
    pub interface_id: Option<String>,
    pub implementer: Option<String>,
    pub target: Option<String>,
    #[graphql(name = "isAuthorized")]
    pub is_authorized: Option<bool>,
    pub version: Option<String>,
    #[graphql(name = "version_gt")]
    pub version_gt: Option<String>,
    #[graphql(name = "version_lt")]
    pub version_lt: Option<String>,
}

impl EventFilter {
    pub(crate) fn into_domain_filter(self) -> StorageEventFilter {
        let parent_id = self.domain.clone();
        self.into_storage_filter(parent_id)
    }

    pub(crate) fn into_registration_filter(self) -> StorageEventFilter {
        let parent_id = self.registration.clone();
        self.into_storage_filter(parent_id)
    }

    pub(crate) fn into_resolver_filter(self) -> StorageEventFilter {
        let parent_id = self.resolver.clone();
        self.into_storage_filter(parent_id)
    }

    fn into_storage_filter(self, parent_id: Option<String>) -> StorageEventFilter {
        StorageEventFilter {
            id: self.id,
            id_not: self.id_not,
            id_in: self.id_in,
            id_not_in: self.id_not_in,
            parent_id,
            block_number: self.block_number,
            block_number_gt: self.block_number_gt,
            block_number_lt: self.block_number_lt,
            block_number_gte: self.block_number_gte,
            block_number_lte: self.block_number_lte,
            transaction_id: self.transaction_id,
            transaction_id_not: self.transaction_id_not,
            transaction_id_in: self.transaction_id_in,
            transaction_id_not_in: self.transaction_id_not_in,
            owner_id: self.owner,
            parent_domain_id: self.parent_domain,
            resolver_id: self.resolver,
            registrant_id: self.registrant,
            new_owner_id: self.new_owner,
            addr_id: self.addr,
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
