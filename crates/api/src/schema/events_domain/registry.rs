use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::super::{resolve_historical_block, visible_at_block, with_event_block};
use crate::{
    filters::{
        EventFilter, NewOwnerFilter, NewOwnerOrderBy, NewResolverFilter, NewResolverOrderBy,
        NewTtlFilter, NewTtlOrderBy, OrderDirection, TransferFilter, TransferOrderBy,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{NewOwnerEvent, NewResolverEvent, NewTtlEvent, TransferEvent},
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct RegistryDomainEventQueries;

// Concrete event roots intentionally mirror subgraph pagination/filter/order
// arguments for compatibility with generated GraphQL clients.
#[allow(clippy::too_many_arguments)]
#[Object]
impl RegistryDomainEventQueries {
    #[graphql(name = "transfer")]
    async fn transfer_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<TransferEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage.events().find_transfer_by_id(id.as_ref()).await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<TransferEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_transfers(
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

    #[graphql(name = "newOwner")]
    async fn new_owner_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NewOwnerEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage.events().find_new_owner_by_id(id.as_ref()).await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NewOwnerEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_new_owners(
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

    #[graphql(name = "newResolver")]
    async fn new_resolver_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NewResolverEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_new_resolver_by_id(id.as_ref())
                .await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NewResolverEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_new_resolvers(
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

    #[graphql(name = "newTTL")]
    async fn new_ttl_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NewTtlEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage.events().find_new_ttl_by_id(id.as_ref()).await?,
            block_number,
        )
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NewTtlEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_new_ttls(
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
