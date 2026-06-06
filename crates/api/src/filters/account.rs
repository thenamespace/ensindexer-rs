use async_graphql::InputObject;
use storage::AccountFilter as StorageAccountFilter;

#[derive(Debug, Clone, InputObject, Default)]
pub struct AccountFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    pub and: Option<Vec<AccountFilter>>,
    pub or: Option<Vec<AccountFilter>>,
}

impl From<AccountFilter> for StorageAccountFilter {
    fn from(value: AccountFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            and: value
                .and
                .map(|filters| filters.into_iter().map(Into::into).collect()),
            or: value
                .or
                .map(|filters| filters.into_iter().map(Into::into).collect()),
        }
    }
}
