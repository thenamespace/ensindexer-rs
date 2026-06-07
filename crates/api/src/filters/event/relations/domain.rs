use async_graphql::InputObject;

use crate::filters::DomainFilter;
use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct DomainRelationFilter {
    #[graphql(name = "domain")]
    pub domain: Option<String>,
    #[graphql(name = "domain_not")]
    pub domain_not: Option<String>,
    #[graphql(name = "domain_gt")]
    pub domain_gt: Option<String>,
    #[graphql(name = "domain_lt")]
    pub domain_lt: Option<String>,
    #[graphql(name = "domain_gte")]
    pub domain_gte: Option<String>,
    #[graphql(name = "domain_lte")]
    pub domain_lte: Option<String>,
    #[graphql(name = "domain_in")]
    pub domain_in: Option<Vec<String>>,
    #[graphql(name = "domain_not_in")]
    pub domain_not_in: Option<Vec<String>>,
    #[graphql(name = "domain_contains")]
    pub domain_contains: Option<String>,
    #[graphql(name = "domain_contains_nocase")]
    pub domain_contains_nocase: Option<String>,
    #[graphql(name = "domain_not_contains")]
    pub domain_not_contains: Option<String>,
    #[graphql(name = "domain_not_contains_nocase")]
    pub domain_not_contains_nocase: Option<String>,
    #[graphql(name = "domain_starts_with")]
    pub domain_starts_with: Option<String>,
    #[graphql(name = "domain_starts_with_nocase")]
    pub domain_starts_with_nocase: Option<String>,
    #[graphql(name = "domain_not_starts_with")]
    pub domain_not_starts_with: Option<String>,
    #[graphql(name = "domain_not_starts_with_nocase")]
    pub domain_not_starts_with_nocase: Option<String>,
    #[graphql(name = "domain_ends_with")]
    pub domain_ends_with: Option<String>,
    #[graphql(name = "domain_ends_with_nocase")]
    pub domain_ends_with_nocase: Option<String>,
    #[graphql(name = "domain_not_ends_with")]
    pub domain_not_ends_with: Option<String>,
    #[graphql(name = "domain_not_ends_with_nocase")]
    pub domain_not_ends_with_nocase: Option<String>,
    #[graphql(name = "domain_")]
    pub domain_filter: Option<Box<DomainFilter>>,
}

impl ApplyEventFilter for DomainRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_id = self.domain;
        filter.parent_id_not = self.domain_not;
        filter.parent_id_gt = self.domain_gt;
        filter.parent_id_lt = self.domain_lt;
        filter.parent_id_gte = self.domain_gte;
        filter.parent_id_lte = self.domain_lte;
        filter.parent_id_in = self.domain_in;
        filter.parent_id_not_in = self.domain_not_in;
        filter.parent_id_contains = self.domain_contains;
        filter.parent_id_contains_nocase = self.domain_contains_nocase;
        filter.parent_id_not_contains = self.domain_not_contains;
        filter.parent_id_not_contains_nocase = self.domain_not_contains_nocase;
        filter.parent_id_starts_with = self.domain_starts_with;
        filter.parent_id_starts_with_nocase = self.domain_starts_with_nocase;
        filter.parent_id_not_starts_with = self.domain_not_starts_with;
        filter.parent_id_not_starts_with_nocase = self.domain_not_starts_with_nocase;
        filter.parent_id_ends_with = self.domain_ends_with;
        filter.parent_id_ends_with_nocase = self.domain_ends_with_nocase;
        filter.parent_id_not_ends_with = self.domain_not_ends_with;
        filter.parent_id_not_ends_with_nocase = self.domain_not_ends_with_nocase;
        filter.domain_filter = self.domain_filter;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ParentDomainRelationFilter {
    #[graphql(name = "parentDomain")]
    pub parent_domain: Option<String>,
    #[graphql(name = "parentDomain_not")]
    pub parent_domain_not: Option<String>,
    #[graphql(name = "parentDomain_gt")]
    pub parent_domain_gt: Option<String>,
    #[graphql(name = "parentDomain_lt")]
    pub parent_domain_lt: Option<String>,
    #[graphql(name = "parentDomain_gte")]
    pub parent_domain_gte: Option<String>,
    #[graphql(name = "parentDomain_lte")]
    pub parent_domain_lte: Option<String>,
    #[graphql(name = "parentDomain_in")]
    pub parent_domain_in: Option<Vec<String>>,
    #[graphql(name = "parentDomain_not_in")]
    pub parent_domain_not_in: Option<Vec<String>>,
    #[graphql(name = "parentDomain_contains")]
    pub parent_domain_contains: Option<String>,
    #[graphql(name = "parentDomain_contains_nocase")]
    pub parent_domain_contains_nocase: Option<String>,
    #[graphql(name = "parentDomain_not_contains")]
    pub parent_domain_not_contains: Option<String>,
    #[graphql(name = "parentDomain_not_contains_nocase")]
    pub parent_domain_not_contains_nocase: Option<String>,
    #[graphql(name = "parentDomain_starts_with")]
    pub parent_domain_starts_with: Option<String>,
    #[graphql(name = "parentDomain_starts_with_nocase")]
    pub parent_domain_starts_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_not_starts_with")]
    pub parent_domain_not_starts_with: Option<String>,
    #[graphql(name = "parentDomain_not_starts_with_nocase")]
    pub parent_domain_not_starts_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_ends_with")]
    pub parent_domain_ends_with: Option<String>,
    #[graphql(name = "parentDomain_ends_with_nocase")]
    pub parent_domain_ends_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_not_ends_with")]
    pub parent_domain_not_ends_with: Option<String>,
    #[graphql(name = "parentDomain_not_ends_with_nocase")]
    pub parent_domain_not_ends_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_")]
    pub parent_domain_filter: Option<Box<DomainFilter>>,
}

