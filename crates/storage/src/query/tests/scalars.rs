use super::*;

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
fn hashed_exact_text_filter_keeps_exact_recheck() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_field_filters_hashed_exact(
            &mut separated,
            &mut has_where,
            "name",
            TextFieldFilter {
                exact: Some("vitalik.eth".into()),
                contains: Some("eth".into()),
                ..TextFieldFilter::default()
            },
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where md5(name) = md5($1) and name = $2 and name like $3 "
    );
}

#[test]
fn hashed_exact_text_in_filter_keeps_exact_recheck() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_field_filters_hashed_exact(
            &mut separated,
            &mut has_where,
            "name",
            TextFieldFilter {
                in_values: Some(vec!["vitalik.eth".into(), "nick.eth".into()]),
                ..TextFieldFilter::default()
            },
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where md5(name) = any(array[md5($1), md5($2)]) and name = any($3) "
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

#[test]
fn text_element_filters_support_nocase_and_negation() {
    let mut query = QueryBuilder::<Postgres>::new("select id from resolvers");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            Some("email".into()),
            false,
            false,
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            Some("Url".into()),
            true,
            true,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from resolvers where texts @> array[$1]::text[] and not (exists (select 1 from unnest(texts) as value where lower(value) = lower($2))) "
    );
}

#[test]
fn numeric_element_filters_support_negation() {
    let mut query = QueryBuilder::<Postgres>::new("select id from resolvers");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            Some("60".into()),
            false,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            Some("0".into()),
            true,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from resolvers where coin_types @> array[$1::numeric] and not (coin_types @> array[$2::numeric]) "
    );
}
