use async_graphql::{Context, Result, SimpleObject};
use storage::{
    AccountRow, DomainFilter as StorageDomainFilter, DomainOrderField,
    OrderDirection as StorageOrderDirection, RegistrationFilter as StorageRegistrationFilter,
    RegistrationOrderField, Storage, WrappedDomainFilter as StorageWrappedDomainFilter,
    WrappedDomainOrderField,
};

use super::{Domain, Registration, WrappedDomain};
use crate::pagination::{normalize_first, normalize_skip};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Account {
    pub id: String,
}

impl From<AccountRow> for Account {
    fn from(value: AccountRow) -> Self {
        Self { id: value.id }
    }
}

#[async_graphql::ComplexObject]
impl Account {
    async fn domains(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
    ) -> Result<Vec<Domain>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .list_filtered(
                normalize_first(first),
                normalize_skip(skip),
                StorageDomainFilter {
                    owner_id: Some(self.id.clone()),
                    ..StorageDomainFilter::default()
                },
                DomainOrderField::Id,
                StorageOrderDirection::Asc,
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn registrations(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
    ) -> Result<Vec<Registration>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .registrations()
            .list_filtered(
                normalize_first(first),
                normalize_skip(skip),
                StorageRegistrationFilter {
                    registrant_id: Some(self.id.clone()),
                    ..StorageRegistrationFilter::default()
                },
                RegistrationOrderField::Id,
                StorageOrderDirection::Asc,
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn wrapped_domains(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        skip: Option<i32>,
    ) -> Result<Vec<WrappedDomain>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .wrapped_domains()
            .list(
                normalize_first(first),
                normalize_skip(skip),
                StorageWrappedDomainFilter {
                    owner_id: Some(self.id.clone()),
                    ..StorageWrappedDomainFilter::default()
                },
                WrappedDomainOrderField::Id,
                StorageOrderDirection::Asc,
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
