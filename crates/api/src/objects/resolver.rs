use async_graphql::{Context, Result, SimpleObject};
use storage::{ResolverRow, Storage};

use super::{Account, Domain, ResolverEvent, hydrate_resolver_event};
use crate::pagination::{normalize_first, normalize_skip};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Resolver {
    pub id: String,
    pub address: String,
    pub addr: Option<Account>,
    #[graphql(name = "contentHash")]
    pub content_hash: Option<String>,
    pub texts: Vec<String>,
    #[graphql(name = "coinTypes")]
    pub coin_types: Vec<String>,
    #[graphql(skip)]
    pub domain_id: Option<String>,
    #[graphql(skip)]
    pub addr_id: Option<String>,
}

impl From<ResolverRow> for Resolver {
    fn from(value: ResolverRow) -> Self {
        Self {
            id: value.id,
            address: value.address,
            addr: value.addr_id.clone().map(|id| Account { id }),
            content_hash: value.content_hash,
            texts: value.texts,
            coin_types: value
                .coin_types
                .into_iter()
                .map(|value| value.to_string())
                .collect(),
            domain_id: value.domain_id,
            addr_id: value.addr_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl Resolver {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        let Some(domain_id) = self.domain_id.as_ref() else {
            return Ok(None);
        };
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .find_by_id(domain_id)
            .await?
            .map(Into::into))
    }

    async fn events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
    ) -> Result<Vec<ResolverEvent>> {
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_resolver_event_refs(
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
            if let Some(event) = hydrate_resolver_event(storage, reference).await? {
                events.push(event);
            }
        }
        Ok(events)
    }
}
