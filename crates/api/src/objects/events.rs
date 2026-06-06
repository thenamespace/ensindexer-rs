use async_graphql::{Context, Interface, Result, SimpleObject};
use storage::{
    AbiChangedEventRow, AddrChangedEventRow, AuthorisationChangedEventRow,
    ContenthashChangedEventRow, DomainRow, EventReferenceRow, ExpiryExtendedEventRow,
    FusesSetEventRow, InterfaceChangedEventRow, MulticoinAddrChangedEventRow, NameChangedEventRow,
    NameRegisteredEventRow, NameRenewedEventRow, NameTransferredEventRow, NameUnwrappedEventRow,
    NameWrappedEventRow, NewOwnerEventRow, NewResolverEventRow, NewTtlEventRow,
    PubkeyChangedEventRow, RegistrationRow, ResolverRow, Storage, TextChangedEventRow,
    TransferEventRow, VersionChangedEventRow, WrappedTransferEventRow,
};

use crate::objects::{Account, Domain, Registration, Resolver};

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

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "Transfer")]
pub struct TransferEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<TransferEventRow> for TransferEvent {
    fn from(value: TransferEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            domain_id: value.domain_id,
            owner_id: value.owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl TransferEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.owner_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NewOwner")]
pub struct NewOwnerEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub parent_domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<NewOwnerEventRow> for NewOwnerEvent {
    fn from(value: NewOwnerEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            domain_id: value.domain_id,
            parent_domain_id: value.parent_domain_id,
            owner_id: value.owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NewOwnerEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn parent_domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.parent_domain_id).await
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.owner_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NewResolver")]
pub struct NewResolverEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<NewResolverEventRow> for NewResolverEvent {
    fn from(value: NewResolverEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            domain_id: value.domain_id,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NewResolverEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NewTTL")]
pub struct NewTtlEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub ttl: String,
    #[graphql(skip)]
    pub domain_id: String,
}

impl From<NewTtlEventRow> for NewTtlEvent {
    fn from(value: NewTtlEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            ttl: value.ttl.to_string(),
            domain_id: value.domain_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NewTtlEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }
}

macro_rules! impl_domain_owner_event {
    ($name:ident, $graphql_name:literal, $row:ident) => {
        #[derive(Debug, Clone, SimpleObject)]
        #[graphql(complex, name = $graphql_name)]
        pub struct $name {
            pub id: String,
            #[graphql(name = "blockNumber")]
            pub block_number: i32,
            #[graphql(name = "transactionID")]
            pub transaction_id: String,
            #[graphql(skip)]
            pub domain_id: String,
            #[graphql(skip)]
            pub owner_id: String,
        }

        impl From<$row> for $name {
            fn from(value: $row) -> Self {
                Self {
                    id: value.id,
                    block_number: value.block_number,
                    transaction_id: value.transaction_id,
                    domain_id: value.domain_id,
                    owner_id: value.owner_id,
                }
            }
        }

        #[async_graphql::ComplexObject]
        impl $name {
            async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
                domain_by_id(ctx, &self.domain_id).await
            }

            async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
                account_by_id(ctx, &self.owner_id).await
            }
        }
    };
}

impl_domain_owner_event!(
    WrappedTransferEvent,
    "WrappedTransfer",
    WrappedTransferEventRow
);
impl_domain_owner_event!(NameUnwrappedEvent, "NameUnwrapped", NameUnwrappedEventRow);

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameWrapped")]
pub struct NameWrappedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub name: Option<String>,
    pub fuses: i32,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<NameWrappedEventRow> for NameWrappedEvent {
    fn from(value: NameWrappedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            name: value.name,
            fuses: value.fuses,
            expiry_date: value.expiry_date.to_string(),
            domain_id: value.domain_id,
            owner_id: value.owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameWrappedEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.owner_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "FusesSet")]
pub struct FusesSetEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub fuses: i32,
    #[graphql(skip)]
    pub domain_id: String,
}

impl From<FusesSetEventRow> for FusesSetEvent {
    fn from(value: FusesSetEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            fuses: value.fuses,
            domain_id: value.domain_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl FusesSetEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "ExpiryExtended")]
pub struct ExpiryExtendedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub domain_id: String,
}

