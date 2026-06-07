use async_graphql::InputObject;
use storage::ResolverFilter as StorageResolverFilter;

use super::{AccountFilter, DomainFilter, extras::ResolverFilterExtras};

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "Resolver_filter")]
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
    #[graphql(flatten)]
    extras: ResolverFilterExtras,
}

impl From<ResolverFilter> for StorageResolverFilter {
    fn from(value: ResolverFilter) -> Self {
        let extras = value.extras;
        Self {
            id: value.id,
            id_not: value.id_not,
            id_gt: extras.id_gt,
            id_lt: extras.id_lt,
            id_gte: extras.id_gte,
            id_lte: extras.id_lte,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            domain_id: value.domain,
            domain_id_not: extras.domain_not,
            domain_id_gt: extras.domain_gt,
            domain_id_lt: extras.domain_lt,
            domain_id_gte: extras.domain_gte,
            domain_id_lte: extras.domain_lte,
            domain_id_in: extras.domain_in,
            domain_id_not_in: extras.domain_not_in,
            domain_id_contains: extras.domain_contains,
            domain_id_contains_nocase: extras.domain_contains_nocase,
            domain_id_not_contains: extras.domain_not_contains,
            domain_id_not_contains_nocase: extras.domain_not_contains_nocase,
            domain_id_starts_with: extras.domain_starts_with,
            domain_id_starts_with_nocase: extras.domain_starts_with_nocase,
            domain_id_not_starts_with: extras.domain_not_starts_with,
            domain_id_not_starts_with_nocase: extras.domain_not_starts_with_nocase,
            domain_id_ends_with: extras.domain_ends_with,
            domain_id_ends_with_nocase: extras.domain_ends_with_nocase,
            domain_id_not_ends_with: extras.domain_not_ends_with,
            domain_id_not_ends_with_nocase: extras.domain_not_ends_with_nocase,
            domain_filter: value.domain_filter.map(|filter| Box::new((*filter).into())),
            address: value.address,
            address_not: extras.address_not,
            address_gt: extras.address_gt,
            address_lt: extras.address_lt,
            address_gte: extras.address_gte,
            address_lte: extras.address_lte,
            address_in: value.address_in,
            address_not_in: extras.address_not_in,
            address_contains: extras.address_contains,
            address_not_contains: extras.address_not_contains,
            addr_id: value.addr,
            addr_id_not: extras.addr_not,
            addr_id_gt: extras.addr_gt,
            addr_id_lt: extras.addr_lt,
            addr_id_gte: extras.addr_gte,
            addr_id_lte: extras.addr_lte,
            addr_id_in: extras.addr_in,
            addr_id_not_in: extras.addr_not_in,
            addr_id_contains: extras.addr_contains,
            addr_id_contains_nocase: extras.addr_contains_nocase,
            addr_id_not_contains: extras.addr_not_contains,
            addr_id_not_contains_nocase: extras.addr_not_contains_nocase,
            addr_id_starts_with: extras.addr_starts_with,
            addr_id_starts_with_nocase: extras.addr_starts_with_nocase,
            addr_id_not_starts_with: extras.addr_not_starts_with,
            addr_id_not_starts_with_nocase: extras.addr_not_starts_with_nocase,
            addr_id_ends_with: extras.addr_ends_with,
            addr_id_ends_with_nocase: extras.addr_ends_with_nocase,
            addr_id_not_ends_with: extras.addr_not_ends_with,
            addr_id_not_ends_with_nocase: extras.addr_not_ends_with_nocase,
            addr_filter: value.addr_filter.map(|filter| Box::new((*filter).into())),
            content_hash: value.content_hash,
            content_hash_not: value.content_hash_not,
            content_hash_gt: extras.content_hash_gt,
            content_hash_lt: extras.content_hash_lt,
            content_hash_gte: extras.content_hash_gte,
            content_hash_lte: extras.content_hash_lte,
            content_hash_in: value.content_hash_in,
            content_hash_not_in: value.content_hash_not_in,
            content_hash_contains: value.content_hash_contains,
            content_hash_not_contains: extras.content_hash_not_contains,
            texts: extras.texts,
            texts_not: extras.texts_not,
            texts_contains: value.texts_contains,
            texts_contains_nocase: extras.texts_contains_nocase,
            texts_not_contains: extras.texts_not_contains,
            texts_not_contains_nocase: extras.texts_not_contains_nocase,
            coin_types: extras.coin_types,
            coin_types_not: extras.coin_types_not,
            coin_types_contains: value.coin_types_contains,
            coin_types_contains_nocase: extras.coin_types_contains_nocase,
            coin_types_not_contains: extras.coin_types_not_contains,
            coin_types_not_contains_nocase: extras.coin_types_not_contains_nocase,
            events_filter: extras.events_.map(|filter| {
                let filter: super::EventFilter = (*filter).into();
                Box::new(filter.into_resolver_filter())
            }),
            change_block_number_gte: extras.change_block.and_then(|change| change.number_gte),
            and: extras
                .and
                .map(|filters| filters.into_iter().map(Into::into).collect()),
            or: extras
                .or
                .map(|filters| filters.into_iter().map(Into::into).collect()),
        }
    }
}
