use async_graphql::Enum;
use storage::{
    AccountOrderField, DomainOrderField, EventOrderField, OrderDirection as StorageOrderDirection,
    RegistrationOrderField, ResolverOrderField, WrappedDomainOrderField,
};

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "OrderDirection")]
pub enum OrderDirection {
    #[default]
    #[graphql(name = "asc")]
    Asc,
    #[graphql(name = "desc")]
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

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "DomainEvent_orderBy")]
pub enum EventOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "blockNumber")]
    BlockNumber,
    #[graphql(name = "transactionID")]
    TransactionId,
    #[graphql(name = "domain")]
    Domain,
}

impl From<EventOrderBy> for EventOrderField {
    fn from(value: EventOrderBy) -> Self {
        match value {
            EventOrderBy::Id => Self::Id,
            EventOrderBy::BlockNumber => Self::BlockNumber,
            EventOrderBy::TransactionId => Self::TransactionId,
            EventOrderBy::Domain => Self::Domain,
        }
    }
}

macro_rules! event_order_wrapper {
    ($name:ident, $graphql_name:literal, [$($variant:ident => ($graphql_name_variant:literal, $field:ident)),* $(,)?]) => {
        #[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
        #[graphql(name = $graphql_name)]
        pub enum $name {
            #[default]
            #[graphql(name = "id")]
            Id,
            #[graphql(name = "blockNumber")]
            BlockNumber,
            #[graphql(name = "transactionID")]
            TransactionId,
            $(
                #[graphql(name = $graphql_name_variant)]
                $variant,
            )*
        }

        impl From<$name> for EventOrderField {
            fn from(value: $name) -> Self {
                match value {
                    $name::Id => Self::Id,
                    $name::BlockNumber => Self::BlockNumber,
                    $name::TransactionId => Self::TransactionId,
                    $($name::$variant => Self::$field,)*
                }
            }
        }
    };
}

event_order_wrapper!(TransferOrderBy, "Transfer_orderBy", [
    Domain => ("domain", Domain),
    Owner => ("owner", Owner),
]);
event_order_wrapper!(NewOwnerOrderBy, "NewOwner_orderBy", [
    ParentDomain => ("parentDomain", ParentDomain),
    Domain => ("domain", Domain),
    Owner => ("owner", Owner),
]);
event_order_wrapper!(NewResolverOrderBy, "NewResolver_orderBy", [
    Domain => ("domain", Domain),
    Resolver => ("resolver", Resolver),
]);
event_order_wrapper!(NewTtlOrderBy, "NewTTL_orderBy", [
    Domain => ("domain", Domain),
    Ttl => ("ttl", Ttl),
]);
event_order_wrapper!(WrappedTransferOrderBy, "WrappedTransfer_orderBy", [
    Domain => ("domain", Domain),
    Owner => ("owner", Owner),
]);
event_order_wrapper!(NameWrappedOrderBy, "NameWrapped_orderBy", [
    Domain => ("domain", Domain),
    Name => ("name", Name),
    Fuses => ("fuses", Fuses),
    Owner => ("owner", Owner),
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
event_order_wrapper!(NameUnwrappedOrderBy, "NameUnwrapped_orderBy", [
    Domain => ("domain", Domain),
    Owner => ("owner", Owner),
]);
event_order_wrapper!(FusesSetOrderBy, "FusesSet_orderBy", [
    Domain => ("domain", Domain),
    Fuses => ("fuses", Fuses),
]);
event_order_wrapper!(ExpiryExtendedOrderBy, "ExpiryExtended_orderBy", [
    Domain => ("domain", Domain),
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
event_order_wrapper!(NameRegisteredOrderBy, "NameRegistered_orderBy", [
    Registration => ("registration", Registration),
    Registrant => ("registrant", Registrant),
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
event_order_wrapper!(NameRenewedOrderBy, "NameRenewed_orderBy", [
    Registration => ("registration", Registration),
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
event_order_wrapper!(NameTransferredOrderBy, "NameTransferred_orderBy", [
    Registration => ("registration", Registration),
    NewOwner => ("newOwner", NewOwner),
]);
event_order_wrapper!(AddrChangedOrderBy, "AddrChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    Addr => ("addr", Addr),
]);
event_order_wrapper!(MulticoinAddrChangedOrderBy, "MulticoinAddrChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    CoinType => ("coinType", CoinType),
    Addr => ("addr", Addr),
]);
event_order_wrapper!(NameChangedOrderBy, "NameChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    Name => ("name", Name),
]);
event_order_wrapper!(AbiChangedOrderBy, "AbiChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    ContentType => ("contentType", ContentType),
]);
event_order_wrapper!(PubkeyChangedOrderBy, "PubkeyChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    X => ("x", X),
    Y => ("y", Y),
]);
event_order_wrapper!(TextChangedOrderBy, "TextChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    Key => ("key", Key),
    Value => ("value", Value),
]);
event_order_wrapper!(ContenthashChangedOrderBy, "ContenthashChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    Hash => ("hash", Hash),
]);
event_order_wrapper!(InterfaceChangedOrderBy, "InterfaceChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    InterfaceId => ("interfaceID", InterfaceId),
    Implementer => ("implementer", Implementer),
]);
event_order_wrapper!(AuthorisationChangedOrderBy, "AuthorisationChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    Owner => ("owner", Owner),
    Target => ("target", Target),
    IsAuthorized => ("isAuthorized", IsAuthorized),
]);
event_order_wrapper!(VersionChangedOrderBy, "VersionChanged_orderBy", [
    Resolver => ("resolver", Resolver),
    Version => ("version", Version),
]);
event_order_wrapper!(RegistrationEventOrderBy, "RegistrationEvent_orderBy", [
    Registration => ("registration", Registration),
]);
event_order_wrapper!(ResolverEventOrderBy, "ResolverEvent_orderBy", [
    Resolver => ("resolver", Resolver),
]);
