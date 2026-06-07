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
        filter.parent_id_not = self.resolver_not;
        filter.parent_id_gt = self.resolver_gt;
        filter.parent_id_lt = self.resolver_lt;
        filter.parent_id_gte = self.resolver_gte;
        filter.parent_id_lte = self.resolver_lte;
        filter.parent_id_in = self.resolver_in;
        filter.parent_id_not_in = self.resolver_not_in;
        filter.parent_id_contains = self.resolver_contains;
        filter.parent_id_contains_nocase = self.resolver_contains_nocase;
        filter.parent_id_not_contains = self.resolver_not_contains;
        filter.parent_id_not_contains_nocase = self.resolver_not_contains_nocase;
        filter.parent_id_starts_with = self.resolver_starts_with;
        filter.parent_id_starts_with_nocase = self.resolver_starts_with_nocase;
        filter.parent_id_not_starts_with = self.resolver_not_starts_with;
        filter.parent_id_not_starts_with_nocase = self.resolver_not_starts_with_nocase;
        filter.parent_id_ends_with = self.resolver_ends_with;
        filter.parent_id_ends_with_nocase = self.resolver_ends_with_nocase;
        filter.parent_id_not_ends_with = self.resolver_not_ends_with;
        filter.parent_id_not_ends_with_nocase = self.resolver_not_ends_with_nocase;
        filter.resolver_filter = self.resolver_filter;
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
        filter.resolver_filter = self.resolver_filter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolver_relation_filter_maps_parent_operator_fields() {
        let mut filter = EventFilter::default();
        ResolverRelationFilter {
            resolver_gt: Some("0x1000".into()),
            resolver_not_ends_with: Some("ffff".into()),
            ..ResolverRelationFilter::default()
        }
        .apply(&mut filter);

        assert_eq!(filter.parent_id_gt.as_deref(), Some("0x1000"));
        assert_eq!(filter.parent_id_not_ends_with.as_deref(), Some("ffff"));
    }
}
