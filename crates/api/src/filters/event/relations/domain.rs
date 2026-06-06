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
    }
}
