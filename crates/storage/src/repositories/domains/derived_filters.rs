use sqlx::{Postgres, query_builder::Separated};

use crate::{
    filters::{RegistrationFilter, WrappedDomainFilter},
    query::push_where_prefix,
    repositories::{
        registrations::{push_registration_subquery_filters, registration_filter_has_conditions},
        wrapped_domains::{
            push_wrapped_domain_subquery_filters, wrapped_domain_filter_has_conditions,
        },
    },
};

pub(super) fn push_domain_derived_relation_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    registration_filter: Option<Box<RegistrationFilter>>,
    wrapped_domain_filter: Option<Box<WrappedDomainFilter>>,
) {
    push_registration_filter(separated, has_where, registration_filter);
    push_wrapped_domain_filter(separated, has_where, wrapped_domain_filter);
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
    fn domain_filter_supports_registration_relation() {
        let filter = Some(Box::new(RegistrationFilter {
            label_name_contains: Some("alice".into()),
            ..RegistrationFilter::default()
        }));

        let mut query = QueryBuilder::<Postgres>::new("select id from domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_derived_relation_filters(&mut separated, &mut has_where, filter, None);

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
        push_domain_derived_relation_filters(&mut separated, &mut has_where, None, filter);

        assert_eq!(
            query.build().sql(),
            "select id from domains where id in (select domain_id from wrapped_domains where id = any($1) and fuses >= $2)"
        );
    }
}
