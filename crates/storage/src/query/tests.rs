use sqlx::{Execute, Postgres, QueryBuilder};

use crate::{
    filters::{AccountFilter, DomainFilter},
    query::{
        push_account_filters, push_account_relation_filter, push_domain_relation_filter,
        push_i32_array_filter, push_numeric_text_array_filter, push_numeric_text_filter,
        push_text_filter, push_text_not_contains_filter, push_text_not_prefix_filter,
        push_text_prefix_nocase_filter,
    },
};

#[test]
fn scalar_filter_fragments_do_not_inject_separators_inside_predicates() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_filter(&mut separated, &mut has_where, "id", Some("0xabc".into()));
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            ">=",
            Some("10".into()),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where id = $1 and created_at >= $2::numeric "
    );
}

#[test]
fn relationship_filter_fragments_keep_subquery_predicates_grouped() {
    let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            Some(Box::new(DomainFilter {
                name_contains_nocase: Some("foo".into()),
                is_migrated: Some(false),
                ..DomainFilter::default()
            })),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registrations where domain_id in (select id from domains where lower(name) like lower($1) and is_migrated = $2) "
    );
}

#[test]
fn entity_id_comparison_filters_emit_sql_predicates() {
    let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            Some(Box::new(DomainFilter {
                id_gt: Some("0x1000".into()),
                id_lte: Some("0xffff".into()),
                ..DomainFilter::default()
            })),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registrations where domain_id in (select id from domains where id > $1 and id <= $2) "
    );
}

#[test]
fn account_filter_supports_boolean_composition() {
    let mut query = QueryBuilder::<Postgres>::new("select id from accounts");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_account_filters(
            &mut separated,
            &mut has_where,
            AccountFilter {
                id_not: Some("0x0000".into()),
                or: Some(vec![
                    AccountFilter {
                        id: Some("0x1111".into()),
                        ..AccountFilter::default()
                    },
                    AccountFilter {
                        id: Some("0x2222".into()),
                        ..AccountFilter::default()
                    },
                ]),
                ..AccountFilter::default()
            },
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from accounts where id != $1 and ((id = $2) or (id = $3)) "
    );
}

#[test]
fn account_relation_filter_supports_boolean_composition() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "owner_id",
            Some(Box::new(AccountFilter {
                and: Some(vec![AccountFilter {
                    id_in: Some(vec!["0x1111".into(), "0x2222".into()]),
                    ..AccountFilter::default()
                }]),
                ..AccountFilter::default()
            })),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where owner_id in (select id from accounts where ((id = any($1)))) "
    );
}

#[test]
fn negated_text_like_filters_keep_predicates_grouped() {
    let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_not_contains_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            Some("foo".into()),
            true,
        );
        push_text_not_prefix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            Some("bar".into()),
            false,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registrations where not (lower(label_name) like lower($1)) and not (label_name like $2) "
    );
}

#[test]
fn nocase_prefix_filter_emits_lowered_predicate() {
    let mut query = QueryBuilder::<Postgres>::new("select id from wrapped_domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_filter(&mut separated, &mut has_where, "id", Some("abc".into()));
        push_text_prefix_nocase_filter(&mut separated, &mut has_where, "name", Some("foo".into()));
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from wrapped_domains where id = $1 and lower(name) like lower($2) "
    );
}

#[test]
fn numeric_text_array_filters_cast_each_bound_value() {
    let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            Some(vec!["10".into(), "20".into()]),
            false,
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "cost",
            Some(vec!["30".into()]),
            true,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registrations where expiry_date = any(array[$1::numeric, $2::numeric]) and not (cost = any(array[$3::numeric])) "
    );
}

#[test]
fn i32_array_filters_emit_any_predicates() {
    let mut query = QueryBuilder::<Postgres>::new("select id from wrapped_domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_i32_array_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            Some(vec![1, 2]),
            false,
        );
        push_i32_array_filter(&mut separated, &mut has_where, "fuses", Some(vec![3]), true);
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from wrapped_domains where fuses = any($1) and not (fuses = any($2)) "
    );
}
