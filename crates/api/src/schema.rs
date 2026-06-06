use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Result, Schema};
use storage::Storage;

use crate::{
    filters::{
        AccountFilter, AccountOrderBy, DomainFilter, DomainOrderBy, EventFilter, EventOrderBy,
        OrderDirection, RegistrationFilter, RegistrationOrderBy, ResolverFilter, ResolverOrderBy,
        WrappedDomainFilter, WrappedDomainOrderBy,
    },
    meta::{BlockHeight, Meta, MetaBlock, SubgraphErrorPolicy},
    objects::*,
    pagination::{normalize_first, normalize_skip},
};

pub type EnsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(storage: Storage) -> EnsSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(storage)
        .finish()
}

#[derive(Default)]
pub struct QueryRoot;

#[allow(clippy::too_many_arguments)]
#[Object]
impl QueryRoot {
    #[graphql(name = "_meta")]
    async fn meta(&self, ctx: &Context<'_>, block: Option<BlockHeight>) -> Result<Meta> {
        let storage = ctx.data::<Storage>()?;
        let block_row = match block.unwrap_or_default() {
            BlockHeight {
                hash: Some(hash), ..
            } => storage.blocks().find_by_hash(&hash).await?,
            BlockHeight {
                number: Some(number),
                ..
            } => storage.blocks().find_by_number(number.into()).await?,
            BlockHeight {
                number_gte: Some(number),
                ..
            } => {
                storage
                    .blocks()
                    .find_latest_at_or_after(number.into())
                    .await?
            }
            BlockHeight { .. } => storage.blocks().find_latest().await?,
        };

        let block = match block_row {
            Some(block) => MetaBlock::try_from(block)?,
            None => MetaBlock {
                hash: None,
                number: 0,
                timestamp: None,
            },
        };

        Ok(Meta {
            block,
            deployment: "local-rust-ens-indexer".to_owned(),
            has_indexing_errors: false,
        })
    }

