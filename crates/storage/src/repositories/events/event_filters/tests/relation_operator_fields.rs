use super::*;
use crate::filters::TextOperatorFilter;

#[test]
fn domain_event_relation_column_filters_include_operator_variants() {
    let filter = EventFilter {
        parent_domain_id: Some("0xparent".into()),
        parent_domain_id_ops: TextOperatorFilter {
            contains_nocase: Some("abcd".into()),
            not_ends_with: Some("ffff".into()),
            ..TextOperatorFilter::default()
        },
        resolver_id_ops: TextOperatorFilter {
            gt: Some("0x1000".into()),
            not_in: Some(vec!["0xdead".into()]),
            ..TextOperatorFilter::default()
        },
        ..EventFilter::default()
    };
    let mut query = QueryBuilder::<Postgres>::new("select id from new_owner_events");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_specific_filters(&mut separated, &mut has_where, "new_owner_events", &filter);
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from new_owner_events where parent_domain_id = $1 and lower(parent_domain_id) like lower($2) and not (parent_domain_id like $3) "
    );

    let mut query = QueryBuilder::<Postgres>::new("select id from new_resolver_events");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_specific_filters(
            &mut separated,
            &mut has_where,
            "new_resolver_events",
            &filter,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from new_resolver_events where resolver_id > $1 and not (resolver_id = any($2)) "
    );
}

#[test]
fn registration_event_relation_column_filters_include_operator_variants() {
    let filter = EventFilter {
        registrant_id_ops: TextOperatorFilter {
            not: Some("0xold".into()),
            starts_with_nocase: Some("0xabc".into()),
            ..TextOperatorFilter::default()
        },
        new_owner_id: Some("0xowner".into()),
        new_owner_id_ops: TextOperatorFilter {
            contains_nocase: Some("beef".into()),
            not_in: Some(vec!["0xdead".into()]),
            ..TextOperatorFilter::default()
        },
        ..EventFilter::default()
    };
    let mut query = QueryBuilder::<Postgres>::new("select id from name_registered_events");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_specific_filters(
            &mut separated,
            &mut has_where,
            "name_registered_events",
            &filter,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from name_registered_events where registrant_id != $1 and lower(registrant_id) like lower($2) "
    );

    let mut query = QueryBuilder::<Postgres>::new("select id from registration_event_refs");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_specific_filters(
            &mut separated,
            &mut has_where,
            "registration_event_refs",
            &filter,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registration_event_refs where registrant_id != $1 and lower(registrant_id) like lower($2) and new_owner_id = $3 and not (new_owner_id = any($4)) and lower(new_owner_id) like lower($5) "
    );
}
