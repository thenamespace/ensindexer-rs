use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::super::ensure_current_block;
use crate::{
    filters::{
        AddrChangedFilter, AddrChangedOrderBy, EventFilter, MulticoinAddrChangedFilter,
        MulticoinAddrChangedOrderBy, NameChangedFilter, NameChangedOrderBy, OrderDirection,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{AddrChangedEvent, MulticoinAddrChangedEvent, NameChangedEvent},
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct ResolverRecordEventQueries;

#[allow(clippy::too_many_arguments)]
#[Object]
impl ResolverRecordEventQueries {
    #[graphql(name = "addrChanged")]
    async fn addr_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<AddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_addr_changed_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "addrChangeds")]
    async fn addr_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<AddrChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<AddrChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<AddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_addr_changed(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "multicoinAddrChanged")]
    async fn multicoin_addr_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<MulticoinAddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_multicoin_addr_changed_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "multicoinAddrChangeds")]
    async fn multicoin_addr_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<MulticoinAddrChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<MulticoinAddrChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<MulticoinAddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_multicoin_addr_changed(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    #[graphql(name = "nameChanged")]
    async fn name_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<NameChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_changed_by_id(id.as_ref())
            .await?
            .map(Into::into))
    }

    #[graphql(name = "nameChangeds")]
    async fn name_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<NameChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<NameChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<NameChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_changed(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
