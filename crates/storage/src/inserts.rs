use bigdecimal::BigDecimal;

#[derive(Debug, Clone)]
pub struct DomainUpsert {
    pub id: String,
    pub created_at: BigDecimal,
    pub owner_id: String,
    pub is_migrated: bool,
}

#[derive(Debug, Clone)]
pub struct NewOwnerEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub parent_domain_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct TransferEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct NewResolverEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub resolver_id: String,
}

#[derive(Debug, Clone)]
pub struct NewTtlEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub ttl: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct NameRegisteredEventInsert {
    pub id: String,
    pub registration_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub registrant_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct NameRenewedEventInsert {
    pub id: String,
    pub registration_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct NameTransferredEventInsert {
    pub id: String,
    pub registration_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub new_owner_id: String,
}

#[derive(Debug, Clone)]
pub struct WrappedTransferEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct NameWrappedEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub name: Option<String>,
    pub fuses: i32,
    pub owner_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct NameUnwrappedEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct FusesSetEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub fuses: i32,
}

#[derive(Debug, Clone)]
pub struct ExpiryExtendedEventInsert {
    pub id: String,
    pub domain_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub expiry_date: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct AddrChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub addr_id: String,
}

#[derive(Debug, Clone)]
pub struct MulticoinAddrChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub coin_type: BigDecimal,
    pub addr: String,
}

#[derive(Debug, Clone)]
pub struct NameChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AbiChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub content_type: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct PubkeyChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone)]
pub struct TextChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContenthashChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct InterfaceChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub interface_id: String,
    pub implementer: String,
}

#[derive(Debug, Clone)]
pub struct AuthorisationChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub owner: String,
    pub target: String,
    pub is_authorized: bool,
}

#[derive(Debug, Clone)]
pub struct VersionChangedEventInsert {
    pub id: String,
    pub resolver_id: String,
    pub block_number: i32,
    pub transaction_id: String,
    pub version: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct BlockInsert {
    pub number: i64,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub timestamp: i64,
}