impl From<ExpiryExtendedEventRow> for ExpiryExtendedEvent {
    fn from(value: ExpiryExtendedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            expiry_date: value.expiry_date.to_string(),
            domain_id: value.domain_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl ExpiryExtendedEvent {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameRegistered")]
pub struct NameRegisteredEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub registration_id: String,
    #[graphql(skip)]
    pub registrant_id: String,
}

impl From<NameRegisteredEventRow> for NameRegisteredEvent {
    fn from(value: NameRegisteredEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            expiry_date: value.expiry_date.to_string(),
            registration_id: value.registration_id,
            registrant_id: value.registrant_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameRegisteredEvent {
    async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        registration_by_id(ctx, &self.registration_id).await
    }

    async fn registrant(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.registrant_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameRenewed")]
pub struct NameRenewedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub registration_id: String,
}

impl From<NameRenewedEventRow> for NameRenewedEvent {
    fn from(value: NameRenewedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            expiry_date: value.expiry_date.to_string(),
            registration_id: value.registration_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameRenewedEvent {
    async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        registration_by_id(ctx, &self.registration_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameTransferred")]
pub struct NameTransferredEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub registration_id: String,
    #[graphql(skip)]
    pub new_owner_id: String,
}

impl From<NameTransferredEventRow> for NameTransferredEvent {
    fn from(value: NameTransferredEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            registration_id: value.registration_id,
            new_owner_id: value.new_owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameTransferredEvent {
    async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        registration_by_id(ctx, &self.registration_id).await
    }

    async fn new_owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.new_owner_id).await
    }
}

macro_rules! impl_resolver_event {
    ($name:ident, $graphql_name:literal, $row:ident, { $($field:ident : $ty:ty => $expr:expr),* $(,)? }) => {
        #[derive(Debug, Clone, SimpleObject)]
        #[graphql(complex, name = $graphql_name)]
        pub struct $name {
            pub id: String,
            #[graphql(name = "blockNumber")]
            pub block_number: i32,
            #[graphql(name = "transactionID")]
            pub transaction_id: String,
            $(pub $field: $ty,)*
            #[graphql(skip)]
            pub resolver_id: String,
        }

        impl From<$row> for $name {
            fn from(value: $row) -> Self {
                Self {
                    id: value.id.clone(),
                    block_number: value.block_number,
                    transaction_id: value.transaction_id.clone(),
                    $($field: $expr(&value),)*
                    resolver_id: value.resolver_id,
                }
            }
        }

        #[async_graphql::ComplexObject]
        impl $name {
            async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
                resolver_by_id(ctx, &self.resolver_id).await
            }
        }
    };
}

impl_resolver_event!(NameChangedEvent, "NameChanged", NameChangedEventRow, { name: String => |v: &NameChangedEventRow| v.name.clone() });
impl_resolver_event!(PubkeyChangedEvent, "PubkeyChanged", PubkeyChangedEventRow, { x: String => |v: &PubkeyChangedEventRow| v.x.clone(), y: String => |v: &PubkeyChangedEventRow| v.y.clone() });
impl_resolver_event!(TextChangedEvent, "TextChanged", TextChangedEventRow, { key: String => |v: &TextChangedEventRow| v.key.clone(), value: Option<String> => |v: &TextChangedEventRow| v.value.clone() });
impl_resolver_event!(ContenthashChangedEvent, "ContenthashChanged", ContenthashChangedEventRow, { hash: String => |v: &ContenthashChangedEventRow| v.hash.clone() });
impl_resolver_event!(VersionChangedEvent, "VersionChanged", VersionChangedEventRow, { version: String => |v: &VersionChangedEventRow| v.version.to_string() });

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "AddrChanged")]
pub struct AddrChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub resolver_id: String,
    #[graphql(skip)]
    pub addr_id: String,
}

impl From<AddrChangedEventRow> for AddrChangedEvent {
    fn from(value: AddrChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            resolver_id: value.resolver_id,
            addr_id: value.addr_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl AddrChangedEvent {
    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }

    async fn addr(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.addr_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "MulticoinAddrChanged")]
pub struct MulticoinAddrChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "coinType")]
    pub coin_type: String,
    pub addr: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<MulticoinAddrChangedEventRow> for MulticoinAddrChangedEvent {
    fn from(value: MulticoinAddrChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            coin_type: value.coin_type.to_string(),
            addr: value.addr,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl MulticoinAddrChangedEvent {
    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "ABIChanged")]
pub struct AbiChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "contentType")]
    pub content_type: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<AbiChangedEventRow> for AbiChangedEvent {
    fn from(value: AbiChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            content_type: value.content_type.to_string(),
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl AbiChangedEvent {
    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "InterfaceChanged")]
pub struct InterfaceChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "interfaceID")]
    pub interface_id: String,
    pub implementer: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<InterfaceChangedEventRow> for InterfaceChangedEvent {
    fn from(value: InterfaceChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            interface_id: value.interface_id,
            implementer: value.implementer,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl InterfaceChangedEvent {
    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "AuthorisationChanged")]
pub struct AuthorisationChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub owner: String,
    pub target: String,
    #[graphql(name = "isAuthorized")]
    pub is_authorized: bool,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<AuthorisationChangedEventRow> for AuthorisationChangedEvent {
    fn from(value: AuthorisationChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            owner: value.owner,
            target: value.target,
            is_authorized: value.is_authorized,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl AuthorisationChangedEvent {
    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

pub async fn hydrate_domain_event(
    storage: &Storage,
    reference: EventReferenceRow,
) -> Result<Option<DomainEvent>> {
    let event = match reference.kind.as_str() {
        "Transfer" => storage
            .events()
            .find_transfer_by_id(&reference.id)
            .await?
            .map(TransferEvent::from)
            .map(DomainEvent::from),
        "NewOwner" => storage
            .events()
            .find_new_owner_by_id(&reference.id)
            .await?
            .map(NewOwnerEvent::from)
            .map(DomainEvent::from),
        "NewResolver" => storage
            .events()
            .find_new_resolver_by_id(&reference.id)
            .await?
            .map(NewResolverEvent::from)
            .map(DomainEvent::from),
        "NewTTL" => storage
            .events()
            .find_new_ttl_by_id(&reference.id)
            .await?
            .map(NewTtlEvent::from)
            .map(DomainEvent::from),
        "WrappedTransfer" => storage
            .events()
            .find_wrapped_transfer_by_id(&reference.id)
            .await?
            .map(WrappedTransferEvent::from)
            .map(DomainEvent::from),
        "NameWrapped" => storage
            .events()
            .find_name_wrapped_by_id(&reference.id)
            .await?
            .map(NameWrappedEvent::from)
            .map(DomainEvent::from),
        "NameUnwrapped" => storage
            .events()
            .find_name_unwrapped_by_id(&reference.id)
            .await?
            .map(NameUnwrappedEvent::from)
            .map(DomainEvent::from),
        "FusesSet" => storage
            .events()
            .find_fuses_set_by_id(&reference.id)
            .await?
            .map(FusesSetEvent::from)
            .map(DomainEvent::from),
        "ExpiryExtended" => storage
            .events()
            .find_expiry_extended_by_id(&reference.id)
            .await?
            .map(ExpiryExtendedEvent::from)
            .map(DomainEvent::from),
        _ => None,
    };
    Ok(event)
}

pub async fn hydrate_registration_event(
    storage: &Storage,
    reference: EventReferenceRow,
) -> Result<Option<RegistrationEvent>> {
    let event = match reference.kind.as_str() {
        "NameRegistered" => storage
            .events()
            .find_name_registered_by_id(&reference.id)
            .await?
            .map(NameRegisteredEvent::from)
            .map(RegistrationEvent::from),
        "NameRenewed" => storage
            .events()
            .find_name_renewed_by_id(&reference.id)
            .await?
            .map(NameRenewedEvent::from)
            .map(RegistrationEvent::from),
        "NameTransferred" => storage
            .events()
            .find_name_transferred_by_id(&reference.id)
            .await?
            .map(NameTransferredEvent::from)
            .map(RegistrationEvent::from),
        _ => None,
    };
    Ok(event)
}

pub async fn hydrate_resolver_event(
    storage: &Storage,
    reference: EventReferenceRow,
) -> Result<Option<ResolverEvent>> {
    let event = match reference.kind.as_str() {
        "AddrChanged" => storage
            .events()
            .find_addr_changed_by_id(&reference.id)
            .await?
            .map(AddrChangedEvent::from)
            .map(ResolverEvent::from),
        "MulticoinAddrChanged" => storage
            .events()
            .find_multicoin_addr_changed_by_id(&reference.id)
            .await?
            .map(MulticoinAddrChangedEvent::from)
            .map(ResolverEvent::from),
        "NameChanged" => storage
            .events()
            .find_name_changed_by_id(&reference.id)
            .await?
            .map(NameChangedEvent::from)
            .map(ResolverEvent::from),
        "AbiChanged" => storage
            .events()
            .find_abi_changed_by_id(&reference.id)
            .await?
            .map(AbiChangedEvent::from)
            .map(ResolverEvent::from),
        "PubkeyChanged" => storage
            .events()
            .find_pubkey_changed_by_id(&reference.id)
            .await?
            .map(PubkeyChangedEvent::from)
            .map(ResolverEvent::from),
        "TextChanged" => storage
            .events()
            .find_text_changed_by_id(&reference.id)
            .await?
            .map(TextChangedEvent::from)
            .map(ResolverEvent::from),
        "ContenthashChanged" => storage
            .events()
            .find_contenthash_changed_by_id(&reference.id)
            .await?
            .map(ContenthashChangedEvent::from)
            .map(ResolverEvent::from),
        "InterfaceChanged" => storage
            .events()
            .find_interface_changed_by_id(&reference.id)
            .await?
            .map(InterfaceChangedEvent::from)
            .map(ResolverEvent::from),
        "AuthorisationChanged" => storage
            .events()
            .find_authorisation_changed_by_id(&reference.id)
            .await?
            .map(AuthorisationChangedEvent::from)
            .map(ResolverEvent::from),
        "VersionChanged" => storage
            .events()
            .find_version_changed_by_id(&reference.id)
            .await?
            .map(VersionChangedEvent::from)
            .map(ResolverEvent::from),
        _ => None,
    };
    Ok(event)
}

async fn account_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Account>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage.accounts().find_by_id(id).await?.map(Into::into))
}

async fn domain_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Domain>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage.domains().find_by_id(id).await?.map(domain_from_row))
}

async fn registration_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Registration>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage
        .registrations()
        .find_by_id(id)
        .await?
        .map(registration_from_row))
}

async fn resolver_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Resolver>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage
        .resolvers()
        .find_by_id(id)
        .await?
        .map(resolver_from_row))
}

fn domain_from_row(row: DomainRow) -> Domain {
    row.into()
}

fn registration_from_row(row: RegistrationRow) -> Registration {
    row.into()
}

fn resolver_from_row(row: ResolverRow) -> Resolver {
    row.into()
}
