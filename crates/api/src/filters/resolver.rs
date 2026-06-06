use async_graphql::InputObject;
use storage::ResolverFilter as StorageResolverFilter;

use super::{AccountFilter, DomainFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub struct ResolverFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    pub domain: Option<String>,
    #[graphql(name = "domain_")]
    pub domain_filter: Option<Box<DomainFilter>>,
    pub address: Option<String>,
    #[graphql(name = "address_in")]
    pub address_in: Option<Vec<String>>,
    pub addr: Option<String>,
    #[graphql(name = "addr_")]
    pub addr_filter: Option<Box<AccountFilter>>,
    #[graphql(name = "contentHash")]
    pub content_hash: Option<String>,
    #[graphql(name = "contentHash_not")]
    pub content_hash_not: Option<String>,
    #[graphql(name = "contentHash_in")]
    pub content_hash_in: Option<Vec<String>>,
    #[graphql(name = "contentHash_not_in")]
    pub content_hash_not_in: Option<Vec<String>>,
    #[graphql(name = "contentHash_contains")]
    pub content_hash_contains: Option<String>,
    #[graphql(name = "texts_contains")]
    pub texts_contains: Option<String>,
    #[graphql(name = "coinTypes_contains")]
    pub coin_types_contains: Option<String>,
}

impl From<ResolverFilter> for StorageResolverFilter {
    fn from(value: ResolverFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            domain_id: value.domain,
            domain_filter: value.domain_filter.map(|filter| Box::new((*filter).into())),
            address: value.address,
            address_in: value.address_in,
            addr_id: value.addr,
            addr_filter: value.addr_filter.map(|filter| Box::new((*filter).into())),
            content_hash: value.content_hash,
            content_hash_not: value.content_hash_not,
            content_hash_in: value.content_hash_in,
            content_hash_not_in: value.content_hash_not_in,
            content_hash_contains: value.content_hash_contains,
            texts_contains: value.texts_contains,
            coin_types_contains: value.coin_types_contains,
        }
    }
}
