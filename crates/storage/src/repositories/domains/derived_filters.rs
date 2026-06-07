use sqlx::{Postgres, query_builder::Separated};

use crate::{
    filters::{DomainFilter, EventFilter, RegistrationFilter, WrappedDomainFilter},
    query::{
        domain_filter_has_conditions, push_domain_scalar_filter_conditions, push_where_prefix,
    },
    repositories::{
        events::push_domain_events_filter,
        registrations::{push_registration_subquery_filters, registration_filter_has_conditions},
        wrapped_domains::{
            push_wrapped_domain_subquery_filters, wrapped_domain_filter_has_conditions,
        },
    },
};

pub(super) fn push_domain_derived_relation_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    events_filter: Option<Box<EventFilter>>,
    subdomains_filter: Option<Box<DomainFilter>>,
    registration_filter: Option<Box<RegistrationFilter>>,
    wrapped_domain_filter: Option<Box<WrappedDomainFilter>>,
) {
    push_domain_events_filter(separated, has_where, events_filter);
    push_subdomains_filter(separated, has_where, subdomains_filter);
    push_registration_filter(separated, has_where, registration_filter);
    push_wrapped_domain_filter(separated, has_where, wrapped_domain_filter);
}

fn push_subdomains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: Option<Box<DomainFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !domain_filter_has_conditions(&filter) {
        return;
    }

    push_where_prefix(separated, has_where);
    separated.push("id in (select parent_id from domains");
    let mut sub_has_where = false;
    push_domain_scalar_filter_conditions(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

fn push_registration_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: Option<Box<RegistrationFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !registration_filter_has_conditions(&filter) {
        return;
    }

    push_where_prefix(separated, has_where);
    separated.push("id in (select domain_id from registrations");
    let mut sub_has_where = false;
    push_registration_subquery_filters(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

fn push_wrapped_domain_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: Option<Box<WrappedDomainFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !wrapped_domain_filter_has_conditions(&filter) {
        return;
    }

    push_where_prefix(separated, has_where);
    separated.push("id in (select domain_id from wrapped_domains");
    let mut sub_has_where = false;
    push_wrapped_domain_subquery_filters(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use crate::filters::{RegistrationFilter, WrappedDomainFilter};

    use super::*;

    #[test]
    fn domain_filter_supports_subdomains_relation() {
        let filter = Some(Box::new(DomainFilter {
            name_contains_nocase: Some("alice".into()),
            ..DomainFilter::default()
        }));

        let mut query = QueryBuilder::<Postgres>::new("select id from domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_derived_relation_filters(
            &mut separated,
            &mut has_where,
            None,
            filter,
            None,
            None,
        );

        assert_eq!(
            query.build().sql(),
            "select id from domains where id in (select parent_id from domains where lower(name) like lower($1))"
        );
    }

    #[test]
    fn domain_filter_supports_registration_relation() {
        let filter = Some(Box::new(RegistrationFilter {
            label_name_contains: Some("alice".into()),
            ..RegistrationFilter::default()
        }));

        let mut query = QueryBuilder::<Postgres>::new("select id from domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_derived_relation_filters(
            &mut separated,
            &mut has_where,
            None,
            None,
            filter,
            None,
        );

        assert_eq!(
            query.build().sql(),
            "select id from domains where id in (select domain_id from registrations where label_name like $1)"
        );
    }

    #[test]
    fn domain_filter_supports_wrapped_domain_relation() {
        let filter = Some(Box::new(WrappedDomainFilter {
            fuses_gte: Some(10),
            id_in: Some(vec!["wrapped-a".into(), "wrapped-b".into()]),
            ..WrappedDomainFilter::default()
        }));

        let mut query = QueryBuilder::<Postgres>::new("select id from domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_derived_relation_filters(
            &mut separated,
            &mut has_where,
            None,
            None,
            None,
            filter,
        );

        assert_eq!(
            query.build().sql(),
            "select id from domains where id in (select domain_id from wrapped_domains where id = any($1) and fuses >= $2)"
        );
    }
}
