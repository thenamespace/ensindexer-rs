use super::*;

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
fn account_filter_supports_change_block_predicate() {
    let mut query = QueryBuilder::<Postgres>::new("select id from accounts");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_account_filters(
            &mut separated,
            &mut has_where,
            AccountFilter {
                change_block_number_gte: Some(100),
                ..AccountFilter::default()
            },
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from accounts where exists (select 1 from entity_changes where entity_type = $1 and entity_id = accounts.id and block_number >= $2) "
    );
}

#[test]
fn change_block_filter_emits_entity_changes_predicate() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_change_block_filter(
            &mut separated,
            &mut has_where,
            "Domain",
            "domains.id",
            Some(50),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where exists (select 1 from entity_changes where entity_type = $1 and entity_id = domains.id and block_number >= $2) "
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
fn domain_filter_supports_boolean_composition() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_filter_group(
            &mut separated,
            &mut has_where,
            " or ",
            Some(vec![
                DomainFilter {
                    name_contains_nocase: Some("foo".into()),
                    is_migrated: Some(true),
                    ..DomainFilter::default()
                },
                DomainFilter {
                    labelhash_not: Some("0x00".into()),
                    ttl_gte: Some("100".into()),
                    ..DomainFilter::default()
                },
            ]),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where (id in (select id from domains where lower(name) like lower($1) and is_migrated = $2) or id in (select id from domains where labelhash != $3 and ttl >= $4::numeric)) "
    );
}

#[test]
fn domain_relation_filter_supports_boolean_composition() {
    let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            Some(Box::new(DomainFilter {
                and: Some(vec![
                    DomainFilter {
                        name_starts_with: Some("vitalik".into()),
                        ..DomainFilter::default()
                    },
                    DomainFilter {
                        expiry_date_gt: Some("1000".into()),
                        ..DomainFilter::default()
                    },
                ]),
                ..DomainFilter::default()
            })),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registrations where domain_id in (select id from domains where (id in (select id from domains where name like $1) and id in (select id from domains where expiry_date > $2::numeric))) "
    );
}

#[test]
fn domain_relation_filter_supports_nested_domain_relationships() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "parent_id",
            Some(Box::new(DomainFilter {
                parent_filter: Some(Box::new(DomainFilter {
                    name: Some("eth".into()),
                    ..DomainFilter::default()
                })),
                ..DomainFilter::default()
            })),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where parent_id in (select id from domains where parent_id in (select id from domains where name = $1)) "
    );
}

#[test]
fn domain_filter_composition_supports_nested_relationship_only_conditions() {
    let mut query = QueryBuilder::<Postgres>::new("select id from domains");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_domain_filter_group(
            &mut separated,
            &mut has_where,
            " and ",
            Some(vec![DomainFilter {
                parent_filter: Some(Box::new(DomainFilter {
                    owner_filter: Some(Box::new(AccountFilter {
                        id: Some("0xowner".into()),
                        ..AccountFilter::default()
                    })),
                    ..DomainFilter::default()
                })),
                ..DomainFilter::default()
            }]),
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domains where (id in (select id from domains where parent_id in (select id from domains where owner_id in (select id from accounts where id = $1)))) "
    );
}
