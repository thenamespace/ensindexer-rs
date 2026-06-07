use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::{resolve_historical_block, visible_at_block, with_event_block};
use crate::{
    filters::{
        EventFilter, NameRegisteredFilter, NameRegisteredOrderBy, NameRenewedFilter,
        NameRenewedOrderBy, NameTransferredFilter, NameTransferredOrderBy, OrderDirection,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{NameRegisteredEvent, NameRenewedEvent, NameTransferredEvent},
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct RegistrationEventQueries;

#[allow(clippy::too_many_arguments)]
#[Object]
impl RegistrationEventQueries {
    #[graphql(name = "nameRegistered")]
    async fn name_registered_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NameRegisteredEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_name_registered_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "nameRegistereds")]
    async fn name_registered_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameRegisteredFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NameRegisteredOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NameRegisteredEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_name_registered(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
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

    #[graphql(name = "nameRenewed")]
    async fn name_renewed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NameRenewedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_name_renewed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "nameReneweds")]
    async fn name_renewed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameRenewedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NameRenewedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NameRenewedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_name_renewed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
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

    #[graphql(name = "nameTransferred")]
    async fn name_transferred_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NameTransferredEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_name_transferred_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "nameTransferreds")]
    async fn name_transferred_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameTransferredFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NameTransferredOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NameTransferredEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_name_transferred(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
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
