use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct KeyFieldFilter {
    #[graphql(name = "key")]
    pub key: Option<String>,
    #[graphql(name = "key_not")]
    pub key_not: Option<String>,
    #[graphql(name = "key_gt")]
    pub key_gt: Option<String>,
    #[graphql(name = "key_lt")]
    pub key_lt: Option<String>,
    #[graphql(name = "key_gte")]
    pub key_gte: Option<String>,
    #[graphql(name = "key_lte")]
    pub key_lte: Option<String>,
    #[graphql(name = "key_in")]
    pub key_in: Option<Vec<String>>,
    #[graphql(name = "key_not_in")]
    pub key_not_in: Option<Vec<String>>,
    #[graphql(name = "key_contains")]
    pub key_contains: Option<String>,
    #[graphql(name = "key_contains_nocase")]
    pub key_contains_nocase: Option<String>,
    #[graphql(name = "key_not_contains")]
    pub key_not_contains: Option<String>,
    #[graphql(name = "key_not_contains_nocase")]
    pub key_not_contains_nocase: Option<String>,
    #[graphql(name = "key_starts_with")]
    pub key_starts_with: Option<String>,
    #[graphql(name = "key_starts_with_nocase")]
    pub key_starts_with_nocase: Option<String>,
    #[graphql(name = "key_not_starts_with")]
    pub key_not_starts_with: Option<String>,
    #[graphql(name = "key_not_starts_with_nocase")]
    pub key_not_starts_with_nocase: Option<String>,
    #[graphql(name = "key_ends_with")]
    pub key_ends_with: Option<String>,
    #[graphql(name = "key_ends_with_nocase")]
    pub key_ends_with_nocase: Option<String>,
    #[graphql(name = "key_not_ends_with")]
    pub key_not_ends_with: Option<String>,
    #[graphql(name = "key_not_ends_with_nocase")]
    pub key_not_ends_with_nocase: Option<String>,
}

impl ApplyEventFilter for KeyFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.key = self.key;
        filter.key_contains = self.key_contains;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ValueFieldFilter {
    #[graphql(name = "value")]
    pub value: Option<String>,
    #[graphql(name = "value_not")]
    pub value_not: Option<String>,
    #[graphql(name = "value_gt")]
    pub value_gt: Option<String>,
    #[graphql(name = "value_lt")]
    pub value_lt: Option<String>,
    #[graphql(name = "value_gte")]
    pub value_gte: Option<String>,
    #[graphql(name = "value_lte")]
    pub value_lte: Option<String>,
    #[graphql(name = "value_in")]
    pub value_in: Option<Vec<String>>,
    #[graphql(name = "value_not_in")]
    pub value_not_in: Option<Vec<String>>,
    #[graphql(name = "value_contains")]
    pub value_contains: Option<String>,
    #[graphql(name = "value_contains_nocase")]
    pub value_contains_nocase: Option<String>,
    #[graphql(name = "value_not_contains")]
    pub value_not_contains: Option<String>,
    #[graphql(name = "value_not_contains_nocase")]
    pub value_not_contains_nocase: Option<String>,
    #[graphql(name = "value_starts_with")]
    pub value_starts_with: Option<String>,
    #[graphql(name = "value_starts_with_nocase")]
    pub value_starts_with_nocase: Option<String>,
    #[graphql(name = "value_not_starts_with")]
    pub value_not_starts_with: Option<String>,
    #[graphql(name = "value_not_starts_with_nocase")]
    pub value_not_starts_with_nocase: Option<String>,
    #[graphql(name = "value_ends_with")]
    pub value_ends_with: Option<String>,
    #[graphql(name = "value_ends_with_nocase")]
    pub value_ends_with_nocase: Option<String>,
    #[graphql(name = "value_not_ends_with")]
    pub value_not_ends_with: Option<String>,
    #[graphql(name = "value_not_ends_with_nocase")]
    pub value_not_ends_with_nocase: Option<String>,
}

impl ApplyEventFilter for ValueFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.value = self.value;
        filter.value_contains = self.value_contains;
    }
}
