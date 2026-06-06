use async_graphql::InputObject;
use storage::DomainFilter as StorageDomainFilter;

use super::{AccountFilter, ResolverFilter, extras::DomainFilterExtras};

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "Domain_filter")]
pub struct DomainFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    pub name: Option<String>,
    #[graphql(name = "name_contains")]
    pub name_contains: Option<String>,
    #[graphql(name = "name_contains_nocase")]
    pub name_contains_nocase: Option<String>,
    #[graphql(name = "name_starts_with")]
    pub name_starts_with: Option<String>,
    #[graphql(name = "name_ends_with")]
    pub name_ends_with: Option<String>,
    #[graphql(name = "labelName")]
    pub label_name: Option<String>,
    #[graphql(name = "labelName_contains")]
    pub label_name_contains: Option<String>,
    #[graphql(name = "labelName_contains_nocase")]
    pub label_name_contains_nocase: Option<String>,
    #[graphql(name = "labelName_starts_with")]
    pub label_name_starts_with: Option<String>,
    #[graphql(name = "labelName_ends_with")]
    pub label_name_ends_with: Option<String>,
    pub labelhash: Option<String>,
    #[graphql(name = "labelhash_not")]
    pub labelhash_not: Option<String>,
    #[graphql(name = "labelhash_in")]
    pub labelhash_in: Option<Vec<String>>,
    #[graphql(name = "labelhash_not_in")]
    pub labelhash_not_in: Option<Vec<String>>,
    pub parent: Option<String>,
    #[graphql(name = "parent_")]
    pub parent_filter: Option<Box<DomainFilter>>,
    #[graphql(name = "subdomainCount")]
    pub subdomain_count: Option<i32>,
    #[graphql(name = "subdomainCount_gt")]
    pub subdomain_count_gt: Option<i32>,
    #[graphql(name = "subdomainCount_lt")]
    pub subdomain_count_lt: Option<i32>,
    #[graphql(name = "subdomainCount_gte")]
    pub subdomain_count_gte: Option<i32>,
    #[graphql(name = "subdomainCount_lte")]
    pub subdomain_count_lte: Option<i32>,
    #[graphql(name = "resolvedAddress")]
    pub resolved_address: Option<String>,
    #[graphql(name = "resolvedAddress_")]
    pub resolved_address_filter: Option<Box<AccountFilter>>,
    pub owner: Option<String>,
    #[graphql(name = "owner_")]
    pub owner_filter: Option<Box<AccountFilter>>,
    pub resolver: Option<String>,
    #[graphql(name = "resolver_")]
    pub resolver_filter: Option<Box<ResolverFilter>>,
    pub registrant: Option<String>,
    #[graphql(name = "registrant_")]
    pub registrant_filter: Option<Box<AccountFilter>>,
    #[graphql(name = "wrappedOwner")]
    pub wrapped_owner: Option<String>,
    #[graphql(name = "wrappedOwner_")]
    pub wrapped_owner_filter: Option<Box<AccountFilter>>,
    #[graphql(name = "isMigrated")]
    pub is_migrated: Option<bool>,
    #[graphql(name = "isMigrated_not")]
    pub is_migrated_not: Option<bool>,
    #[graphql(name = "createdAt")]
    pub created_at: Option<String>,
    #[graphql(name = "createdAt_gt")]
    pub created_at_gt: Option<String>,
    #[graphql(name = "createdAt_lt")]
    pub created_at_lt: Option<String>,
    #[graphql(name = "createdAt_gte")]
    pub created_at_gte: Option<String>,
    #[graphql(name = "createdAt_lte")]
    pub created_at_lte: Option<String>,
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
    pub ttl: Option<String>,
    #[graphql(name = "ttl_gt")]
    pub ttl_gt: Option<String>,
    #[graphql(name = "ttl_lt")]
    pub ttl_lt: Option<String>,
    #[graphql(name = "ttl_gte")]
    pub ttl_gte: Option<String>,
    #[graphql(name = "ttl_lte")]
    pub ttl_lte: Option<String>,
    #[graphql(flatten)]
    extras: DomainFilterExtras,
}

