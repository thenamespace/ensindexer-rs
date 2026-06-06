use async_graphql::Enum;
use storage::{
    AccountOrderField, DomainOrderField, RegistrationOrderField, ResolverOrderField,
    WrappedDomainOrderField,
};

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "Account_orderBy")]
pub enum AccountOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "domains")]
    Domains,
    #[graphql(name = "wrappedDomains")]
    WrappedDomains,
    #[graphql(name = "registrations")]
    Registrations,
}

impl From<AccountOrderBy> for AccountOrderField {
    fn from(value: AccountOrderBy) -> Self {
        match value {
            AccountOrderBy::Id => Self::Id,
            AccountOrderBy::Domains => Self::Domains,
            AccountOrderBy::WrappedDomains => Self::WrappedDomains,
            AccountOrderBy::Registrations => Self::Registrations,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "Domain_orderBy")]
pub enum DomainOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "name")]
    Name,
    #[graphql(name = "labelName")]
    LabelName,
    #[graphql(name = "labelhash")]
    Labelhash,
    #[graphql(name = "parent")]
    Parent,
    #[graphql(name = "parent__id")]
    ParentId,
    #[graphql(name = "parent__name")]
    ParentName,
    #[graphql(name = "parent__labelName")]
    ParentLabelName,
    #[graphql(name = "parent__labelhash")]
    ParentLabelhash,
    #[graphql(name = "parent__subdomainCount")]
    ParentSubdomainCount,
    #[graphql(name = "parent__ttl")]
    ParentTtl,
    #[graphql(name = "parent__isMigrated")]
    ParentIsMigrated,
    #[graphql(name = "parent__createdAt")]
    ParentCreatedAt,
    #[graphql(name = "parent__expiryDate")]
    ParentExpiryDate,
    #[graphql(name = "subdomains")]
    Subdomains,
    #[graphql(name = "subdomainCount")]
    SubdomainCount,
    #[graphql(name = "resolvedAddress")]
    ResolvedAddress,
    #[graphql(name = "resolvedAddress__id")]
    ResolvedAddressId,
    #[graphql(name = "resolver")]
    Resolver,
    #[graphql(name = "resolver__id")]
    ResolverId,
    #[graphql(name = "resolver__address")]
    ResolverAddress,
    #[graphql(name = "resolver__contentHash")]
    ResolverContentHash,
    #[graphql(name = "ttl")]
    Ttl,
    #[graphql(name = "isMigrated")]
    IsMigrated,
    #[graphql(name = "createdAt")]
    CreatedAt,
    #[graphql(name = "owner")]
    Owner,
    #[graphql(name = "owner__id")]
    OwnerId,
    #[graphql(name = "registrant")]
    Registrant,
    #[graphql(name = "registrant__id")]
    RegistrantId,
    #[graphql(name = "wrappedOwner")]
    WrappedOwner,
    #[graphql(name = "wrappedOwner__id")]
    WrappedOwnerId,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    #[graphql(name = "registration")]
    Registration,
    #[graphql(name = "registration__id")]
    RegistrationId,
    #[graphql(name = "registration__registrationDate")]
    RegistrationRegistrationDate,
    #[graphql(name = "registration__expiryDate")]
    RegistrationExpiryDate,
    #[graphql(name = "registration__cost")]
    RegistrationCost,
    #[graphql(name = "registration__labelName")]
    RegistrationLabelName,
    #[graphql(name = "wrappedDomain")]
    WrappedDomain,
    #[graphql(name = "wrappedDomain__id")]
    WrappedDomainId,
    #[graphql(name = "wrappedDomain__expiryDate")]
    WrappedDomainExpiryDate,
    #[graphql(name = "wrappedDomain__fuses")]
    WrappedDomainFuses,
    #[graphql(name = "wrappedDomain__name")]
    WrappedDomainName,
    #[graphql(name = "events")]
    Events,
}

