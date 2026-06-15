use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::super::{resolve_historical_block, visible_at_block, with_event_block};
use crate::{
    filters::{
        EventFilter, ExpiryExtendedFilter, ExpiryExtendedOrderBy, FusesSetFilter, FusesSetOrderBy,
        NameUnwrappedFilter, NameUnwrappedOrderBy, NameWrappedFilter, NameWrappedOrderBy,
        OrderDirection, WrappedTransferFilter, WrappedTransferOrderBy,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{
        ExpiryExtendedEvent, FusesSetEvent, NameUnwrappedEvent, NameWrappedEvent,
        WrappedTransferEvent,
    },
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct WrapperDomainEventQueries;

// Concrete event roots intentionally mirror subgraph pagination/filter/order
// arguments for compatibility with generated GraphQL clients.
#[allow(clippy::too_many_arguments)]
#[Object]
impl WrapperDomainEventQueries {
    #[graphql(name = "wrappedTransfer")]
    async fn wrapped_transfer_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<WrappedTransferEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_wrapped_transfer_by_id(id.as_ref())
                .await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<WrappedTransferEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_wrapped_transfers(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                    block_number,
                ),
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NameWrappedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_name_wrapped_by_id(id.as_ref())
                .await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NameWrappedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_name_wrapped(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                    block_number,
                ),
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NameUnwrappedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_name_unwrapped_by_id(id.as_ref())
                .await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NameUnwrappedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_name_unwrapped(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                    block_number,
                ),
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<FusesSetEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage.events().find_fuses_set_by_id(id.as_ref()).await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<FusesSetEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_fuses_set(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                    block_number,
                ),
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<ExpiryExtendedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_expiry_extended_by_id(id.as_ref())
                .await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<ExpiryExtendedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_expiry_extended(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                    block_number,
                ),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
