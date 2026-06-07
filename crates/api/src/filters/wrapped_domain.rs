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
            owner_id: value.owner,
            owner_id_not: extras.owner_not,
            owner_id_gt: extras.owner_gt,
            owner_id_lt: extras.owner_lt,
            owner_id_gte: extras.owner_gte,
            owner_id_lte: extras.owner_lte,
            owner_id_in: extras.owner_in,
            owner_id_not_in: extras.owner_not_in,
            owner_id_contains: extras.owner_contains,
            owner_id_contains_nocase: extras.owner_contains_nocase,
            owner_id_not_contains: extras.owner_not_contains,
            owner_id_not_contains_nocase: extras.owner_not_contains_nocase,
            owner_id_starts_with: extras.owner_starts_with,
            owner_id_starts_with_nocase: extras.owner_starts_with_nocase,
            owner_id_not_starts_with: extras.owner_not_starts_with,
            owner_id_not_starts_with_nocase: extras.owner_not_starts_with_nocase,
            owner_id_ends_with: extras.owner_ends_with,
            owner_id_ends_with_nocase: extras.owner_ends_with_nocase,
            owner_id_not_ends_with: extras.owner_not_ends_with,
            owner_id_not_ends_with_nocase: extras.owner_not_ends_with_nocase,
            owner_filter: value.owner_filter.map(|filter| Box::new((*filter).into())),
            name: value.name,
            name_not: extras.name_not,
            name_gt: extras.name_gt,
            name_lt: extras.name_lt,
            name_gte: extras.name_gte,
            name_lte: extras.name_lte,
            name_in: extras.name_in,
            name_not_in: extras.name_not_in,
            name_contains: value.name_contains,
            name_contains_nocase: value.name_contains_nocase,
            name_not_contains: extras.name_not_contains,
            name_not_contains_nocase: extras.name_not_contains_nocase,
            name_starts_with: value.name_starts_with,
            name_starts_with_nocase: extras.name_starts_with_nocase,
            name_not_starts_with: extras.name_not_starts_with,
            name_not_starts_with_nocase: extras.name_not_starts_with_nocase,
            name_ends_with: value.name_ends_with,
            name_ends_with_nocase: extras.name_ends_with_nocase,
            name_not_ends_with: extras.name_not_ends_with,
            name_not_ends_with_nocase: extras.name_not_ends_with_nocase,
            expiry_date: value.expiry_date,
            expiry_date_not: extras.expiry_date_not,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            expiry_date_in: extras.expiry_date_in,
            expiry_date_not_in: extras.expiry_date_not_in,
            fuses: value.fuses,
            fuses_not: extras.fuses_not.and_then(|value| value.parse().ok()),
            fuses_gt: value.fuses_gt,
            fuses_lt: value.fuses_lt,
            fuses_gte: value.fuses_gte,
            fuses_lte: value.fuses_lte,
            fuses_in: extras.fuses_in,
            fuses_not_in: extras.fuses_not_in,
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
