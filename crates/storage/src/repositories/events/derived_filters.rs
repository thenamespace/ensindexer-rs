use sqlx::{Postgres, query_builder::Separated};

use super::{
    event_filters::push_event_filters,
    event_sql::{
        domain_event_ref_union_sql, registration_event_ref_union_sql, resolver_event_ref_union_sql,
    },
    relation_filters::push_interface_parent_relation_filter,
    specific_filters::push_event_specific_filters,
};
use crate::{filters::EventFilter, query::push_where_prefix};

pub(crate) fn push_domain_events_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: Option<Box<EventFilter>>,
) {
    push_events_filter(
        separated,
        has_where,
        domain_event_ref_union_sql(),
        "domain_event_refs",
        filter,
    );
}

pub(crate) fn push_registration_events_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: Option<Box<EventFilter>>,
) {
    push_events_filter(
        separated,
        has_where,
        registration_event_ref_union_sql(),
        "registration_event_refs",
        filter,
    );
}

pub(crate) fn push_resolver_events_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: Option<Box<EventFilter>>,
) {
    push_events_filter(
        separated,
        has_where,
        resolver_event_ref_union_sql(),
        "resolver_event_refs",
        filter,
    );
}

fn push_events_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    union_sql: &'static str,
    interface_table: &'static str,
    filter: Option<Box<EventFilter>>,
) {
    let Some(mut filter) = filter else {
        return;
    };
    let and_filters = filter.and.take();
    let or_filters = filter.or.take();

    push_where_prefix(separated, has_where);
    separated.push("id in (select parent_id from (");
    separated.push_unseparated(union_sql);
    separated.push_unseparated(") event_refs");

    separated.push_unseparated(" where true");
    let mut sub_has_where = true;
    push_event_filters(separated, &mut sub_has_where, "parent_id", &filter);
    push_interface_parent_relation_filter(separated, &mut sub_has_where, interface_table, &filter);
    push_event_specific_filters(separated, &mut sub_has_where, interface_table, &filter);
    push_nested_groups(
        separated,
        &mut sub_has_where,
        union_sql,
        interface_table,
        " and ",
        and_filters,
    );
    push_nested_groups(
        separated,
        &mut sub_has_where,
        union_sql,
        interface_table,
        " or ",
        or_filters,
    );
    separated.push_unseparated(")");
}

fn push_nested_groups<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    union_sql: &'static str,
    interface_table: &'static str,
    joiner: &'static str,
    filters: Option<Vec<EventFilter>>,
) {
    let Some(filters) = filters.filter(|filters| !filters.is_empty()) else {
        return;
    };

    separated.push_unseparated(if *has_where { " and " } else { " where " });
    *has_where = true;
    separated.push_unseparated("(");
    for (index, filter) in filters.into_iter().enumerate() {
        if index > 0 {
            separated.push_unseparated(joiner);
        }
        separated.push_unseparated("id in (select id from (");
        separated.push_unseparated(union_sql);
        separated.push_unseparated(") event_refs");
        separated.push_unseparated(" where true");
        let mut sub_has_where = true;
        push_event_filters(separated, &mut sub_has_where, "parent_id", &filter);
        push_interface_parent_relation_filter(
            separated,
            &mut sub_has_where,
            interface_table,
            &filter,
        );
        push_event_specific_filters(separated, &mut sub_has_where, interface_table, &filter);
        separated.push_unseparated(")");
    }
    separated.push_unseparated(")");
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::*;

    #[test]
    fn domain_events_filter_uses_parent_id_subquery() {
        let filter = EventFilter {
            block_number_gte: Some(100),
            name_contains: Some("wrapped".into()),
            ..EventFilter::default()
        };

        let mut query = QueryBuilder::<Postgres>::new("select id from domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_events_filter(&mut separated, &mut has_where, Some(Box::new(filter)));

        let built = query.build();
        let sql_str = built.sql();
        let sql = sql_str.as_str();
        assert!(sql.starts_with("select id from domains where id in (select parent_id from ("));
        assert!(sql.contains("from name_wrapped_events"));
        assert!(
            sql.contains("event_refs where true and block_number >= $1"),
            "{sql}"
        );
        assert!(sql.contains("name like $2"), "{sql}");
    }

    #[test]
    fn registration_events_filter_supports_empty_existence_filter() {
        let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_registration_events_filter(
            &mut separated,
            &mut has_where,
            Some(Box::new(EventFilter::default())),
        );

        let built = query.build();
        let sql_str = built.sql();
        let sql = sql_str.as_str();
        assert!(
            sql.starts_with("select id from registrations where id in (select parent_id from (")
        );
        assert!(sql.contains("from name_registered_events"));
        assert!(sql.ends_with(") event_refs where true)"));
    }

    #[test]
    fn resolver_events_filter_supports_boolean_groups() {
        let filter = EventFilter {
            and: Some(vec![EventFilter {
                key: Some("avatar".into()),
                ..EventFilter::default()
            }]),
            ..EventFilter::default()
        };

        let mut query = QueryBuilder::<Postgres>::new("select id from resolvers");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_resolver_events_filter(&mut separated, &mut has_where, Some(Box::new(filter)));

        let built = query.build();
        let sql_str = built.sql();
        let sql = sql_str.as_str();
        assert!(sql.starts_with("select id from resolvers where id in (select parent_id from ("));
        assert!(sql.contains("from text_changed_events"));
        assert!(sql.contains(" where true and (id in (select id from ("));
        assert!(sql.contains("event_refs where true and key = $1"), "{sql}");
    }
}
