use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::resolve_historical_block;
use crate::{
    filters::{
        AccountFilter, AccountOrderBy, DomainFilter, DomainOrderBy, OrderDirection,
        RegistrationFilter, RegistrationOrderBy, ResolverFilter, ResolverOrderBy,
        WrappedDomainFilter, WrappedDomainOrderBy,
    },
    meta::{BlockHeight, SubgraphErrorPolicy},
    objects::{Account, Domain, Registration, Resolver, WrappedDomain},
    pagination::{normalize_first, normalize_skip},
};

#[derive(Default)]
pub(crate) struct EntityQueries;

// GraphQL root fields mirror The Graph's generated schema arguments, so these
// resolver methods intentionally carry pagination, filters, ordering, and block args.
#[allow(clippy::too_many_arguments)]
#[Object]
impl EntityQueries {
    async fn domain(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<Domain>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let row = match block_number {
            Some(block_number) => {
                storage
                    .domains()
                    .find_by_id_at_block(id.as_ref(), block_number)
                    .await?
            }
            None => storage.domains().find_by_id(id.as_ref()).await?,
        };
        Ok(row.map(Into::into))
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<Domain>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let filter = filter.map(Into::into).unwrap_or_default();
        let rows = match block_number {
            Some(block_number) => {
                storage
                    .domains()
                    .list_filtered_at_block(
                        block_number,
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
            None => {
                storage
                    .domains()
                    .list_filtered(
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn account(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<Account>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let row = match block_number {
            Some(block_number) => {
                storage
                    .accounts()
                    .find_by_id_at_block(id.as_ref(), block_number)
                    .await?
            }
            None => storage.accounts().find_by_id(id.as_ref()).await?,
        };
        Ok(row.map(Into::into))
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<Account>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let filter = filter.map(Into::into).unwrap_or_default();
        let rows = match block_number {
            Some(block_number) => {
                storage
                    .accounts()
                    .list_at_block(
                        block_number,
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
            None => {
                storage
                    .accounts()
                    .list(
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn registration(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<Registration>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let row = match block_number {
            Some(block_number) => {
                storage
                    .registrations()
                    .find_by_id_at_block(id.as_ref(), block_number)
                    .await?
            }
            None => storage.registrations().find_by_id(id.as_ref()).await?,
        };
        Ok(row.map(Into::into))
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<Registration>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let filter = filter.map(Into::into).unwrap_or_default();
        let rows = match block_number {
            Some(block_number) => {
                storage
                    .registrations()
                    .list_filtered_at_block(
                        block_number,
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
            None => {
                storage
                    .registrations()
                    .list_filtered(
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn wrapped_domain(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<WrappedDomain>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let row = match block_number {
            Some(block_number) => {
                storage
                    .wrapped_domains()
                    .find_by_id_at_block(id.as_ref(), block_number)
                    .await?
            }
            None => storage.wrapped_domains().find_by_id(id.as_ref()).await?,
        };
        Ok(row.map(Into::into))
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<WrappedDomain>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let filter = filter.map(Into::into).unwrap_or_default();
        let rows = match block_number {
            Some(block_number) => {
                storage
                    .wrapped_domains()
                    .list_at_block(
                        block_number,
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
            None => {
                storage
                    .wrapped_domains()
                    .list(
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn resolver(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Option<Resolver>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let row = match block_number {
            Some(block_number) => {
                storage
                    .resolvers()
                    .find_by_id_at_block(id.as_ref(), block_number)
                    .await?
            }
            None => storage.resolvers().find_by_id(id.as_ref()).await?,
        };
        Ok(row.map(Into::into))
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
        #[graphql(name = "subgraphError", default)] _subgraph_error: SubgraphErrorPolicy,
    ) -> Result<Vec<Resolver>> {
        let storage = ctx.data::<Storage>()?;
        let block_number = resolve_historical_block(storage, block).await?;
        let filter = filter.map(Into::into).unwrap_or_default();
        let rows = match block_number {
            Some(block_number) => {
                storage
                    .resolvers()
                    .list_filtered_at_block(
                        block_number,
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
            None => {
                storage
                    .resolvers()
                    .list_filtered(
                        normalize_first(first),
                        normalize_skip(skip),
                        filter,
                        order_by.unwrap_or_default().into(),
                        order_direction.unwrap_or_default().into(),
                    )
                    .await?
            }
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }
}
