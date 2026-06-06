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
}

impl From<AccountOrderBy> for AccountOrderField {
    fn from(value: AccountOrderBy) -> Self {
        match value {
            AccountOrderBy::Id => Self::Id,
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
    #[graphql(name = "subdomainCount")]
    SubdomainCount,
    #[graphql(name = "resolvedAddress")]
    ResolvedAddress,
    #[graphql(name = "resolver")]
    Resolver,
    #[graphql(name = "ttl")]
    Ttl,
    #[graphql(name = "isMigrated")]
    IsMigrated,
    #[graphql(name = "createdAt")]
    CreatedAt,
    #[graphql(name = "owner")]
    Owner,
    #[graphql(name = "registrant")]
    Registrant,
    #[graphql(name = "wrappedOwner")]
    WrappedOwner,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
}

impl From<DomainOrderBy> for DomainOrderField {
    fn from(value: DomainOrderBy) -> Self {
        match value {
            DomainOrderBy::Id => Self::Id,
            DomainOrderBy::Name => Self::Name,
            DomainOrderBy::LabelName => Self::LabelName,
            DomainOrderBy::Labelhash => Self::Labelhash,
            DomainOrderBy::Parent => Self::Parent,
            DomainOrderBy::SubdomainCount => Self::SubdomainCount,
            DomainOrderBy::ResolvedAddress => Self::ResolvedAddress,
            DomainOrderBy::Resolver => Self::Resolver,
            DomainOrderBy::Ttl => Self::Ttl,
            DomainOrderBy::IsMigrated => Self::IsMigrated,
            DomainOrderBy::CreatedAt => Self::CreatedAt,
            DomainOrderBy::Owner => Self::Owner,
            DomainOrderBy::Registrant => Self::Registrant,
            DomainOrderBy::WrappedOwner => Self::WrappedOwner,
            DomainOrderBy::ExpiryDate => Self::ExpiryDate,
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
    #[graphql(name = "registrationDate")]
    RegistrationDate,
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    #[graphql(name = "cost")]
    Cost,
    #[graphql(name = "registrant")]
    Registrant,
    #[graphql(name = "labelName")]
    LabelName,
}

impl From<RegistrationOrderBy> for RegistrationOrderField {
    fn from(value: RegistrationOrderBy) -> Self {
        match value {
            RegistrationOrderBy::Id => Self::Id,
            RegistrationOrderBy::Domain => Self::Domain,
            RegistrationOrderBy::RegistrationDate => Self::RegistrationDate,
            RegistrationOrderBy::ExpiryDate => Self::ExpiryDate,
            RegistrationOrderBy::Cost => Self::Cost,
            RegistrationOrderBy::Registrant => Self::Registrant,
            RegistrationOrderBy::LabelName => Self::LabelName,
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
    #[graphql(name = "expiryDate")]
    ExpiryDate,
    #[graphql(name = "fuses")]
    Fuses,
    #[graphql(name = "owner")]
    Owner,
    #[graphql(name = "name")]
    Name,
}

impl From<WrappedDomainOrderBy> for WrappedDomainOrderField {
    fn from(value: WrappedDomainOrderBy) -> Self {
        match value {
            WrappedDomainOrderBy::Id => Self::Id,
            WrappedDomainOrderBy::Domain => Self::Domain,
            WrappedDomainOrderBy::ExpiryDate => Self::ExpiryDate,
            WrappedDomainOrderBy::Fuses => Self::Fuses,
            WrappedDomainOrderBy::Owner => Self::Owner,
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
    #[graphql(name = "address")]
    Address,
    #[graphql(name = "addr")]
    Addr,
    #[graphql(name = "contentHash")]
    ContentHash,
}

impl From<ResolverOrderBy> for ResolverOrderField {
    fn from(value: ResolverOrderBy) -> Self {
        match value {
            ResolverOrderBy::Id => Self::Id,
            ResolverOrderBy::Domain => Self::Domain,
            ResolverOrderBy::Address => Self::Address,
            ResolverOrderBy::Addr => Self::Addr,
            ResolverOrderBy::ContentHash => Self::ContentHash,
        }
    }
}
