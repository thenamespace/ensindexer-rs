use async_graphql::{Enum, InputObject};
use storage::{
    AccountFilter as StorageAccountFilter, AccountOrderField, DomainFilter as StorageDomainFilter,
    DomainOrderField, EventFilter as StorageEventFilter, EventOrderField,
    OrderDirection as StorageOrderDirection, RegistrationFilter as StorageRegistrationFilter,
    RegistrationOrderField, ResolverFilter as StorageResolverFilter, ResolverOrderField,
    WrappedDomainFilter as StorageWrappedDomainFilter, WrappedDomainOrderField,
};

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum OrderDirection {
    #[default]
    Asc,
    Desc,
}

impl From<OrderDirection> for StorageOrderDirection {
    fn from(value: OrderDirection) -> Self {
        match value {
            OrderDirection::Asc => Self::Asc,
            OrderDirection::Desc => Self::Desc,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum AccountOrderBy {
    #[default]
    Id,
}

impl From<AccountOrderBy> for AccountOrderField {
    fn from(value: AccountOrderBy) -> Self {
        match value {
            AccountOrderBy::Id => Self::Id,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum DomainOrderBy {
    #[default]
    Id,
    Name,
    #[graphql(name = "labelName")]
    LabelName,
    #[graphql(name = "subdomainCount")]
    SubdomainCount,
    #[graphql(name = "createdAt")]
    CreatedAt,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
}

impl From<DomainOrderBy> for DomainOrderField {
    fn from(value: DomainOrderBy) -> Self {
        match value {
            DomainOrderBy::Id => Self::Id,
            DomainOrderBy::Name => Self::Name,
            DomainOrderBy::LabelName => Self::LabelName,
            DomainOrderBy::SubdomainCount => Self::SubdomainCount,
            DomainOrderBy::CreatedAt => Self::CreatedAt,
            DomainOrderBy::ExpiryDate => Self::ExpiryDate,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum RegistrationOrderBy {
    #[default]
    Id,
    #[graphql(name = "registrationDate")]
    RegistrationDate,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    Cost,
    #[graphql(name = "labelName")]
    LabelName,
}

impl From<RegistrationOrderBy> for RegistrationOrderField {
    fn from(value: RegistrationOrderBy) -> Self {
        match value {
            RegistrationOrderBy::Id => Self::Id,
            RegistrationOrderBy::RegistrationDate => Self::RegistrationDate,
            RegistrationOrderBy::ExpiryDate => Self::ExpiryDate,
            RegistrationOrderBy::Cost => Self::Cost,
            RegistrationOrderBy::LabelName => Self::LabelName,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum WrappedDomainOrderBy {
    #[default]
    Id,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    Fuses,
    Name,
}

impl From<WrappedDomainOrderBy> for WrappedDomainOrderField {
    fn from(value: WrappedDomainOrderBy) -> Self {
        match value {
            WrappedDomainOrderBy::Id => Self::Id,
            WrappedDomainOrderBy::ExpiryDate => Self::ExpiryDate,
            WrappedDomainOrderBy::Fuses => Self::Fuses,
            WrappedDomainOrderBy::Name => Self::Name,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum ResolverOrderBy {
    #[default]
    Id,
    Address,
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
pub enum EventOrderBy {
    #[default]
    Id,
    #[graphql(name = "blockNumber")]
    BlockNumber,
}

impl From<EventOrderBy> for EventOrderField {
    fn from(value: EventOrderBy) -> Self {
        match value {
            EventOrderBy::Id => Self::Id,
            EventOrderBy::BlockNumber => Self::BlockNumber,
        }
    }
}

impl From<ResolverOrderBy> for ResolverOrderField {
    fn from(value: ResolverOrderBy) -> Self {
        match value {
            ResolverOrderBy::Id => Self::Id,
            ResolverOrderBy::Address => Self::Address,
        }
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub struct AccountFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    pub and: Option<Vec<AccountFilter>>,
    pub or: Option<Vec<AccountFilter>>,
}

impl From<AccountFilter> for StorageAccountFilter {
    fn from(value: AccountFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            and: value
                .and
                .map(|filters| filters.into_iter().map(Into::into).collect()),
            or: value
                .or
                .map(|filters| filters.into_iter().map(Into::into).collect()),
        }
    }
}

#[derive(Debug, Clone, InputObject, Default)]
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
}

impl From<DomainFilter> for StorageDomainFilter {
    fn from(value: DomainFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            name: value.name,
            name_contains: value.name_contains,
            name_contains_nocase: value.name_contains_nocase,
            name_starts_with: value.name_starts_with,
            name_ends_with: value.name_ends_with,
            label_name: value.label_name,
            label_name_contains: value.label_name_contains,
            label_name_contains_nocase: value.label_name_contains_nocase,
            label_name_starts_with: value.label_name_starts_with,
            label_name_ends_with: value.label_name_ends_with,
            labelhash: value.labelhash,
            labelhash_not: value.labelhash_not,
            labelhash_in: value.labelhash_in,
            labelhash_not_in: value.labelhash_not_in,
            parent_id: value.parent,
            parent_filter: value.parent_filter.map(|filter| Box::new((*filter).into())),
            subdomain_count: value.subdomain_count,
            subdomain_count_gt: value.subdomain_count_gt,
            subdomain_count_lt: value.subdomain_count_lt,
            subdomain_count_gte: value.subdomain_count_gte,
            subdomain_count_lte: value.subdomain_count_lte,
            resolved_address_id: value.resolved_address,
            resolved_address_filter: value
                .resolved_address_filter
                .map(|filter| Box::new((*filter).into())),
            owner_id: value.owner,
            owner_filter: value.owner_filter.map(|filter| Box::new((*filter).into())),
            resolver_id: value.resolver,
            resolver_filter: value
                .resolver_filter
                .map(|filter| Box::new((*filter).into())),
            registrant_id: value.registrant,
            registrant_filter: value
                .registrant_filter
                .map(|filter| Box::new((*filter).into())),
            wrapped_owner_id: value.wrapped_owner,
            wrapped_owner_filter: value
                .wrapped_owner_filter
                .map(|filter| Box::new((*filter).into())),
            is_migrated: value.is_migrated,
            is_migrated_not: value.is_migrated_not,
            created_at: value.created_at,
            created_at_gt: value.created_at_gt,
            created_at_lt: value.created_at_lt,
            created_at_gte: value.created_at_gte,
            created_at_lte: value.created_at_lte,
            expiry_date: value.expiry_date,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            ttl: value.ttl,
            ttl_gt: value.ttl_gt,
            ttl_lt: value.ttl_lt,
            ttl_gte: value.ttl_gte,
            ttl_lte: value.ttl_lte,
        }
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub struct RegistrationFilter {
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
    pub registrant: Option<String>,
    #[graphql(name = "registrant_")]
    pub registrant_filter: Option<Box<AccountFilter>>,
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
    #[graphql(name = "registrationDate")]
    pub registration_date: Option<String>,
    #[graphql(name = "registrationDate_gt")]
    pub registration_date_gt: Option<String>,
    #[graphql(name = "registrationDate_lt")]
    pub registration_date_lt: Option<String>,
    #[graphql(name = "registrationDate_gte")]
    pub registration_date_gte: Option<String>,
    #[graphql(name = "registrationDate_lte")]
    pub registration_date_lte: Option<String>,
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
    pub cost: Option<String>,
    #[graphql(name = "cost_gt")]
    pub cost_gt: Option<String>,
    #[graphql(name = "cost_lt")]
    pub cost_lt: Option<String>,
    #[graphql(name = "cost_gte")]
    pub cost_gte: Option<String>,
    #[graphql(name = "cost_lte")]
    pub cost_lte: Option<String>,
}

impl From<RegistrationFilter> for StorageRegistrationFilter {
    fn from(value: RegistrationFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            domain_id: value.domain,
            domain_filter: value.domain_filter.map(|filter| Box::new((*filter).into())),
            registrant_id: value.registrant,
            registrant_filter: value
                .registrant_filter
                .map(|filter| Box::new((*filter).into())),
            label_name: value.label_name,
            label_name_contains: value.label_name_contains,
            label_name_contains_nocase: value.label_name_contains_nocase,
            label_name_starts_with: value.label_name_starts_with,
            label_name_ends_with: value.label_name_ends_with,
            registration_date: value.registration_date,
            registration_date_gt: value.registration_date_gt,
            registration_date_lt: value.registration_date_lt,
            registration_date_gte: value.registration_date_gte,
            registration_date_lte: value.registration_date_lte,
            expiry_date: value.expiry_date,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            cost: value.cost,
            cost_gt: value.cost_gt,
            cost_lt: value.cost_lt,
            cost_gte: value.cost_gte,
            cost_lte: value.cost_lte,
        }
    }
}

#[derive(Debug, Clone, InputObject, Default)]
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
