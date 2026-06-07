use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::{resolve_historical_block, with_event_block};
use crate::{
    filters::{
        DomainEventFilter, EventFilter, EventOrderBy, OrderDirection, RegistrationEventFilter,
        RegistrationEventOrderBy, ResolverEventFilter, ResolverEventOrderBy,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{
        DomainEvent, RegistrationEvent, ResolverEvent, hydrate_domain_event,
        hydrate_registration_event, hydrate_resolver_event,
    },
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct InterfaceEventQueries;

#[allow(clippy::too_many_arguments)]
#[Object]
impl InterfaceEventQueries {
    async fn domain_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<DomainEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let refs = storage
            .events()
            .list_domain_event_refs(
                1,
                0,
                with_event_block(
                    storage::EventFilter {
                        id: Some(id.to_string()),
                        ..storage::EventFilter::default()
                    },
                    block_number,
                ),
                storage::EventOrderField::Id,
                storage::OrderDirection::Asc,
            )
            .await?;
        let Some(reference) = refs.into_iter().next() else {
            return Ok(None);
        };
        hydrate_domain_event(storage, reference).await
    }

    async fn domain_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<DomainEventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<DomainEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let refs = storage
            .events()
            .list_domain_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
                    block_number,
                ),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?;
        let mut events = Vec::with_capacity(refs.len());
        for reference in refs {
            if let Some(event) = hydrate_domain_event(storage, reference).await? {
                events.push(event);
            }
        }
        Ok(events)
    }

    async fn registration_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<RegistrationEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let refs = storage
            .events()
            .list_registration_event_refs(
                1,
                0,
                with_event_block(
                    storage::EventFilter {
                        id: Some(id.to_string()),
                        ..storage::EventFilter::default()
                    },
                    block_number,
                ),
                storage::EventOrderField::Id,
                storage::OrderDirection::Asc,
            )
            .await?;
        let Some(reference) = refs.into_iter().next() else {
            return Ok(None);
        };
        hydrate_registration_event(storage, reference).await
    }

    async fn registration_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<RegistrationEventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<RegistrationEventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<RegistrationEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let refs = storage
            .events()
            .list_registration_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
                    block_number,
                ),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?;
        let mut events = Vec::with_capacity(refs.len());
        for reference in refs {
            if let Some(event) = hydrate_registration_event(storage, reference).await? {
                events.push(event);
            }
        }
        Ok(events)
    }

    async fn resolver_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<ResolverEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let refs = storage
            .events()
            .list_resolver_event_refs(
                1,
                0,
                with_event_block(
                    storage::EventFilter {
                        id: Some(id.to_string()),
                        ..storage::EventFilter::default()
                    },
                    block_number,
                ),
                storage::EventOrderField::Id,
                storage::OrderDirection::Asc,
            )
            .await?;
        let Some(reference) = refs.into_iter().next() else {
            return Ok(None);
        };
        hydrate_resolver_event(storage, reference).await
    }

    async fn resolver_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<ResolverEventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<ResolverEventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<ResolverEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let refs = storage
            .events()
            .list_resolver_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
                    block_number,
                ),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?;
        let mut events = Vec::with_capacity(refs.len());
        for reference in refs {
            if let Some(event) = hydrate_resolver_event(storage, reference).await? {
                events.push(event);
            }
        }
        Ok(events)
    }
}
