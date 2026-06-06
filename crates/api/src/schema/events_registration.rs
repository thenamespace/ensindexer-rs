use async_graphql::{Context, Object, Result};
use storage::Storage;

use super::ensure_current_block;
use crate::{
    filters::{
        EventFilter, EventOrderBy, NameRegisteredFilter, NameRenewedFilter, NameTransferredFilter,
        OrderDirection,
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
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameRegisteredEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_registered_by_id(&id)
            .await?
            .map(Into::into))
    }

    #[graphql(name = "nameRegistereds")]
    async fn name_registered_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameRegisteredFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NameRegisteredEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_registered(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
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
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameRenewedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_renewed_by_id(&id)
            .await?
            .map(Into::into))
    }

    #[graphql(name = "nameReneweds")]
    async fn name_renewed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameRenewedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NameRenewedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_renewed(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
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
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameTransferredEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_transferred_by_id(&id)
            .await?
            .map(Into::into))
    }

    #[graphql(name = "nameTransferreds")]
    async fn name_transferred_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameTransferredFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NameTransferredEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_transferred(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
