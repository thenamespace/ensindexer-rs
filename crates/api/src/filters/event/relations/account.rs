use async_graphql::InputObject;

use crate::filters::AccountFilter;
use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct OwnerRelationFilter {
    #[graphql(name = "owner")]
    pub owner: Option<String>,
    #[graphql(name = "owner_not")]
    pub owner_not: Option<String>,
    #[graphql(name = "owner_gt")]
    pub owner_gt: Option<String>,
    #[graphql(name = "owner_lt")]
    pub owner_lt: Option<String>,
    #[graphql(name = "owner_gte")]
    pub owner_gte: Option<String>,
    #[graphql(name = "owner_lte")]
    pub owner_lte: Option<String>,
    #[graphql(name = "owner_in")]
    pub owner_in: Option<Vec<String>>,
    #[graphql(name = "owner_not_in")]
    pub owner_not_in: Option<Vec<String>>,
    #[graphql(name = "owner_contains")]
    pub owner_contains: Option<String>,
    #[graphql(name = "owner_contains_nocase")]
    pub owner_contains_nocase: Option<String>,
    #[graphql(name = "owner_not_contains")]
    pub owner_not_contains: Option<String>,
    #[graphql(name = "owner_not_contains_nocase")]
    pub owner_not_contains_nocase: Option<String>,
    #[graphql(name = "owner_starts_with")]
    pub owner_starts_with: Option<String>,
    #[graphql(name = "owner_starts_with_nocase")]
    pub owner_starts_with_nocase: Option<String>,
    #[graphql(name = "owner_not_starts_with")]
    pub owner_not_starts_with: Option<String>,
    #[graphql(name = "owner_not_starts_with_nocase")]
    pub owner_not_starts_with_nocase: Option<String>,
    #[graphql(name = "owner_ends_with")]
    pub owner_ends_with: Option<String>,
    #[graphql(name = "owner_ends_with_nocase")]
    pub owner_ends_with_nocase: Option<String>,
    #[graphql(name = "owner_not_ends_with")]
    pub owner_not_ends_with: Option<String>,
    #[graphql(name = "owner_not_ends_with_nocase")]
    pub owner_not_ends_with_nocase: Option<String>,
    #[graphql(name = "owner_")]
    pub owner_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for OwnerRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.owner_id = self.owner;
        filter.owner_id_not = self.owner_not;
        filter.owner_id_gt = self.owner_gt;
        filter.owner_id_lt = self.owner_lt;
        filter.owner_id_gte = self.owner_gte;
        filter.owner_id_lte = self.owner_lte;
        filter.owner_id_in = self.owner_in;
        filter.owner_id_not_in = self.owner_not_in;
        filter.owner_id_contains = self.owner_contains;
        filter.owner_id_not_contains = self.owner_not_contains;
        filter.owner_filter = self.owner_filter;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct RegistrantRelationFilter {
    #[graphql(name = "registrant")]
    pub registrant: Option<String>,
    #[graphql(name = "registrant_not")]
    pub registrant_not: Option<String>,
    #[graphql(name = "registrant_gt")]
    pub registrant_gt: Option<String>,
    #[graphql(name = "registrant_lt")]
    pub registrant_lt: Option<String>,
    #[graphql(name = "registrant_gte")]
    pub registrant_gte: Option<String>,
    #[graphql(name = "registrant_lte")]
    pub registrant_lte: Option<String>,
    #[graphql(name = "registrant_in")]
    pub registrant_in: Option<Vec<String>>,
    #[graphql(name = "registrant_not_in")]
    pub registrant_not_in: Option<Vec<String>>,
    #[graphql(name = "registrant_contains")]
    pub registrant_contains: Option<String>,
    #[graphql(name = "registrant_contains_nocase")]
    pub registrant_contains_nocase: Option<String>,
    #[graphql(name = "registrant_not_contains")]
    pub registrant_not_contains: Option<String>,
    #[graphql(name = "registrant_not_contains_nocase")]
    pub registrant_not_contains_nocase: Option<String>,
    #[graphql(name = "registrant_starts_with")]
    pub registrant_starts_with: Option<String>,
    #[graphql(name = "registrant_starts_with_nocase")]
    pub registrant_starts_with_nocase: Option<String>,
    #[graphql(name = "registrant_not_starts_with")]
    pub registrant_not_starts_with: Option<String>,
    #[graphql(name = "registrant_not_starts_with_nocase")]
    pub registrant_not_starts_with_nocase: Option<String>,
    #[graphql(name = "registrant_ends_with")]
    pub registrant_ends_with: Option<String>,
    #[graphql(name = "registrant_ends_with_nocase")]
    pub registrant_ends_with_nocase: Option<String>,
    #[graphql(name = "registrant_not_ends_with")]
    pub registrant_not_ends_with: Option<String>,
    #[graphql(name = "registrant_not_ends_with_nocase")]
    pub registrant_not_ends_with_nocase: Option<String>,
    #[graphql(name = "registrant_")]
    pub registrant_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for RegistrantRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.registrant_id = self.registrant;
        filter.registrant_id_ops.not = self.registrant_not;
        filter.registrant_id_ops.gt = self.registrant_gt;
        filter.registrant_id_ops.lt = self.registrant_lt;
        filter.registrant_id_ops.gte = self.registrant_gte;
        filter.registrant_id_ops.lte = self.registrant_lte;
        filter.registrant_id_ops.in_values = self.registrant_in;
        filter.registrant_id_ops.not_in = self.registrant_not_in;
        filter.registrant_id_ops.contains = self.registrant_contains;
        filter.registrant_id_ops.contains_nocase = self.registrant_contains_nocase;
        filter.registrant_id_ops.not_contains = self.registrant_not_contains;
        filter.registrant_id_ops.not_contains_nocase = self.registrant_not_contains_nocase;
        filter.registrant_id_ops.starts_with = self.registrant_starts_with;
        filter.registrant_id_ops.starts_with_nocase = self.registrant_starts_with_nocase;
        filter.registrant_id_ops.not_starts_with = self.registrant_not_starts_with;
        filter.registrant_id_ops.not_starts_with_nocase = self.registrant_not_starts_with_nocase;
        filter.registrant_id_ops.ends_with = self.registrant_ends_with;
        filter.registrant_id_ops.ends_with_nocase = self.registrant_ends_with_nocase;
        filter.registrant_id_ops.not_ends_with = self.registrant_not_ends_with;
        filter.registrant_id_ops.not_ends_with_nocase = self.registrant_not_ends_with_nocase;
        filter.registrant_filter = self.registrant_filter;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct NewOwnerRelationFilter {
    #[graphql(name = "newOwner")]
    pub new_owner: Option<String>,
    #[graphql(name = "newOwner_not")]
    pub new_owner_not: Option<String>,
    #[graphql(name = "newOwner_gt")]
    pub new_owner_gt: Option<String>,
    #[graphql(name = "newOwner_lt")]
    pub new_owner_lt: Option<String>,
    #[graphql(name = "newOwner_gte")]
    pub new_owner_gte: Option<String>,
    #[graphql(name = "newOwner_lte")]
    pub new_owner_lte: Option<String>,
    #[graphql(name = "newOwner_in")]
    pub new_owner_in: Option<Vec<String>>,
    #[graphql(name = "newOwner_not_in")]
    pub new_owner_not_in: Option<Vec<String>>,
    #[graphql(name = "newOwner_contains")]
    pub new_owner_contains: Option<String>,
    #[graphql(name = "newOwner_contains_nocase")]
    pub new_owner_contains_nocase: Option<String>,
    #[graphql(name = "newOwner_not_contains")]
    pub new_owner_not_contains: Option<String>,
    #[graphql(name = "newOwner_not_contains_nocase")]
    pub new_owner_not_contains_nocase: Option<String>,
    #[graphql(name = "newOwner_starts_with")]
    pub new_owner_starts_with: Option<String>,
    #[graphql(name = "newOwner_starts_with_nocase")]
    pub new_owner_starts_with_nocase: Option<String>,
    #[graphql(name = "newOwner_not_starts_with")]
    pub new_owner_not_starts_with: Option<String>,
    #[graphql(name = "newOwner_not_starts_with_nocase")]
    pub new_owner_not_starts_with_nocase: Option<String>,
    #[graphql(name = "newOwner_ends_with")]
    pub new_owner_ends_with: Option<String>,
    #[graphql(name = "newOwner_ends_with_nocase")]
    pub new_owner_ends_with_nocase: Option<String>,
    #[graphql(name = "newOwner_not_ends_with")]
    pub new_owner_not_ends_with: Option<String>,
    #[graphql(name = "newOwner_not_ends_with_nocase")]
    pub new_owner_not_ends_with_nocase: Option<String>,
    #[graphql(name = "newOwner_")]
    pub new_owner_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for NewOwnerRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.new_owner_id = self.new_owner;
        filter.new_owner_id_ops.not = self.new_owner_not;
        filter.new_owner_id_ops.gt = self.new_owner_gt;
        filter.new_owner_id_ops.lt = self.new_owner_lt;
        filter.new_owner_id_ops.gte = self.new_owner_gte;
        filter.new_owner_id_ops.lte = self.new_owner_lte;
        filter.new_owner_id_ops.in_values = self.new_owner_in;
        filter.new_owner_id_ops.not_in = self.new_owner_not_in;
        filter.new_owner_id_ops.contains = self.new_owner_contains;
        filter.new_owner_id_ops.contains_nocase = self.new_owner_contains_nocase;
        filter.new_owner_id_ops.not_contains = self.new_owner_not_contains;
        filter.new_owner_id_ops.not_contains_nocase = self.new_owner_not_contains_nocase;
        filter.new_owner_id_ops.starts_with = self.new_owner_starts_with;
        filter.new_owner_id_ops.starts_with_nocase = self.new_owner_starts_with_nocase;
        filter.new_owner_id_ops.not_starts_with = self.new_owner_not_starts_with;
        filter.new_owner_id_ops.not_starts_with_nocase = self.new_owner_not_starts_with_nocase;
        filter.new_owner_id_ops.ends_with = self.new_owner_ends_with;
        filter.new_owner_id_ops.ends_with_nocase = self.new_owner_ends_with_nocase;
        filter.new_owner_id_ops.not_ends_with = self.new_owner_not_ends_with;
        filter.new_owner_id_ops.not_ends_with_nocase = self.new_owner_not_ends_with_nocase;
        filter.new_owner_filter = self.new_owner_filter;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct AddrAccountRelationFilter {
    #[graphql(name = "addr")]
    pub addr: Option<String>,
    #[graphql(name = "addr_not")]
    pub addr_not: Option<String>,
    #[graphql(name = "addr_gt")]
    pub addr_gt: Option<String>,
    #[graphql(name = "addr_lt")]
    pub addr_lt: Option<String>,
    #[graphql(name = "addr_gte")]
    pub addr_gte: Option<String>,
    #[graphql(name = "addr_lte")]
    pub addr_lte: Option<String>,
    #[graphql(name = "addr_in")]
    pub addr_in: Option<Vec<String>>,
    #[graphql(name = "addr_not_in")]
    pub addr_not_in: Option<Vec<String>>,
    #[graphql(name = "addr_contains")]
    pub addr_contains: Option<String>,
    #[graphql(name = "addr_contains_nocase")]
    pub addr_contains_nocase: Option<String>,
    #[graphql(name = "addr_not_contains")]
    pub addr_not_contains: Option<String>,
    #[graphql(name = "addr_not_contains_nocase")]
    pub addr_not_contains_nocase: Option<String>,
    #[graphql(name = "addr_starts_with")]
    pub addr_starts_with: Option<String>,
    #[graphql(name = "addr_starts_with_nocase")]
    pub addr_starts_with_nocase: Option<String>,
    #[graphql(name = "addr_not_starts_with")]
    pub addr_not_starts_with: Option<String>,
    #[graphql(name = "addr_not_starts_with_nocase")]
    pub addr_not_starts_with_nocase: Option<String>,
    #[graphql(name = "addr_ends_with")]
    pub addr_ends_with: Option<String>,
    #[graphql(name = "addr_ends_with_nocase")]
    pub addr_ends_with_nocase: Option<String>,
    #[graphql(name = "addr_not_ends_with")]
    pub addr_not_ends_with: Option<String>,
    #[graphql(name = "addr_not_ends_with_nocase")]
    pub addr_not_ends_with_nocase: Option<String>,
    #[graphql(name = "addr_")]
    pub addr_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for AddrAccountRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.addr_id = self.addr;
        filter.addr_id_not = self.addr_not;
        filter.addr_id_gt = self.addr_gt;
        filter.addr_id_lt = self.addr_lt;
        filter.addr_id_gte = self.addr_gte;
        filter.addr_id_lte = self.addr_lte;
        filter.addr_id_in = self.addr_in;
        filter.addr_id_not_in = self.addr_not_in;
        filter.addr_id_contains = self.addr_contains;
        filter.addr_id_not_contains = self.addr_not_contains;
        filter.addr_filter = self.addr_filter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_relation_filters_map_operator_fields() {
        let mut filter = EventFilter::default();
        OwnerRelationFilter {
            owner_not: Some("0xold".into()),
            owner_in: Some(vec!["0xone".into(), "0xtwo".into()]),
            owner_contains: Some("abcd".into()),
            owner_filter: Some(Box::new(AccountFilter {
                id: Some("0xowner".into()),
                ..AccountFilter::default()
            })),
            ..OwnerRelationFilter::default()
        }
        .apply(&mut filter);
        AddrAccountRelationFilter {
            addr_gt: Some("0x1000".into()),
            addr_not_contains: Some("ffff".into()),
            addr_filter: Some(Box::new(AccountFilter {
                id: Some("0xaddr".into()),
                ..AccountFilter::default()
            })),
            ..AddrAccountRelationFilter::default()
        }
        .apply(&mut filter);

        assert_eq!(filter.owner_id_not.as_deref(), Some("0xold"));
        assert_eq!(
            filter.owner_id_in,
            Some(vec!["0xone".into(), "0xtwo".into()])
        );
        assert_eq!(filter.owner_id_contains.as_deref(), Some("abcd"));
        assert_eq!(filter.addr_id_gt.as_deref(), Some("0x1000"));
        assert_eq!(filter.addr_id_not_contains.as_deref(), Some("ffff"));
        assert_eq!(
            filter
                .owner_filter
                .as_ref()
                .and_then(|filter| filter.id.as_deref()),
            Some("0xowner")
        );
        assert_eq!(
            filter
                .addr_filter
                .as_ref()
                .and_then(|filter| filter.id.as_deref()),
            Some("0xaddr")
        );
    }

    #[test]
    fn registration_account_relation_filters_map_event_column_operator_fields() {
        let mut filter = EventFilter::default();
        RegistrantRelationFilter {
            registrant_not: Some("0xold".into()),
            registrant_contains_nocase: Some("abcd".into()),
            registrant_not_ends_with: Some("ffff".into()),
            ..RegistrantRelationFilter::default()
        }
        .apply(&mut filter);
        NewOwnerRelationFilter {
            new_owner_gt: Some("0x1000".into()),
            new_owner_starts_with_nocase: Some("0xabc".into()),
            new_owner_not_in: Some(vec!["0xdead".into()]),
            ..NewOwnerRelationFilter::default()
        }
        .apply(&mut filter);

        assert_eq!(filter.registrant_id_ops.not.as_deref(), Some("0xold"));
        assert_eq!(
            filter.registrant_id_ops.contains_nocase.as_deref(),
            Some("abcd")
        );
        assert_eq!(
            filter.registrant_id_ops.not_ends_with.as_deref(),
            Some("ffff")
        );
        assert_eq!(filter.new_owner_id_ops.gt.as_deref(), Some("0x1000"));
        assert_eq!(
            filter.new_owner_id_ops.starts_with_nocase.as_deref(),
            Some("0xabc")
        );
        assert_eq!(filter.new_owner_id_ops.not_in, Some(vec!["0xdead".into()]));
    }
}
