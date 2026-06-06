use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct NameFieldFilter {
    #[graphql(name = "name")]
    pub name: Option<String>,
    #[graphql(name = "name_not")]
    pub name_not: Option<String>,
    #[graphql(name = "name_gt")]
    pub name_gt: Option<String>,
    #[graphql(name = "name_lt")]
    pub name_lt: Option<String>,
    #[graphql(name = "name_gte")]
    pub name_gte: Option<String>,
    #[graphql(name = "name_lte")]
    pub name_lte: Option<String>,
    #[graphql(name = "name_in")]
    pub name_in: Option<Vec<String>>,
    #[graphql(name = "name_not_in")]
    pub name_not_in: Option<Vec<String>>,
    #[graphql(name = "name_contains")]
    pub name_contains: Option<String>,
    #[graphql(name = "name_contains_nocase")]
    pub name_contains_nocase: Option<String>,
    #[graphql(name = "name_not_contains")]
    pub name_not_contains: Option<String>,
    #[graphql(name = "name_not_contains_nocase")]
    pub name_not_contains_nocase: Option<String>,
    #[graphql(name = "name_starts_with")]
    pub name_starts_with: Option<String>,
    #[graphql(name = "name_starts_with_nocase")]
    pub name_starts_with_nocase: Option<String>,
    #[graphql(name = "name_not_starts_with")]
    pub name_not_starts_with: Option<String>,
    #[graphql(name = "name_not_starts_with_nocase")]
    pub name_not_starts_with_nocase: Option<String>,
    #[graphql(name = "name_ends_with")]
    pub name_ends_with: Option<String>,
    #[graphql(name = "name_ends_with_nocase")]
    pub name_ends_with_nocase: Option<String>,
    #[graphql(name = "name_not_ends_with")]
    pub name_not_ends_with: Option<String>,
    #[graphql(name = "name_not_ends_with_nocase")]
    pub name_not_ends_with_nocase: Option<String>,
}

impl ApplyEventFilter for NameFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.name = self.name;
        filter.name_contains = self.name_contains;
        filter.name_contains_nocase = self.name_contains_nocase;
    }
}
