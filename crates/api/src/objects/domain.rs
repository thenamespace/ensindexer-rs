use async_graphql::{Context, Error, Result, SimpleObject, dataloader::DataLoader};
use storage::{DomainRow, Storage};

use super::{Account, DomainEvent, Registration, Resolver, WrappedDomain, hydrate_domain_event};
use crate::filters::{DomainFilter, DomainOrderBy, OrderDirection};
use crate::loaders::{
    AccountKey, EntityLoader, RegistrationByDomainKey, ResolverKey, WrappedDomainByDomainKey,
};
use crate::pagination::{normalize_first, normalize_skip};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Domain {
    pub id: String,
    pub name: Option<String>,
    #[graphql(name = "labelName")]
    pub label_name: Option<String>,
    pub labelhash: Option<String>,
    #[graphql(name = "subdomainCount")]
    pub subdomain_count: i32,
    pub ttl: Option<String>,
    #[graphql(name = "isMigrated")]
    pub is_migrated: bool,
    #[graphql(name = "createdAt")]
    pub created_at: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: Option<String>,
    #[graphql(skip)]
    pub owner_id: String,
    #[graphql(skip)]
    pub parent_id: Option<String>,
    #[graphql(skip)]
    pub resolved_address_id: Option<String>,
    #[graphql(skip)]
    pub resolver_id: Option<String>,
    #[graphql(skip)]
    pub registrant_id: Option<String>,
    #[graphql(skip)]
    pub wrapped_owner_id: Option<String>,
}

impl From<DomainRow> for Domain {
    fn from(value: DomainRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            label_name: value.label_name,
            labelhash: value.labelhash,
            subdomain_count: value.subdomain_count,
            ttl: value.ttl.map(|v| v.to_string()),
            is_migrated: value.is_migrated,
            created_at: value.created_at.to_string(),
            expiry_date: value.expiry_date.map(|v| v.to_string()),
            owner_id: value.owner_id,
            parent_id: value.parent_id,
            resolved_address_id: value.resolved_address_id,
            resolver_id: value.resolver_id,
            registrant_id: value.registrant_id,
            wrapped_owner_id: value.wrapped_owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl Domain {
    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        let Some(parent_id) = self.parent_id.as_ref() else {
            return Ok(None);
        };
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .find_by_id(parent_id)
            .await?
            .map(Into::into))
    }

    async fn subdomains(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
        #[graphql(name = "where")] filter: Option<DomainFilter>,
        #[graphql(name = "orderBy")] order_by: Option<DomainOrderBy>,
        #[graphql(name = "orderDirection")] order_direction: Option<OrderDirection>,
    ) -> Result<Vec<Domain>> {
        let storage = ctx.data::<Storage>()?;
        let mut filter = filter.map(storage::DomainFilter::from).unwrap_or_default();
        filter.parent_id = Some(self.id.clone());
        Ok(storage
            .domains()
            .list_filtered(
                normalize_first(first),
                normalize_skip(skip),
                filter,
                order_by.unwrap_or_default().into(),
                order_direction.unwrap_or_default().into(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn resolved_address(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let Some(account_id) = self.resolved_address_id.as_ref() else {
            return Ok(None);
        };
        load_account(ctx, account_id).await
    }

    async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        let Some(resolver_id) = self.resolver_id.as_ref() else {
            return Ok(None);
        };
        Ok(ctx
            .data::<DataLoader<EntityLoader>>()?
            .load_one(ResolverKey(resolver_id.clone()))
            .await
            .map_err(Error::new)?
            .map(Into::into))
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        load_account(ctx, &self.owner_id).await
    }

    async fn registrant(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let Some(registrant_id) = self.registrant_id.as_ref() else {
            return Ok(None);
        };
        load_account(ctx, registrant_id).await
    }

    async fn wrapped_owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let Some(wrapped_owner_id) = self.wrapped_owner_id.as_ref() else {
            return Ok(None);
        };
        load_account(ctx, wrapped_owner_id).await
    }

    async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        Ok(ctx
            .data::<DataLoader<EntityLoader>>()?
            .load_one(RegistrationByDomainKey(self.id.clone()))
            .await
            .map_err(Error::new)?
            .map(Into::into))
    }

    async fn wrapped_domain(&self, ctx: &Context<'_>) -> Result<Option<WrappedDomain>> {
        Ok(ctx
            .data::<DataLoader<EntityLoader>>()?
            .load_one(WrappedDomainByDomainKey(self.id.clone()))
            .await
            .map_err(Error::new)?
            .map(Into::into))
    }

    async fn events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
    ) -> Result<Vec<DomainEvent>> {
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_domain_event_refs(
                normalize_first(first),
                normalize_skip(skip),
                storage::EventFilter {
                    parent_id: Some(self.id.clone()),
                    ..storage::EventFilter::default()
                },
                storage::EventOrderField::BlockNumber,
                storage::OrderDirection::Asc,
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
}

async fn load_account(ctx: &Context<'_>, id: &str) -> Result<Option<Account>> {
    Ok(ctx
        .data::<DataLoader<EntityLoader>>()?
        .load_one(AccountKey(id.to_owned()))
        .await
        .map_err(Error::new)?
        .map(Into::into))
}