impl From<DomainFilter> for StorageDomainFilter {
    fn from(value: DomainFilter) -> Self {
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
            label_name: value.label_name,
            label_name_not: extras.label_name_not,
            label_name_gt: extras.label_name_gt,
            label_name_lt: extras.label_name_lt,
            label_name_gte: extras.label_name_gte,
            label_name_lte: extras.label_name_lte,
            label_name_in: extras.label_name_in,
            label_name_not_in: extras.label_name_not_in,
            label_name_contains: value.label_name_contains,
            label_name_contains_nocase: value.label_name_contains_nocase,
            label_name_not_contains: extras.label_name_not_contains,
            label_name_not_contains_nocase: extras.label_name_not_contains_nocase,
            label_name_starts_with: value.label_name_starts_with,
            label_name_starts_with_nocase: extras.label_name_starts_with_nocase,
            label_name_not_starts_with: extras.label_name_not_starts_with,
            label_name_not_starts_with_nocase: extras.label_name_not_starts_with_nocase,
            label_name_ends_with: value.label_name_ends_with,
            label_name_ends_with_nocase: extras.label_name_ends_with_nocase,
            label_name_not_ends_with: extras.label_name_not_ends_with,
            label_name_not_ends_with_nocase: extras.label_name_not_ends_with_nocase,
            labelhash: value.labelhash,
            labelhash_not: value.labelhash_not,
            labelhash_gt: extras.labelhash_gt,
            labelhash_lt: extras.labelhash_lt,
            labelhash_gte: extras.labelhash_gte,
            labelhash_lte: extras.labelhash_lte,
            labelhash_in: value.labelhash_in,
            labelhash_not_in: value.labelhash_not_in,
            labelhash_contains: extras.labelhash_contains,
            labelhash_not_contains: extras.labelhash_not_contains,
            parent_id: value.parent,
            parent_id_not: extras.parent_not,
            parent_id_gt: extras.parent_gt,
            parent_id_lt: extras.parent_lt,
            parent_id_gte: extras.parent_gte,
            parent_id_lte: extras.parent_lte,
            parent_id_in: extras.parent_in,
            parent_id_not_in: extras.parent_not_in,
            parent_id_contains: extras.parent_contains,
            parent_id_contains_nocase: extras.parent_contains_nocase,
            parent_id_not_contains: extras.parent_not_contains,
            parent_id_not_contains_nocase: extras.parent_not_contains_nocase,
            parent_id_starts_with: extras.parent_starts_with,
            parent_id_starts_with_nocase: extras.parent_starts_with_nocase,
            parent_id_not_starts_with: extras.parent_not_starts_with,
            parent_id_not_starts_with_nocase: extras.parent_not_starts_with_nocase,
            parent_id_ends_with: extras.parent_ends_with,
            parent_id_ends_with_nocase: extras.parent_ends_with_nocase,
            parent_id_not_ends_with: extras.parent_not_ends_with,
            parent_id_not_ends_with_nocase: extras.parent_not_ends_with_nocase,
            parent_filter: value.parent_filter.map(|filter| Box::new((*filter).into())),
            subdomain_count: value.subdomain_count,
            subdomain_count_not: extras
                .subdomain_count_not
                .and_then(|value| value.parse().ok()),
            subdomain_count_gt: value.subdomain_count_gt,
            subdomain_count_lt: value.subdomain_count_lt,
            subdomain_count_gte: value.subdomain_count_gte,
            subdomain_count_lte: value.subdomain_count_lte,
            subdomain_count_in: extras.subdomain_count_in,
            subdomain_count_not_in: extras.subdomain_count_not_in,
            resolved_address_id: value.resolved_address,
            resolved_address_id_not: extras.resolved_address_not,
            resolved_address_id_gt: extras.resolved_address_gt,
            resolved_address_id_lt: extras.resolved_address_lt,
            resolved_address_id_gte: extras.resolved_address_gte,
            resolved_address_id_lte: extras.resolved_address_lte,
            resolved_address_id_in: extras.resolved_address_in,
            resolved_address_id_not_in: extras.resolved_address_not_in,
            resolved_address_id_contains: extras.resolved_address_contains,
            resolved_address_id_contains_nocase: extras.resolved_address_contains_nocase,
            resolved_address_id_not_contains: extras.resolved_address_not_contains,
            resolved_address_id_not_contains_nocase: extras.resolved_address_not_contains_nocase,
            resolved_address_id_starts_with: extras.resolved_address_starts_with,
            resolved_address_id_starts_with_nocase: extras.resolved_address_starts_with_nocase,
            resolved_address_id_not_starts_with: extras.resolved_address_not_starts_with,
            resolved_address_id_not_starts_with_nocase: extras
                .resolved_address_not_starts_with_nocase,
            resolved_address_id_ends_with: extras.resolved_address_ends_with,
            resolved_address_id_ends_with_nocase: extras.resolved_address_ends_with_nocase,
            resolved_address_id_not_ends_with: extras.resolved_address_not_ends_with,
            resolved_address_id_not_ends_with_nocase: extras.resolved_address_not_ends_with_nocase,
            resolved_address_filter: value
                .resolved_address_filter
                .map(|filter| Box::new((*filter).into())),
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
            resolver_id: value.resolver,
            resolver_id_not: extras.resolver_not,
            resolver_id_gt: extras.resolver_gt,
            resolver_id_lt: extras.resolver_lt,
            resolver_id_gte: extras.resolver_gte,
            resolver_id_lte: extras.resolver_lte,
            resolver_id_in: extras.resolver_in,
            resolver_id_not_in: extras.resolver_not_in,
            resolver_id_contains: extras.resolver_contains,
            resolver_id_contains_nocase: extras.resolver_contains_nocase,
            resolver_id_not_contains: extras.resolver_not_contains,
            resolver_id_not_contains_nocase: extras.resolver_not_contains_nocase,
            resolver_id_starts_with: extras.resolver_starts_with,
            resolver_id_starts_with_nocase: extras.resolver_starts_with_nocase,
            resolver_id_not_starts_with: extras.resolver_not_starts_with,
            resolver_id_not_starts_with_nocase: extras.resolver_not_starts_with_nocase,
            resolver_id_ends_with: extras.resolver_ends_with,
            resolver_id_ends_with_nocase: extras.resolver_ends_with_nocase,
            resolver_id_not_ends_with: extras.resolver_not_ends_with,
            resolver_id_not_ends_with_nocase: extras.resolver_not_ends_with_nocase,
            resolver_filter: value
                .resolver_filter
                .map(|filter| Box::new((*filter).into())),
            registrant_id: value.registrant,
            registrant_id_not: extras.registrant_not,
            registrant_id_gt: extras.registrant_gt,
            registrant_id_lt: extras.registrant_lt,
            registrant_id_gte: extras.registrant_gte,
            registrant_id_lte: extras.registrant_lte,
            registrant_id_in: extras.registrant_in,
            registrant_id_not_in: extras.registrant_not_in,
            registrant_id_contains: extras.registrant_contains,
            registrant_id_contains_nocase: extras.registrant_contains_nocase,
            registrant_id_not_contains: extras.registrant_not_contains,
            registrant_id_not_contains_nocase: extras.registrant_not_contains_nocase,
            registrant_id_starts_with: extras.registrant_starts_with,
            registrant_id_starts_with_nocase: extras.registrant_starts_with_nocase,
            registrant_id_not_starts_with: extras.registrant_not_starts_with,
            registrant_id_not_starts_with_nocase: extras.registrant_not_starts_with_nocase,
            registrant_id_ends_with: extras.registrant_ends_with,
            registrant_id_ends_with_nocase: extras.registrant_ends_with_nocase,
            registrant_id_not_ends_with: extras.registrant_not_ends_with,
            registrant_id_not_ends_with_nocase: extras.registrant_not_ends_with_nocase,
            registrant_filter: value
                .registrant_filter
                .map(|filter| Box::new((*filter).into())),
            wrapped_owner_id: value.wrapped_owner,
            wrapped_owner_id_not: extras.wrapped_owner_not,
            wrapped_owner_id_gt: extras.wrapped_owner_gt,
            wrapped_owner_id_lt: extras.wrapped_owner_lt,
            wrapped_owner_id_gte: extras.wrapped_owner_gte,
            wrapped_owner_id_lte: extras.wrapped_owner_lte,
            wrapped_owner_id_in: extras.wrapped_owner_in,
            wrapped_owner_id_not_in: extras.wrapped_owner_not_in,
            wrapped_owner_id_contains: extras.wrapped_owner_contains,
            wrapped_owner_id_contains_nocase: extras.wrapped_owner_contains_nocase,
            wrapped_owner_id_not_contains: extras.wrapped_owner_not_contains,
            wrapped_owner_id_not_contains_nocase: extras.wrapped_owner_not_contains_nocase,
            wrapped_owner_id_starts_with: extras.wrapped_owner_starts_with,
            wrapped_owner_id_starts_with_nocase: extras.wrapped_owner_starts_with_nocase,
            wrapped_owner_id_not_starts_with: extras.wrapped_owner_not_starts_with,
            wrapped_owner_id_not_starts_with_nocase: extras.wrapped_owner_not_starts_with_nocase,
            wrapped_owner_id_ends_with: extras.wrapped_owner_ends_with,
            wrapped_owner_id_ends_with_nocase: extras.wrapped_owner_ends_with_nocase,
            wrapped_owner_id_not_ends_with: extras.wrapped_owner_not_ends_with,
            wrapped_owner_id_not_ends_with_nocase: extras.wrapped_owner_not_ends_with_nocase,
            wrapped_owner_filter: value
                .wrapped_owner_filter
                .map(|filter| Box::new((*filter).into())),
            is_migrated: value.is_migrated,
            is_migrated_not: value.is_migrated_not,
            is_migrated_in: extras.is_migrated_in,
            is_migrated_not_in: extras.is_migrated_not_in,
            created_at: value.created_at,
            created_at_not: extras.created_at_not,
            created_at_gt: value.created_at_gt,
            created_at_lt: value.created_at_lt,
            created_at_gte: value.created_at_gte,
            created_at_lte: value.created_at_lte,
            created_at_in: extras.created_at_in,
            created_at_not_in: extras.created_at_not_in,
            expiry_date: value.expiry_date,
            expiry_date_not: extras.expiry_date_not,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            expiry_date_in: extras.expiry_date_in,
            expiry_date_not_in: extras.expiry_date_not_in,
            ttl: value.ttl,
            ttl_not: extras.ttl_not,
            ttl_gt: value.ttl_gt,
            ttl_lt: value.ttl_lt,
            ttl_gte: value.ttl_gte,
            ttl_lte: value.ttl_lte,
            ttl_in: extras.ttl_in,
            ttl_not_in: extras.ttl_not_in,
        }
    }
}
