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
    Domains,
    WrappedDomains,
    Registrations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainOrderField {
    Id,
    Name,
    LabelName,
    Labelhash,
    Parent,
    ParentId,
    ParentName,
    ParentLabelName,
    ParentLabelhash,
    ParentSubdomainCount,
    ParentTtl,
    ParentIsMigrated,
    ParentCreatedAt,
    ParentExpiryDate,
    Subdomains,
    SubdomainCount,
    ResolvedAddress,
    ResolvedAddressId,
    Resolver,
    ResolverId,
    ResolverAddress,
    ResolverContentHash,
    Ttl,
    IsMigrated,
    CreatedAt,
    Owner,
    OwnerId,
    Registrant,
    RegistrantId,
    WrappedOwner,
    WrappedOwnerId,
    ExpiryDate,
    Registration,
    RegistrationId,
    RegistrationRegistrationDate,
    RegistrationExpiryDate,
    RegistrationCost,
    RegistrationLabelName,
    WrappedDomain,
    WrappedDomainId,
    WrappedDomainExpiryDate,
    WrappedDomainFuses,
    WrappedDomainName,
    Events,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationOrderField {
    Id,
    Domain,
    DomainId,
    DomainName,
    DomainLabelName,
    DomainLabelhash,
    DomainSubdomainCount,
    DomainTtl,
    DomainIsMigrated,
    DomainCreatedAt,
    DomainExpiryDate,
    RegistrationDate,
    ExpiryDate,
    Cost,
    Registrant,
    RegistrantId,
    LabelName,
    Events,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrappedDomainOrderField {
    Id,
    Domain,
    DomainId,
    DomainName,
    DomainLabelName,
    DomainLabelhash,
    DomainSubdomainCount,
    DomainTtl,
    DomainIsMigrated,
    DomainCreatedAt,
    DomainExpiryDate,
    ExpiryDate,
    Fuses,
    Owner,
    OwnerId,
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverOrderField {
    Id,
    Domain,
    DomainId,
    DomainName,
    DomainLabelName,
    DomainLabelhash,
    DomainSubdomainCount,
    DomainTtl,
    DomainIsMigrated,
    DomainCreatedAt,
    DomainExpiryDate,
    Address,
    Addr,
    AddrId,
    ContentHash,
    Texts,
    CoinTypes,
    Events,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventOrderField {
    Id,
    BlockNumber,
    TransactionId,
    Domain,
    ParentDomain,
    Registration,
    Resolver,
    Owner,
    Registrant,
    NewOwner,
    Addr,
    Name,
    Fuses,
    Ttl,
    ExpiryDate,
    CoinType,
    ContentType,
    X,
    Y,
    Key,
    Value,
    Hash,
    InterfaceId,
    Implementer,
    Target,
    IsAuthorized,
    Version,
}