impl From<DomainOrderBy> for DomainOrderField {
    fn from(value: DomainOrderBy) -> Self {
        match value {
            DomainOrderBy::Id => Self::Id,
            DomainOrderBy::Name => Self::Name,
            DomainOrderBy::LabelName => Self::LabelName,
            DomainOrderBy::Labelhash => Self::Labelhash,
            DomainOrderBy::Parent => Self::Parent,
            DomainOrderBy::ParentId => Self::ParentId,
            DomainOrderBy::ParentName => Self::ParentName,
            DomainOrderBy::ParentLabelName => Self::ParentLabelName,
            DomainOrderBy::ParentLabelhash => Self::ParentLabelhash,
            DomainOrderBy::ParentSubdomainCount => Self::ParentSubdomainCount,
            DomainOrderBy::ParentTtl => Self::ParentTtl,
            DomainOrderBy::ParentIsMigrated => Self::ParentIsMigrated,
            DomainOrderBy::ParentCreatedAt => Self::ParentCreatedAt,
            DomainOrderBy::ParentExpiryDate => Self::ParentExpiryDate,
            DomainOrderBy::Subdomains => Self::Subdomains,
            DomainOrderBy::SubdomainCount => Self::SubdomainCount,
            DomainOrderBy::ResolvedAddress => Self::ResolvedAddress,
            DomainOrderBy::ResolvedAddressId => Self::ResolvedAddressId,
            DomainOrderBy::Resolver => Self::Resolver,
            DomainOrderBy::ResolverId => Self::ResolverId,
            DomainOrderBy::ResolverAddress => Self::ResolverAddress,
            DomainOrderBy::ResolverContentHash => Self::ResolverContentHash,
            DomainOrderBy::Ttl => Self::Ttl,
            DomainOrderBy::IsMigrated => Self::IsMigrated,
            DomainOrderBy::CreatedAt => Self::CreatedAt,
            DomainOrderBy::Owner => Self::Owner,
            DomainOrderBy::OwnerId => Self::OwnerId,
            DomainOrderBy::Registrant => Self::Registrant,
            DomainOrderBy::RegistrantId => Self::RegistrantId,
            DomainOrderBy::WrappedOwner => Self::WrappedOwner,
            DomainOrderBy::WrappedOwnerId => Self::WrappedOwnerId,
            DomainOrderBy::ExpiryDate => Self::ExpiryDate,
            DomainOrderBy::Registration => Self::Registration,
            DomainOrderBy::RegistrationId => Self::RegistrationId,
            DomainOrderBy::RegistrationRegistrationDate => Self::RegistrationRegistrationDate,
            DomainOrderBy::RegistrationExpiryDate => Self::RegistrationExpiryDate,
            DomainOrderBy::RegistrationCost => Self::RegistrationCost,
            DomainOrderBy::RegistrationLabelName => Self::RegistrationLabelName,
            DomainOrderBy::WrappedDomain => Self::WrappedDomain,
            DomainOrderBy::WrappedDomainId => Self::WrappedDomainId,
            DomainOrderBy::WrappedDomainExpiryDate => Self::WrappedDomainExpiryDate,
            DomainOrderBy::WrappedDomainFuses => Self::WrappedDomainFuses,
            DomainOrderBy::WrappedDomainName => Self::WrappedDomainName,
            DomainOrderBy::Events => Self::Events,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "Registration_orderBy")]
pub enum RegistrationOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "domain")]
    Domain,
    #[graphql(name = "domain__id")]
    DomainId,
    #[graphql(name = "domain__name")]
    DomainName,
    #[graphql(name = "domain__labelName")]
    DomainLabelName,
    #[graphql(name = "domain__labelhash")]
    DomainLabelhash,
    #[graphql(name = "domain__subdomainCount")]
    DomainSubdomainCount,
    #[graphql(name = "domain__ttl")]
    DomainTtl,
    #[graphql(name = "domain__isMigrated")]
    DomainIsMigrated,
    #[graphql(name = "domain__createdAt")]
    DomainCreatedAt,
    #[graphql(name = "domain__expiryDate")]
    DomainExpiryDate,
    #[graphql(name = "registrationDate")]
    RegistrationDate,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    #[graphql(name = "cost")]
    Cost,
    #[graphql(name = "registrant")]
    Registrant,
    #[graphql(name = "registrant__id")]
    RegistrantId,
    #[graphql(name = "labelName")]
    LabelName,
    #[graphql(name = "events")]
    Events,
}

impl From<RegistrationOrderBy> for RegistrationOrderField {
    fn from(value: RegistrationOrderBy) -> Self {
        match value {
            RegistrationOrderBy::Id => Self::Id,
            RegistrationOrderBy::Domain => Self::Domain,
            RegistrationOrderBy::DomainId => Self::DomainId,
            RegistrationOrderBy::DomainName => Self::DomainName,
            RegistrationOrderBy::DomainLabelName => Self::DomainLabelName,
            RegistrationOrderBy::DomainLabelhash => Self::DomainLabelhash,
            RegistrationOrderBy::DomainSubdomainCount => Self::DomainSubdomainCount,
            RegistrationOrderBy::DomainTtl => Self::DomainTtl,
            RegistrationOrderBy::DomainIsMigrated => Self::DomainIsMigrated,
            RegistrationOrderBy::DomainCreatedAt => Self::DomainCreatedAt,
            RegistrationOrderBy::DomainExpiryDate => Self::DomainExpiryDate,
            RegistrationOrderBy::RegistrationDate => Self::RegistrationDate,
            RegistrationOrderBy::ExpiryDate => Self::ExpiryDate,
            RegistrationOrderBy::Cost => Self::Cost,
            RegistrationOrderBy::Registrant => Self::Registrant,
            RegistrationOrderBy::RegistrantId => Self::RegistrantId,
            RegistrationOrderBy::LabelName => Self::LabelName,
            RegistrationOrderBy::Events => Self::Events,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "WrappedDomain_orderBy")]
pub enum WrappedDomainOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "domain")]
    Domain,
    #[graphql(name = "domain__id")]
    DomainId,
    #[graphql(name = "domain__name")]
    DomainName,
    #[graphql(name = "domain__labelName")]
    DomainLabelName,
    #[graphql(name = "domain__labelhash")]
    DomainLabelhash,
    #[graphql(name = "domain__subdomainCount")]
    DomainSubdomainCount,
    #[graphql(name = "domain__ttl")]
    DomainTtl,
    #[graphql(name = "domain__isMigrated")]
    DomainIsMigrated,
    #[graphql(name = "domain__createdAt")]
    DomainCreatedAt,
    #[graphql(name = "domain__expiryDate")]
    DomainExpiryDate,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    #[graphql(name = "fuses")]
    Fuses,
    #[graphql(name = "owner")]
    Owner,
    #[graphql(name = "owner__id")]
    OwnerId,
    #[graphql(name = "name")]
    Name,
}

