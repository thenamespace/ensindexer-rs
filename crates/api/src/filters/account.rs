use async_graphql::InputObject;
use storage::AccountFilter as StorageAccountFilter;

use super::{BlockChangedFilter, DomainFilter, RegistrationFilter, WrappedDomainFilter};

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "Account_filter")]
pub struct AccountFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_gt")]
    pub id_gt: Option<String>,
    #[graphql(name = "id_lt")]
    pub id_lt: Option<String>,
    #[graphql(name = "id_gte")]
    pub id_gte: Option<String>,
    #[graphql(name = "id_lte")]
    pub id_lte: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    #[graphql(name = "domains_")]
    pub domains_filter: Option<Box<DomainFilter>>,
    #[graphql(name = "wrappedDomains_")]
    pub wrapped_domains_filter: Option<Box<WrappedDomainFilter>>,
    #[graphql(name = "registrations_")]
    pub registrations_filter: Option<Box<RegistrationFilter>>,
    #[graphql(name = "_change_block")]
    pub change_block: Option<BlockChangedFilter>,
    pub and: Option<Vec<AccountFilter>>,
    pub or: Option<Vec<AccountFilter>>,
}

impl From<AccountFilter> for StorageAccountFilter {
    fn from(value: AccountFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_gt: value.id_gt,
            id_lt: value.id_lt,
            id_gte: value.id_gte,
            id_lte: value.id_lte,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            change_block_number_gte: value.change_block.and_then(|change| change.number_gte),
            and: value
                .and
                .map(|filters| filters.into_iter().map(Into::into).collect()),
            or: value
                .or
                .map(|filters| filters.into_iter().map(Into::into).collect()),
        }
    }
}
