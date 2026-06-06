#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl OrderDirection {
    pub(crate) fn sql(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountOrderField {
    Id,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainOrderField {
    Id,
    Name,
    LabelName,
    SubdomainCount,
    CreatedAt,
    ExpiryDate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationOrderField {
    Id,
    RegistrationDate,
    ExpiryDate,
    Cost,
    LabelName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrappedDomainOrderField {
    Id,
    ExpiryDate,
    Fuses,
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverOrderField {
    Id,
    Address,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventOrderField {
    Id,
    BlockNumber,
    TransactionId,
}

#[derive(Debug, Clone, Default)]
pub struct AccountFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub and: Option<Vec<AccountFilter>>,
    pub or: Option<Vec<AccountFilter>>,
}

#[derive(Debug, Clone, Default)]
pub struct DomainFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
    pub name_contains_nocase: Option<String>,
    pub name_starts_with: Option<String>,
    pub name_ends_with: Option<String>,
    pub label_name: Option<String>,
    pub label_name_contains: Option<String>,
    pub label_name_contains_nocase: Option<String>,
    pub label_name_starts_with: Option<String>,
    pub label_name_ends_with: Option<String>,
    pub labelhash: Option<String>,
    pub labelhash_not: Option<String>,
    pub labelhash_in: Option<Vec<String>>,
    pub labelhash_not_in: Option<Vec<String>>,
    pub parent_id: Option<String>,
    pub parent_filter: Option<Box<DomainFilter>>,
    pub subdomain_count: Option<i32>,
    pub subdomain_count_gt: Option<i32>,
    pub subdomain_count_lt: Option<i32>,
    pub subdomain_count_gte: Option<i32>,
    pub subdomain_count_lte: Option<i32>,
    pub resolved_address_id: Option<String>,
    pub resolved_address_filter: Option<Box<AccountFilter>>,
    pub owner_id: Option<String>,
    pub owner_filter: Option<Box<AccountFilter>>,
    pub resolver_id: Option<String>,
    pub resolver_filter: Option<Box<ResolverFilter>>,
    pub registrant_id: Option<String>,
    pub registrant_filter: Option<Box<AccountFilter>>,
    pub wrapped_owner_id: Option<String>,
    pub wrapped_owner_filter: Option<Box<AccountFilter>>,
    pub is_migrated: Option<bool>,
    pub is_migrated_not: Option<bool>,
    pub created_at: Option<String>,
    pub created_at_gt: Option<String>,
    pub created_at_lt: Option<String>,
    pub created_at_gte: Option<String>,
    pub created_at_lte: Option<String>,
    pub expiry_date: Option<String>,
    pub expiry_date_gt: Option<String>,
    pub expiry_date_lt: Option<String>,
    pub expiry_date_gte: Option<String>,
    pub expiry_date_lte: Option<String>,
    pub ttl: Option<String>,
    pub ttl_gt: Option<String>,
    pub ttl_lt: Option<String>,
    pub ttl_gte: Option<String>,
    pub ttl_lte: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RegistrationFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub domain_id: Option<String>,
    pub domain_filter: Option<Box<DomainFilter>>,
    pub registrant_id: Option<String>,
    pub registrant_filter: Option<Box<AccountFilter>>,
    pub label_name: Option<String>,
    pub label_name_contains: Option<String>,
    pub label_name_contains_nocase: Option<String>,
    pub label_name_starts_with: Option<String>,
    pub label_name_ends_with: Option<String>,
    pub registration_date: Option<String>,
    pub registration_date_gt: Option<String>,
    pub registration_date_lt: Option<String>,
    pub registration_date_gte: Option<String>,
    pub registration_date_lte: Option<String>,
    pub expiry_date: Option<String>,
    pub expiry_date_gt: Option<String>,
    pub expiry_date_lt: Option<String>,
    pub expiry_date_gte: Option<String>,
    pub expiry_date_lte: Option<String>,
    pub cost: Option<String>,
    pub cost_gt: Option<String>,
    pub cost_lt: Option<String>,
    pub cost_gte: Option<String>,
    pub cost_lte: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WrappedDomainFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub domain_id: Option<String>,
    pub domain_filter: Option<Box<DomainFilter>>,
    pub owner_id: Option<String>,
    pub owner_filter: Option<Box<AccountFilter>>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
    pub name_contains_nocase: Option<String>,
    pub name_starts_with: Option<String>,
    pub name_ends_with: Option<String>,
    pub expiry_date: Option<String>,
    pub expiry_date_gt: Option<String>,
    pub expiry_date_lt: Option<String>,
    pub expiry_date_gte: Option<String>,
    pub expiry_date_lte: Option<String>,
    pub fuses: Option<i32>,
    pub fuses_gt: Option<i32>,
    pub fuses_lt: Option<i32>,
    pub fuses_gte: Option<i32>,
    pub fuses_lte: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct ResolverFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub domain_id: Option<String>,
    pub domain_filter: Option<Box<DomainFilter>>,
    pub address: Option<String>,
    pub address_in: Option<Vec<String>>,
    pub addr_id: Option<String>,
    pub addr_filter: Option<Box<AccountFilter>>,
    pub content_hash: Option<String>,
    pub content_hash_not: Option<String>,
    pub content_hash_in: Option<Vec<String>>,
    pub content_hash_not_in: Option<Vec<String>>,
    pub content_hash_contains: Option<String>,
    pub texts_contains: Option<String>,
    pub coin_types_contains: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub parent_id: Option<String>,
    pub block_number: Option<i32>,
    pub block_number_gt: Option<i32>,
    pub block_number_lt: Option<i32>,
    pub block_number_gte: Option<i32>,
    pub block_number_lte: Option<i32>,
    pub transaction_id: Option<String>,
    pub transaction_id_not: Option<String>,
    pub transaction_id_in: Option<Vec<String>>,
    pub transaction_id_not_in: Option<Vec<String>>,
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