    async fn domain(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Domain>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage.domains().find_by_id(&id).await?.map(Into::into))
    }

    async fn domains(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<DomainFilter>,
        #[graphql(name = "orderBy")] order_by: Option<DomainOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<Domain>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .list_filtered(
                normalize_first(first),
                normalize_skip(skip),
                filter.map(Into::into).unwrap_or_default(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn account(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Account>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage.accounts().find_by_id(&id).await?.map(Into::into))
    }

    async fn accounts(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<AccountFilter>,
        #[graphql(name = "orderBy")] order_by: Option<AccountOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<Account>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .accounts()
            .list(
                normalize_first(first),
                normalize_skip(skip),
                filter.map(Into::into).unwrap_or_default(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn registration(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Registration>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .registrations()
            .find_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn registrations(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<RegistrationFilter>,
        #[graphql(name = "orderBy")] order_by: Option<RegistrationOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<Registration>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .registrations()
            .list_filtered(
                normalize_first(first),
                normalize_skip(skip),
                filter.map(Into::into).unwrap_or_default(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn wrapped_domain(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<WrappedDomain>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .wrapped_domains()
            .find_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn wrapped_domains(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<WrappedDomainFilter>,
        #[graphql(name = "orderBy")] order_by: Option<WrappedDomainOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<WrappedDomain>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .wrapped_domains()
            .list(
                normalize_first(first),
                normalize_skip(skip),
                filter.map(Into::into).unwrap_or_default(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn resolver(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Resolver>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage.resolvers().find_by_id(&id).await?.map(Into::into))
    }

    async fn resolvers(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<ResolverFilter>,
        #[graphql(name = "orderBy")] order_by: Option<ResolverOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<Resolver>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .resolvers()
            .list_filtered(
                normalize_first(first),
                normalize_skip(skip),
                filter.map(Into::into).unwrap_or_default(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn domain_event(
        &self,
        ctx: &Context<'_>,
        id: String,
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
                    id: Some(id),
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
        #[graphql(name = "where")] filter: Option<EventFilter>,
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
                filter.unwrap_or_default().into_domain_filter(),
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
        id: String,
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
                    id: Some(id),
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
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_registration_filter(),
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
        id: String,
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
                    id: Some(id),
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
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_resolver_filter(),
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

    async fn transfer_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<TransferEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_transfer_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn transfer_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn new_owner_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NewOwnerEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_new_owner_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn new_owner_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn new_resolver_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NewResolverEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_new_resolver_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn new_resolver_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn new_ttl_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NewTtlEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_new_ttl_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn new_ttl_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

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

    async fn name_registered_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
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
                filter.unwrap_or_default().into_registration_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

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

    async fn name_renewed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
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
                filter.unwrap_or_default().into_registration_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

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

    async fn name_transferred_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
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
                filter.unwrap_or_default().into_registration_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn wrapped_transfer_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<WrappedTransferEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_wrapped_transfer_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn wrapped_transfer_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn name_wrapped_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameWrappedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_wrapped_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn name_wrapped_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn name_unwrapped_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameUnwrappedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_unwrapped_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn name_unwrapped_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn fuses_set_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<FusesSetEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_fuses_set_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn fuses_set_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn expiry_extended_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<ExpiryExtendedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_expiry_extended_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn expiry_extended_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
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
                filter.unwrap_or_default().into_domain_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn addr_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<AddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_addr_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn addr_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<AddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_addr_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn multicoin_addr_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<MulticoinAddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_multicoin_addr_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn multicoin_addr_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<MulticoinAddrChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_multicoin_addr_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn name_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<NameChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_name_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn name_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<NameChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_name_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn abi_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<AbiChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_abi_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn abi_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<AbiChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_abi_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn pubkey_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<PubkeyChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_pubkey_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn pubkey_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<PubkeyChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_pubkey_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn text_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<TextChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_text_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn text_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<TextChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_text_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn contenthash_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<ContenthashChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_contenthash_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn contenthash_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<ContenthashChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_contenthash_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn interface_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<InterfaceChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_interface_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn interface_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<InterfaceChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_interface_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn authorisation_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<AuthorisationChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_authorisation_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn authorisation_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<AuthorisationChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_authorisation_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn version_changed_event(
        &self,
        ctx: &Context<'_>,
        id: String,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<VersionChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .find_version_changed_by_id(&id)
            .await?
            .map(Into::into))
    }

    async fn version_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<EventFilter>,
        #[graphql(name = "orderBy")] order_by: Option<EventOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Vec<VersionChangedEvent>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .events()
            .list_version_changed(
                normalize_first(first),
                normalize_skip(skip),
                filter.unwrap_or_default().into_resolver_filter(),
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

fn ensure_current_block(block: Option<BlockHeight>) -> Result<()> {
    if block.as_ref().is_none_or(BlockHeight::is_current) {
        Ok(())
    } else {
        Err(async_graphql::Error::new(
            "historical block queries are not implemented yet",
        ))
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};

    use super::QueryRoot;

    #[test]
    fn core_queries_expose_graph_node_compatibility_arguments() {
        let sdl = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .finish()
            .sdl();

        assert!(sdl.contains(
            "domain(id: String!, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"
        ));
        assert!(sdl.contains("domains(first: Int, skip: Int, where: DomainFilter, orderBy: DomainOrderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"));
        assert!(sdl.contains(
            "domainEvent(id: String!, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"
        ));
        assert!(sdl.contains("domainEvents(first: Int, skip: Int, where: EventFilter, orderBy: EventOrderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"));
        assert!(sdl.contains(
            "transferEvent(id: String!, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"
        ));
        assert!(sdl.contains("transferEvents(first: Int, skip: Int, where: EventFilter, orderBy: EventOrderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"));
        assert!(sdl.contains(
            "versionChangedEvent(id: String!, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"
        ));
        assert!(sdl.contains("versionChangedEvents(first: Int, skip: Int, where: EventFilter, orderBy: EventOrderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_)"));
        assert!(sdl.contains("enum _SubgraphErrorPolicy_"));
    }
}
