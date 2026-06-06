use async_graphql::{Context, ID, Object, Result};
use storage::Storage;

use super::ensure_current_block;
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

#[allow(clippy::too_many_arguments)]
#[Object]
impl EntityQueries {
    async fn domain(
        &self,
        ctx: &Context<'_>,
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Domain>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .find_by_id(id.as_ref())
            .await?
            .map(Into::into))
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
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Account>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .accounts()
            .find_by_id(id.as_ref())
            .await?
            .map(Into::into))
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
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Registration>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .registrations()
            .find_by_id(id.as_ref())
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
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<WrappedDomain>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .wrapped_domains()
            .find_by_id(id.as_ref())
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
        id: ID,
        block: Option<BlockHeight>,
        #[graphql(name = "subgraphError")] _subgraph_error: Option<SubgraphErrorPolicy>,
    ) -> Result<Option<Resolver>> {
        ensure_current_block(block)?;
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .resolvers()
            .find_by_id(id.as_ref())
            .await?
            .map(Into::into))
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
}