impl ApplyEventFilter for ParentDomainRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_domain_id = self.parent_domain;
        filter.parent_domain_id_ops.not = self.parent_domain_not;
        filter.parent_domain_id_ops.gt = self.parent_domain_gt;
        filter.parent_domain_id_ops.lt = self.parent_domain_lt;
        filter.parent_domain_id_ops.gte = self.parent_domain_gte;
        filter.parent_domain_id_ops.lte = self.parent_domain_lte;
        filter.parent_domain_id_ops.in_values = self.parent_domain_in;
        filter.parent_domain_id_ops.not_in = self.parent_domain_not_in;
        filter.parent_domain_id_ops.contains = self.parent_domain_contains;
        filter.parent_domain_id_ops.contains_nocase = self.parent_domain_contains_nocase;
        filter.parent_domain_id_ops.not_contains = self.parent_domain_not_contains;
        filter.parent_domain_id_ops.not_contains_nocase = self.parent_domain_not_contains_nocase;
        filter.parent_domain_id_ops.starts_with = self.parent_domain_starts_with;
        filter.parent_domain_id_ops.starts_with_nocase = self.parent_domain_starts_with_nocase;
        filter.parent_domain_id_ops.not_starts_with = self.parent_domain_not_starts_with;
        filter.parent_domain_id_ops.not_starts_with_nocase =
            self.parent_domain_not_starts_with_nocase;
        filter.parent_domain_id_ops.ends_with = self.parent_domain_ends_with;
        filter.parent_domain_id_ops.ends_with_nocase = self.parent_domain_ends_with_nocase;
        filter.parent_domain_id_ops.not_ends_with = self.parent_domain_not_ends_with;
        filter.parent_domain_id_ops.not_ends_with_nocase = self.parent_domain_not_ends_with_nocase;
        filter.parent_domain_filter = self.parent_domain_filter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_relation_filter_maps_parent_operator_fields() {
        let mut filter = EventFilter::default();
        DomainRelationFilter {
            domain_not: Some("0xold".into()),
            domain_contains_nocase: Some("abcd".into()),
            ..DomainRelationFilter::default()
        }
        .apply(&mut filter);

        assert_eq!(filter.parent_id_not.as_deref(), Some("0xold"));
        assert_eq!(filter.parent_id_contains_nocase.as_deref(), Some("abcd"));
    }

    #[test]
    fn parent_domain_relation_filter_maps_event_column_operator_fields() {
        let mut filter = EventFilter::default();
        ParentDomainRelationFilter {
            parent_domain_not: Some("0xold".into()),
            parent_domain_contains_nocase: Some("abcd".into()),
            parent_domain_not_ends_with: Some("ffff".into()),
            ..ParentDomainRelationFilter::default()
        }
        .apply(&mut filter);

        assert_eq!(filter.parent_domain_id_ops.not.as_deref(), Some("0xold"));
        assert_eq!(
            filter.parent_domain_id_ops.contains_nocase.as_deref(),
            Some("abcd")
        );
        assert_eq!(
            filter.parent_domain_id_ops.not_ends_with.as_deref(),
            Some("ffff")
        );
    }
}
