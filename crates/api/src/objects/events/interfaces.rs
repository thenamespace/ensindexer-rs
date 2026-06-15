use async_graphql::Interface;

use super::domain::{
    ExpiryExtendedEvent, FusesSetEvent, NameUnwrappedEvent, NameWrappedEvent, NewOwnerEvent,
    NewResolverEvent, NewTtlEvent, TransferEvent, WrappedTransferEvent,
};
use super::registration::{NameRegisteredEvent, NameRenewedEvent, NameTransferredEvent};
use super::resolver::{
    AbiChangedEvent, AddrChangedEvent, AuthorisationChangedEvent, ContenthashChangedEvent,
    InterfaceChangedEvent, MulticoinAddrChangedEvent, NameChangedEvent, PubkeyChangedEvent,
    TextChangedEvent, VersionChangedEvent,
};
use crate::objects::{Domain, Registration, Resolver};

// async-graphql interface metadata repeats `ty` per field; clippy reads that as
// duplicate attributes even though the macro requires this shape.
#[allow(clippy::duplicated_attributes)]
#[derive(Interface)]
#[graphql(
    name = "DomainEvent",
    field(name = "id", ty = "&String"),
    field(name = "domain", method = "domain", ty = "Option<Domain>"),
    field(name = "blockNumber", method = "block_number", ty = "&i32"),
    field(name = "transactionID", method = "transaction_id", ty = "&String")
)]
pub enum DomainEvent {
    TransferEvent(TransferEvent),
    NewOwnerEvent(NewOwnerEvent),
    NewResolverEvent(NewResolverEvent),
    NewTtlEvent(NewTtlEvent),
    WrappedTransferEvent(WrappedTransferEvent),
    NameWrappedEvent(NameWrappedEvent),
    NameUnwrappedEvent(NameUnwrappedEvent),
    FusesSetEvent(FusesSetEvent),
    ExpiryExtendedEvent(ExpiryExtendedEvent),
}

// async-graphql interface metadata repeats `ty` per field; clippy reads that as
// duplicate attributes even though the macro requires this shape.
#[allow(clippy::duplicated_attributes)]
#[derive(Interface)]
#[graphql(
    name = "RegistrationEvent",
    field(name = "id", ty = "&String"),
    field(
        name = "registration",
        method = "registration",
        ty = "Option<Registration>"
    ),
    field(name = "blockNumber", method = "block_number", ty = "&i32"),
    field(name = "transactionID", method = "transaction_id", ty = "&String")
)]
pub enum RegistrationEvent {
    NameRegisteredEvent(NameRegisteredEvent),
    NameRenewedEvent(NameRenewedEvent),
    NameTransferredEvent(NameTransferredEvent),
}

// async-graphql interface metadata repeats `ty` per field; clippy reads that as
// duplicate attributes even though the macro requires this shape.
#[allow(clippy::duplicated_attributes)]
#[derive(Interface)]
#[graphql(
    name = "ResolverEvent",
    field(name = "id", ty = "&String"),
    field(name = "resolver", method = "resolver", ty = "Option<Resolver>"),
    field(name = "blockNumber", method = "block_number", ty = "&i32"),
    field(name = "transactionID", method = "transaction_id", ty = "&String")
)]
pub enum ResolverEvent {
    AddrChangedEvent(AddrChangedEvent),
    MulticoinAddrChangedEvent(MulticoinAddrChangedEvent),
    NameChangedEvent(NameChangedEvent),
    AbiChangedEvent(AbiChangedEvent),
    PubkeyChangedEvent(PubkeyChangedEvent),
    TextChangedEvent(TextChangedEvent),
    ContenthashChangedEvent(ContenthashChangedEvent),
    InterfaceChangedEvent(InterfaceChangedEvent),
    AuthorisationChangedEvent(AuthorisationChangedEvent),
    VersionChangedEvent(VersionChangedEvent),
}
