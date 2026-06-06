use async_graphql::{Context, Result, SimpleObject};
use storage::{RegistrationRow, Storage};

use super::{Account, Domain, RegistrationEvent, hydrate_registration_event};
use crate::pagination::{normalize_first, normalize_skip};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Registration {
    pub id: String,
    #[graphql(name = "registrationDate")]
    pub registration_date: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    pub cost: Option<String>,
    #[graphql(name = "labelName")]
    pub label_name: Option<String>,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub registrant_id: String,
}

impl From<RegistrationRow> for Registration {
    fn from(value: RegistrationRow) -> Self {
        Self {
            id: value.id,
            registration_date: value.registration_date.to_string(),
            expiry_date: value.expiry_date.to_string(),
            cost: value.cost.map(|value| value.to_string()),
            label_name: value.label_name,
            domain_id: value.domain_id,
            registrant_id: value.registrant_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl Registration {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .find_by_id(&self.domain_id)
            .await?
            .map(Into::into))
    }

    async fn registrant(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .accounts()
            .find_by_id(&self.registrant_id)
            .await?
            .map(Into::into))
    }

    async fn events(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
    ) -> Result<Vec<RegistrationEvent>> {
        let storage = ctx.data::<Storage>()?;
        let refs = storage
            .events()
            .list_registration_event_refs(
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
            if let Some(event) = hydrate_registration_event(storage, reference).await? {
                events.push(event);
            }
        }
        Ok(events)
    }
}