impl From<WrappedDomainOrderBy> for WrappedDomainOrderField {
    fn from(value: WrappedDomainOrderBy) -> Self {
        match value {
            WrappedDomainOrderBy::Id => Self::Id,
            WrappedDomainOrderBy::Domain => Self::Domain,
            WrappedDomainOrderBy::DomainId => Self::DomainId,
            WrappedDomainOrderBy::DomainName => Self::DomainName,
            WrappedDomainOrderBy::DomainLabelName => Self::DomainLabelName,
            WrappedDomainOrderBy::DomainLabelhash => Self::DomainLabelhash,
            WrappedDomainOrderBy::DomainSubdomainCount => Self::DomainSubdomainCount,
            WrappedDomainOrderBy::DomainTtl => Self::DomainTtl,
            WrappedDomainOrderBy::DomainIsMigrated => Self::DomainIsMigrated,
            WrappedDomainOrderBy::DomainCreatedAt => Self::DomainCreatedAt,
            WrappedDomainOrderBy::DomainExpiryDate => Self::DomainExpiryDate,
            WrappedDomainOrderBy::ExpiryDate => Self::ExpiryDate,
            WrappedDomainOrderBy::Fuses => Self::Fuses,
            WrappedDomainOrderBy::Owner => Self::Owner,
            WrappedDomainOrderBy::OwnerId => Self::OwnerId,
            WrappedDomainOrderBy::Name => Self::Name,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "Resolver_orderBy")]
pub enum ResolverOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "domain")]
    Domain,
    #[graphql(name = "domain__id")]
    DomainId,
    #[graphql(name = "domain__name")]
    DomainName,
    #[graphql(name = "domain__labelName")]
    DomainLabelName,
    #[graphql(name = "domain__labelhash")]
    DomainLabelhash,
    #[graphql(name = "domain__subdomainCount")]
    DomainSubdomainCount,
    #[graphql(name = "domain__ttl")]
    DomainTtl,
    #[graphql(name = "domain__isMigrated")]
    DomainIsMigrated,
    #[graphql(name = "domain__createdAt")]
    DomainCreatedAt,
    #[graphql(name = "domain__expiryDate")]
    DomainExpiryDate,
    #[graphql(name = "address")]
    Address,
    #[graphql(name = "addr")]
    Addr,
    #[graphql(name = "addr__id")]
    AddrId,
    #[graphql(name = "contentHash")]
    ContentHash,
    #[graphql(name = "texts")]
    Texts,
    #[graphql(name = "coinTypes")]
    CoinTypes,
    #[graphql(name = "events")]
    Events,
}

impl From<ResolverOrderBy> for ResolverOrderField {
    fn from(value: ResolverOrderBy) -> Self {
        match value {
            ResolverOrderBy::Id => Self::Id,
            ResolverOrderBy::Domain => Self::Domain,
            ResolverOrderBy::DomainId => Self::DomainId,
            ResolverOrderBy::DomainName => Self::DomainName,
            ResolverOrderBy::DomainLabelName => Self::DomainLabelName,
            ResolverOrderBy::DomainLabelhash => Self::DomainLabelhash,
            ResolverOrderBy::DomainSubdomainCount => Self::DomainSubdomainCount,
            ResolverOrderBy::DomainTtl => Self::DomainTtl,
            ResolverOrderBy::DomainIsMigrated => Self::DomainIsMigrated,
            ResolverOrderBy::DomainCreatedAt => Self::DomainCreatedAt,
            ResolverOrderBy::DomainExpiryDate => Self::DomainExpiryDate,
            ResolverOrderBy::Address => Self::Address,
            ResolverOrderBy::Addr => Self::Addr,
            ResolverOrderBy::AddrId => Self::AddrId,
            ResolverOrderBy::ContentHash => Self::ContentHash,
            ResolverOrderBy::Texts => Self::Texts,
            ResolverOrderBy::CoinTypes => Self::CoinTypes,
            ResolverOrderBy::Events => Self::Events,
        }
    }
}
