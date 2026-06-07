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
        filter.parent_id_not = self.registration_not;
        filter.parent_id_gt = self.registration_gt;
        filter.parent_id_lt = self.registration_lt;
        filter.parent_id_gte = self.registration_gte;
        filter.parent_id_lte = self.registration_lte;
        filter.parent_id_in = self.registration_in;
        filter.parent_id_not_in = self.registration_not_in;
        filter.parent_id_contains = self.registration_contains;
        filter.parent_id_contains_nocase = self.registration_contains_nocase;
        filter.parent_id_not_contains = self.registration_not_contains;
        filter.parent_id_not_contains_nocase = self.registration_not_contains_nocase;
        filter.parent_id_starts_with = self.registration_starts_with;
        filter.parent_id_starts_with_nocase = self.registration_starts_with_nocase;
        filter.parent_id_not_starts_with = self.registration_not_starts_with;
        filter.parent_id_not_starts_with_nocase = self.registration_not_starts_with_nocase;
        filter.parent_id_ends_with = self.registration_ends_with;
        filter.parent_id_ends_with_nocase = self.registration_ends_with_nocase;
        filter.parent_id_not_ends_with = self.registration_not_ends_with;
        filter.parent_id_not_ends_with_nocase = self.registration_not_ends_with_nocase;
        filter.registration_filter = self.registration_filter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_relation_filter_maps_nested_filter() {
        let mut filter = EventFilter::default();
        let mut nested = RegistrationFilter::default();
        nested.label_name = Some("vitalik".into());
        RegistrationRelationFilter {
            registration: Some("registration-id".into()),
            registration_in: Some(vec!["registration-a".into(), "registration-b".into()]),
            registration_filter: Some(Box::new(nested)),
            ..RegistrationRelationFilter::default()
        }
        .apply(&mut filter);

        assert_eq!(filter.parent_id.as_deref(), Some("registration-id"));
        assert_eq!(
            filter.parent_id_in,
            Some(vec!["registration-a".into(), "registration-b".into()])
        );
        assert_eq!(
            filter
                .registration_filter
                .as_ref()
                .and_then(|filter| filter.label_name.as_deref()),
            Some("vitalik")
        );
    }
}
