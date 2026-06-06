use async_graphql::InputObject;
use storage::WrappedDomainFilter as StorageWrappedDomainFilter;

use super::{AccountFilter, DomainFilter, extras::WrappedDomainFilterExtras};

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "WrappedDomain_filter")]
pub struct WrappedDomainFilter {
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
    pub owner: Option<String>,
    #[graphql(name = "owner_")]
    pub owner_filter: Option<Box<AccountFilter>>,
    pub name: Option<String>,
    #[graphql(name = "name_contains")]
    pub name_contains: Option<String>,
    #[graphql(name = "name_contains_nocase")]
    pub name_contains_nocase: Option<String>,
    #[graphql(name = "name_starts_with")]
    pub name_starts_with: Option<String>,
    #[graphql(name = "name_ends_with")]
    pub name_ends_with: Option<String>,
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
    pub fuses: Option<i32>,
    #[graphql(name = "fuses_gt")]
    pub fuses_gt: Option<i32>,
    #[graphql(name = "fuses_lt")]
    pub fuses_lt: Option<i32>,
    #[graphql(name = "fuses_gte")]
    pub fuses_gte: Option<i32>,
    #[graphql(name = "fuses_lte")]
    pub fuses_lte: Option<i32>,
    #[graphql(flatten)]
    extras: WrappedDomainFilterExtras,
}

impl From<WrappedDomainFilter> for StorageWrappedDomainFilter {
    fn from(value: WrappedDomainFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            domain_id: value.domain,
            domain_filter: value.domain_filter.map(|filter| Box::new((*filter).into())),
            owner_id: value.owner,
            owner_filter: value.owner_filter.map(|filter| Box::new((*filter).into())),
            name: value.name,
            name_contains: value.name_contains,
            name_contains_nocase: value.name_contains_nocase,
            name_starts_with: value.name_starts_with,
            name_ends_with: value.name_ends_with,
            expiry_date: value.expiry_date,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            fuses: value.fuses,
            fuses_gt: value.fuses_gt,
            fuses_lt: value.fuses_lt,
            fuses_gte: value.fuses_gte,
            fuses_lte: value.fuses_lte,
        }
    }
}
