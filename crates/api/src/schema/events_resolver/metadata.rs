use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::super::{resolve_historical_block, visible_at_block, with_event_block};
use crate::{
    filters::{
        AbiChangedFilter, AbiChangedOrderBy, AuthorisationChangedFilter,
        AuthorisationChangedOrderBy, ContenthashChangedFilter, ContenthashChangedOrderBy,
        EventFilter, InterfaceChangedFilter, InterfaceChangedOrderBy, OrderDirection,
        PubkeyChangedFilter, PubkeyChangedOrderBy, TextChangedFilter, TextChangedOrderBy,
        VersionChangedFilter, VersionChangedOrderBy,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{
        AbiChangedEvent, AuthorisationChangedEvent, ContenthashChangedEvent, InterfaceChangedEvent,
        PubkeyChangedEvent, TextChangedEvent, VersionChangedEvent,
    },
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct ResolverMetadataEventQueries;

// Concrete event roots intentionally mirror subgraph pagination/filter/order
// arguments for compatibility with generated GraphQL clients.
#[allow(clippy::too_many_arguments)]
#[Object]
impl ResolverMetadataEventQueries {
    #[graphql(name = "abiChanged")]
    async fn abi_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<AbiChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage.events().find_abi_changed_by_id(id.as_ref()).await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "abiChangeds")]
    async fn abi_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<AbiChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<AbiChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<AbiChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_abi_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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

    #[graphql(name = "pubkeyChanged")]
    async fn pubkey_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<PubkeyChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_pubkey_changed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "pubkeyChangeds")]
    async fn pubkey_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<PubkeyChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<PubkeyChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<PubkeyChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_pubkey_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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

    #[graphql(name = "textChanged")]
    async fn text_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<TextChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_text_changed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "textChangeds")]
    async fn text_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<TextChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<TextChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<TextChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_text_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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

    #[graphql(name = "contenthashChanged")]
    async fn contenthash_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<ContenthashChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_contenthash_changed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "contenthashChangeds")]
    async fn contenthash_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<ContenthashChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<ContenthashChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<ContenthashChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_contenthash_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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

    #[graphql(name = "interfaceChanged")]
    async fn interface_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<InterfaceChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_interface_changed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "interfaceChangeds")]
    async fn interface_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<InterfaceChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<InterfaceChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<InterfaceChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_interface_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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

    #[graphql(name = "authorisationChanged")]
    async fn authorisation_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<AuthorisationChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_authorisation_changed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "authorisationChangeds")]
    async fn authorisation_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<AuthorisationChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<AuthorisationChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<AuthorisationChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_authorisation_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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

    #[graphql(name = "versionChanged")]
    async fn version_changed_event(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<VersionChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(visible_at_block(
            storage
                .events()
                .find_version_changed_by_id(id.as_ref())
                .await?,
            block_number,
        )
        .map(Into::into))
    }

    #[graphql(name = "versionChangeds")]
    async fn version_changed_events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<VersionChangedFilter>,
        #[graphql(name = "orderBy")] order_by: Option<VersionChangedOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<VersionChangedEvent>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        Ok(storage
            .events()
            .list_version_changed(
                normalize_first(first),
                normalize_skip(skip),
                with_event_block(
                    EventFilter::from(filter.unwrap_or_default()).into_resolver_filter(),
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
