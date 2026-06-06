use super::{AccountFilter, DomainFilter};

#[derive(Debug, Clone, Default)]
pub struct ResolverFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_gt: Option<String>,
    pub id_lt: Option<String>,
    pub id_gte: Option<String>,
    pub id_lte: Option<String>,
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
