use async_graphql::InputObject;

use crate::filters::ResolverFilter;
use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ResolverRelationFilter {
    #[graphql(name = "resolver")]
    pub resolver: Option<String>,
    #[graphql(name = "resolver_not")]
    pub resolver_not: Option<String>,
    #[graphql(name = "resolver_gt")]
    pub resolver_gt: Option<String>,
    #[graphql(name = "resolver_lt")]
    pub resolver_lt: Option<String>,
    #[graphql(name = "resolver_gte")]
    pub resolver_gte: Option<String>,
    #[graphql(name = "resolver_lte")]
    pub resolver_lte: Option<String>,
    #[graphql(name = "resolver_in")]
    pub resolver_in: Option<Vec<String>>,
    #[graphql(name = "resolver_not_in")]
    pub resolver_not_in: Option<Vec<String>>,
    #[graphql(name = "resolver_contains")]
    pub resolver_contains: Option<String>,
    #[graphql(name = "resolver_contains_nocase")]
    pub resolver_contains_nocase: Option<String>,
    #[graphql(name = "resolver_not_contains")]
    pub resolver_not_contains: Option<String>,
    #[graphql(name = "resolver_not_contains_nocase")]
    pub resolver_not_contains_nocase: Option<String>,
    #[graphql(name = "resolver_starts_with")]
    pub resolver_starts_with: Option<String>,
    #[graphql(name = "resolver_starts_with_nocase")]
    pub resolver_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_starts_with")]
    pub resolver_not_starts_with: Option<String>,
    #[graphql(name = "resolver_not_starts_with_nocase")]
    pub resolver_not_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_ends_with")]
    pub resolver_ends_with: Option<String>,
    #[graphql(name = "resolver_ends_with_nocase")]
    pub resolver_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_ends_with")]
    pub resolver_not_ends_with: Option<String>,
    #[graphql(name = "resolver_not_ends_with_nocase")]
    pub resolver_not_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_")]
    pub resolver_filter: Option<Box<ResolverFilter>>,
}

impl ApplyEventFilter for ResolverRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_id = self.resolver;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct NewResolverRelationFilter {
    #[graphql(name = "resolver")]
    pub resolver: Option<String>,
    #[graphql(name = "resolver_not")]
    pub resolver_not: Option<String>,
    #[graphql(name = "resolver_gt")]
    pub resolver_gt: Option<String>,
    #[graphql(name = "resolver_lt")]
    pub resolver_lt: Option<String>,
    #[graphql(name = "resolver_gte")]
    pub resolver_gte: Option<String>,
    #[graphql(name = "resolver_lte")]
    pub resolver_lte: Option<String>,
    #[graphql(name = "resolver_in")]
    pub resolver_in: Option<Vec<String>>,
    #[graphql(name = "resolver_not_in")]
    pub resolver_not_in: Option<Vec<String>>,
    #[graphql(name = "resolver_contains")]
    pub resolver_contains: Option<String>,
    #[graphql(name = "resolver_contains_nocase")]
    pub resolver_contains_nocase: Option<String>,
    #[graphql(name = "resolver_not_contains")]
    pub resolver_not_contains: Option<String>,
    #[graphql(name = "resolver_not_contains_nocase")]
    pub resolver_not_contains_nocase: Option<String>,
    #[graphql(name = "resolver_starts_with")]
    pub resolver_starts_with: Option<String>,
    #[graphql(name = "resolver_starts_with_nocase")]
    pub resolver_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_starts_with")]
    pub resolver_not_starts_with: Option<String>,
    #[graphql(name = "resolver_not_starts_with_nocase")]
    pub resolver_not_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_ends_with")]
    pub resolver_ends_with: Option<String>,
    #[graphql(name = "resolver_ends_with_nocase")]
    pub resolver_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_ends_with")]
    pub resolver_not_ends_with: Option<String>,
    #[graphql(name = "resolver_not_ends_with_nocase")]
    pub resolver_not_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_")]
    pub resolver_filter: Option<Box<ResolverFilter>>,
}

impl ApplyEventFilter for NewResolverRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.resolver_id = self.resolver;
    }
}
