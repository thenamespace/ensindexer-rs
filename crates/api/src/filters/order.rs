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
#[graphql(name = "Registration_orderBy")]
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
#[graphql(name = "WrappedDomain_orderBy")]
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
#[graphql(name = "Resolver_orderBy")]
pub enum ResolverOrderBy {
    #[default]
    Id,
    Address,
}

impl From<ResolverOrderBy> for ResolverOrderField {
    fn from(value: ResolverOrderBy) -> Self {
        match value {
            ResolverOrderBy::Id => Self::Id,
            ResolverOrderBy::Address => Self::Address,
        }
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "DomainEvent_orderBy")]
pub enum EventOrderBy {
    #[default]
    Id,
    #[graphql(name = "blockNumber")]
    BlockNumber,
    #[graphql(name = "transactionID")]
    TransactionId,
}

impl From<EventOrderBy> for EventOrderField {
    fn from(value: EventOrderBy) -> Self {
        match value {
            EventOrderBy::Id => Self::Id,
            EventOrderBy::BlockNumber => Self::BlockNumber,
            EventOrderBy::TransactionId => Self::TransactionId,
        }
    }
}

macro_rules! event_order_wrapper {
    ($name:ident, $graphql_name:literal) => {
        #[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
        #[graphql(name = $graphql_name)]
        pub enum $name {
            #[default]
            Id,
            #[graphql(name = "blockNumber")]
            BlockNumber,
            #[graphql(name = "transactionID")]
            TransactionId,
        }

        impl From<$name> for EventOrderField {
            fn from(value: $name) -> Self {
                match value {
                    $name::Id => Self::Id,
                    $name::BlockNumber => Self::BlockNumber,
                    $name::TransactionId => Self::TransactionId,
                }
            }
        }
    };
}

event_order_wrapper!(TransferOrderBy, "Transfer_orderBy");
event_order_wrapper!(NewOwnerOrderBy, "NewOwner_orderBy");
event_order_wrapper!(NewResolverOrderBy, "NewResolver_orderBy");
event_order_wrapper!(NewTtlOrderBy, "NewTTL_orderBy");
event_order_wrapper!(WrappedTransferOrderBy, "WrappedTransfer_orderBy");
event_order_wrapper!(NameWrappedOrderBy, "NameWrapped_orderBy");
event_order_wrapper!(NameUnwrappedOrderBy, "NameUnwrapped_orderBy");
event_order_wrapper!(FusesSetOrderBy, "FusesSet_orderBy");
event_order_wrapper!(ExpiryExtendedOrderBy, "ExpiryExtended_orderBy");
event_order_wrapper!(NameRegisteredOrderBy, "NameRegistered_orderBy");
event_order_wrapper!(NameRenewedOrderBy, "NameRenewed_orderBy");
event_order_wrapper!(NameTransferredOrderBy, "NameTransferred_orderBy");
event_order_wrapper!(AddrChangedOrderBy, "AddrChanged_orderBy");
event_order_wrapper!(MulticoinAddrChangedOrderBy, "MulticoinAddrChanged_orderBy");
event_order_wrapper!(NameChangedOrderBy, "NameChanged_orderBy");
event_order_wrapper!(AbiChangedOrderBy, "AbiChanged_orderBy");
event_order_wrapper!(PubkeyChangedOrderBy, "PubkeyChanged_orderBy");
event_order_wrapper!(TextChangedOrderBy, "TextChanged_orderBy");
event_order_wrapper!(ContenthashChangedOrderBy, "ContenthashChanged_orderBy");
event_order_wrapper!(InterfaceChangedOrderBy, "InterfaceChanged_orderBy");
event_order_wrapper!(AuthorisationChangedOrderBy, "AuthorisationChanged_orderBy");
event_order_wrapper!(VersionChangedOrderBy, "VersionChanged_orderBy");
event_order_wrapper!(RegistrationEventOrderBy, "RegistrationEvent_orderBy");
event_order_wrapper!(ResolverEventOrderBy, "ResolverEvent_orderBy");
