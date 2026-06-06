use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::ensure_current_block;
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
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<DomainEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_domain_event_refs(
                1,
                0,
                storage::EventFilter {
                    id: Some(id.to_string()),
                    ..storage::EventFilter::default()
                },
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
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<DomainEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_domain_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_domain_filter(),
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
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<RegistrationEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_registration_event_refs(
                1,
                0,
                storage::EventFilter {
                    id: Some(id.to_string()),
                    ..storage::EventFilter::default()
                },
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
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<RegistrationEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_registration_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_registration_filter(),
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
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<ResolverEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_resolver_event_refs(
                1,
                0,
                storage::EventFilter {
                    id: Some(id.to_string()),
                    ..storage::EventFilter::default()
                },
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
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<ResolverEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_resolver_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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
