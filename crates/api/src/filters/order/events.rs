use async_graphql::Enum;
use storage::EventOrderField;

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
