use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountRow {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DomainRow {
    pub id: String,
    pub name: Option<String>,
    pub label_name: Option<String>,
    pub labelhash: Option<String>,
    pub parent_id: Option<String>,
    pub subdomain_count: i32,
    pub resolved_address_id: Option<String>,
    pub resolver_id: Option<String>,
    pub ttl: Option<BigDecimal>,
    pub is_migrated: bool,
    pub created_at: BigDecimal,
    pub owner_id: String,
    pub registrant_id: Option<String>,
    pub wrapped_owner_id: Option<String>,
    pub expiry_date: Option<BigDecimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegistrationRow {
    pub id: String,
    pub domain_id: String,
    pub registration_date: BigDecimal,
    pub expiry_date: BigDecimal,
    pub cost: Option<BigDecimal>,
    pub registrant_id: String,
    pub label_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResolverRow {
    pub id: String,
    pub domain_id: Option<String>,
    pub address: String,
    pub addr_id: Option<String>,
    pub content_hash: Option<String>,
    pub texts: Vec<String>,
    pub coin_types: Vec<BigDecimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WrappedDomainRow {
    pub id: String,
    pub domain_id: String,
    pub expiry_date: BigDecimal,
    pub fuses: i32,
    pub owner_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SourceCheckpointRow {
    pub source: String,
    pub block_number: i64,
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BlockRow {
    pub number: i64,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransferEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewOwnerEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub parent_domain_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewResolverEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub resolver_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewTtlEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub ttl: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WrappedTransferEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NameWrappedEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub name: Option<String>,
    pub fuses: i32,
    pub owner_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NameUnwrappedEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FusesSetEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub fuses: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExpiryExtendedEventRow {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NameRegisteredEventRow {
    pub id: String,
    pub registration_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub registrant_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NameRenewedEventRow {
    pub id: String,
    pub registration_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NameTransferredEventRow {
    pub id: String,
    pub registration_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub new_owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AddrChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub addr_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MulticoinAddrChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub coin_type: BigDecimal,
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NameChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AbiChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub content_type: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PubkeyChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TextChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContenthashChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InterfaceChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub interface_id: String,
    pub implementer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthorisationChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner: String,
    pub target: String,
    pub is_authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VersionChangedEventRow {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub version: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventReferenceRow {
    pub kind: String,
    pub id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub parent_id: String,
}
