use async_graphql::{Context, Result, SimpleObject};
use storage::{Storage, WrappedDomainRow};

use super::{Account, Domain};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct WrappedDomain {
    pub id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    pub fuses: i32,
    pub name: Option<String>,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<WrappedDomainRow> for WrappedDomain {
    fn from(value: WrappedDomainRow) -> Self {
        Self {
            id: value.id,
            expiry_date: value.expiry_date.to_string(),
            fuses: value.fuses,
            owner_id: value.owner_id,
            domain_id: value.domain_id,
            name: value.name,
        }
    }
}

#[async_graphql::ComplexObject]
impl WrappedDomain {
    async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .domains()
            .find_by_id(&self.domain_id)
            .await?
            .map(Into::into))
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let storage = ctx.data::<Storage>()?;
        Ok(storage
            .accounts()
            .find_by_id(&self.owner_id)
            .await?
            .map(Into::into))
    }
}
