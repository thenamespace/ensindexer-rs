mod registry;
mod wrapper;

use async_graphql::MergedObject;

use self::{registry::RegistryDomainEventQueries, wrapper::WrapperDomainEventQueries};

#[derive(Default, MergedObject)]
pub(crate) struct DomainEventQueries(RegistryDomainEventQueries, WrapperDomainEventQueries);
