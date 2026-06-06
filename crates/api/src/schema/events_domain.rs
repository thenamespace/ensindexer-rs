use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::ensure_current_block;
use crate::{
    filters::{
        EventFilter, ExpiryExtendedFilter, ExpiryExtendedOrderBy, FusesSetFilter, FusesSetOrderBy,
        NameUnwrappedFilter, NameUnwrappedOrderBy, NameWrappedFilter, NameWrappedOrderBy,
        NewOwnerFilter, NewOwnerOrderBy, NewResolverFilter, NewResolverOrderBy, NewTtlFilter,
        NewTtlOrderBy, OrderDirection, TransferFilter, TransferOrderBy, WrappedTransferFilter,
        WrappedTransferOrderBy,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{
        ExpiryExtendedEvent, FusesSetEvent, NameUnwrappedEvent, NameWrappedEvent, NewOwnerEvent,
        NewResolverEvent, NewTtlEvent, TransferEvent, WrappedTransferEvent,
    },
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct DomainEventQueries;

#[allow(clippy::too_many_arguments)]
#[Object]
impl DomainEventQueries {
    #[graphql(name = "transfer")]
    async fn transfer_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<TransferEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_transfer_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "transfers")]
    async fn transfer_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<TransferFilter>,
        #[graphql(name = "orderBy")] order_by: Option<TransferOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<TransferEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_transfers(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "newOwner")]
    async fn new_owner_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NewOwnerEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_new_owner_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "newOwners")]
    async fn new_owner_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NewOwnerFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NewOwnerOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NewOwnerEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_new_owners(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "newResolver")]
    async fn new_resolver_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NewResolverEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_new_resolver_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "newResolvers")]
    async fn new_resolver_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NewResolverFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NewResolverOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NewResolverEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_new_resolvers(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "newTTL")]
    async fn new_ttl_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NewTtlEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_new_ttl_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "newTTLs")]
    async fn new_ttl_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NewTtlFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NewTtlOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NewTtlEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_new_ttls(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "wrappedTransfer")]
    async fn wrapped_transfer_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<WrappedTransferEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_wrapped_transfer_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "wrappedTransfers")]
    async fn wrapped_transfer_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<WrappedTransferFilter>,
        #[graphql(name = "orderBy")] order_by: Option<WrappedTransferOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<WrappedTransferEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_wrapped_transfers(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "nameWrapped")]
    async fn name_wrapped_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameWrappedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_wrapped_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "nameWrappeds")]
    async fn name_wrapped_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameWrappedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NameWrappedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NameWrappedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_wrapped(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "nameUnwrapped")]
    async fn name_unwrapped_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameUnwrappedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_unwrapped_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "nameUnwrappeds")]
    async fn name_unwrapped_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameUnwrappedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NameUnwrappedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NameUnwrappedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_unwrapped(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "fusesSet")]
    async fn fuses_set_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<FusesSetEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_fuses_set_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "fusesSets")]
    async fn fuses_set_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<FusesSetFilter>,
        #[graphql(name = "orderBy")] order_by: Option<FusesSetOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<FusesSetEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_fuses_set(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "expiryExtended")]
    async fn expiry_extended_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<ExpiryExtendedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_expiry_extended_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "expiryExtendeds")]
    async fn expiry_extended_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<ExpiryExtendedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<ExpiryExtendedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<ExpiryExtendedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_expiry_extended(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
