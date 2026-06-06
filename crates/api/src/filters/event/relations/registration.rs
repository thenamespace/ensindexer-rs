use async_graphql::InputObject;

use crate::filters::RegistrationFilter;
use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct RegistrationRelationFilter {
    #[graphql(name = "registration")]
    pub registration: Option<String>,
    #[graphql(name = "registration_not")]
    pub registration_not: Option<String>,
    #[graphql(name = "registration_gt")]
    pub registration_gt: Option<String>,
    #[graphql(name = "registration_lt")]
    pub registration_lt: Option<String>,
    #[graphql(name = "registration_gte")]
    pub registration_gte: Option<String>,
    #[graphql(name = "registration_lte")]
    pub registration_lte: Option<String>,
    #[graphql(name = "registration_in")]
    pub registration_in: Option<Vec<String>>,
    #[graphql(name = "registration_not_in")]
    pub registration_not_in: Option<Vec<String>>,
    #[graphql(name = "registration_contains")]
    pub registration_contains: Option<String>,
    #[graphql(name = "registration_contains_nocase")]
    pub registration_contains_nocase: Option<String>,
    #[graphql(name = "registration_not_contains")]
    pub registration_not_contains: Option<String>,
    #[graphql(name = "registration_not_contains_nocase")]
    pub registration_not_contains_nocase: Option<String>,
    #[graphql(name = "registration_starts_with")]
    pub registration_starts_with: Option<String>,
    #[graphql(name = "registration_starts_with_nocase")]
    pub registration_starts_with_nocase: Option<String>,
    #[graphql(name = "registration_not_starts_with")]
    pub registration_not_starts_with: Option<String>,
    #[graphql(name = "registration_not_starts_with_nocase")]
    pub registration_not_starts_with_nocase: Option<String>,
    #[graphql(name = "registration_ends_with")]
    pub registration_ends_with: Option<String>,
    #[graphql(name = "registration_ends_with_nocase")]
    pub registration_ends_with_nocase: Option<String>,
    #[graphql(name = "registration_not_ends_with")]
    pub registration_not_ends_with: Option<String>,
    #[graphql(name = "registration_not_ends_with_nocase")]
    pub registration_not_ends_with_nocase: Option<String>,
    #[graphql(name = "registration_")]
    pub registration_filter: Option<Box<RegistrationFilter>>,
}

impl ApplyEventFilter for RegistrationRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_id = self.registration;
    }
}
