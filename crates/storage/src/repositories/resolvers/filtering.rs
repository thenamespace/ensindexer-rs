use sqlx::{Postgres, query_builder::Separated};

use super::composition::push_resolver_filter_group;
use crate::{filters::ResolverFilter, query::*, repositories::events::push_resolver_events_filter};

pub(super) fn push_resolver_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    mut filter: ResolverFilter,
) {
    push_text_filter(separated, has_where, "id", filter.id.take());
    push_text_not_filter(separated, has_where, "id", filter.id_not.take());
    push_text_comparison_filters(
        separated,
        has_where,
        "id",
        filter.id_gt.take(),
        filter.id_lt.take(),
        filter.id_gte.take(),
        filter.id_lte.take(),
    );
    push_text_array_filter(separated, has_where, "id", filter.id_in.take());
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in.take());
    push_change_block_filter(
        separated,
        has_where,
        "Resolver",
        "resolvers.id",
        filter.change_block_number_gte.take(),
    );
    push_resolver_events_filter(separated, has_where, filter.events_filter.take());
    let and_filters = filter.and.take();
    let or_filters = filter.or.take();
    push_text_field_filters(separated, has_where, "domain_id", domain_field(&mut filter));
    push_domain_relation_filter(
        separated,
        has_where,
        "domain_id",
        filter.domain_filter.take(),
    );
    push_text_field_filters(separated, has_where, "address", address_field(&mut filter));
    push_text_field_filters(separated, has_where, "addr_id", addr_field(&mut filter));
    push_account_relation_filter(separated, has_where, "addr_id", filter.addr_filter.take());
    push_text_field_filters(
        separated,
        has_where,
        "content_hash",
        content_hash_field(&mut filter),
    );
    push_record_array_filters(separated, has_where, filter);
    push_resolver_filter_group(separated, has_where, " and ", and_filters);
    push_resolver_filter_group(separated, has_where, " or ", or_filters);
}

fn domain_field(filter: &mut ResolverFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.domain_id.take(),
        not: filter.domain_id_not.take(),
        gt: filter.domain_id_gt.take(),
        lt: filter.domain_id_lt.take(),
        gte: filter.domain_id_gte.take(),
        lte: filter.domain_id_lte.take(),
        in_values: filter.domain_id_in.take(),
        not_in: filter.domain_id_not_in.take(),
        contains: filter.domain_id_contains.take(),
        contains_nocase: filter.domain_id_contains_nocase.take(),
        not_contains: filter.domain_id_not_contains.take(),
        not_contains_nocase: filter.domain_id_not_contains_nocase.take(),
        starts_with: filter.domain_id_starts_with.take(),
        starts_with_nocase: filter.domain_id_starts_with_nocase.take(),
        not_starts_with: filter.domain_id_not_starts_with.take(),
        not_starts_with_nocase: filter.domain_id_not_starts_with_nocase.take(),
        ends_with: filter.domain_id_ends_with.take(),
        ends_with_nocase: filter.domain_id_ends_with_nocase.take(),
        not_ends_with: filter.domain_id_not_ends_with.take(),
        not_ends_with_nocase: filter.domain_id_not_ends_with_nocase.take(),
    }
}

fn address_field(filter: &mut ResolverFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.address.take(),
        not: filter.address_not.take(),
        gt: filter.address_gt.take(),
        lt: filter.address_lt.take(),
        gte: filter.address_gte.take(),
        lte: filter.address_lte.take(),
        in_values: filter.address_in.take(),
        not_in: filter.address_not_in.take(),
        contains: filter.address_contains.take(),
        not_contains: filter.address_not_contains.take(),
        ..TextFieldFilter::default()
    }
}

fn addr_field(filter: &mut ResolverFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.addr_id.take(),
        not: filter.addr_id_not.take(),
        gt: filter.addr_id_gt.take(),
        lt: filter.addr_id_lt.take(),
        gte: filter.addr_id_gte.take(),
        lte: filter.addr_id_lte.take(),
        in_values: filter.addr_id_in.take(),
        not_in: filter.addr_id_not_in.take(),
        contains: filter.addr_id_contains.take(),
        contains_nocase: filter.addr_id_contains_nocase.take(),
        not_contains: filter.addr_id_not_contains.take(),
        not_contains_nocase: filter.addr_id_not_contains_nocase.take(),
        starts_with: filter.addr_id_starts_with.take(),
        starts_with_nocase: filter.addr_id_starts_with_nocase.take(),
        not_starts_with: filter.addr_id_not_starts_with.take(),
        not_starts_with_nocase: filter.addr_id_not_starts_with_nocase.take(),
        ends_with: filter.addr_id_ends_with.take(),
        ends_with_nocase: filter.addr_id_ends_with_nocase.take(),
        not_ends_with: filter.addr_id_not_ends_with.take(),
        not_ends_with_nocase: filter.addr_id_not_ends_with_nocase.take(),
    }
}

fn content_hash_field(filter: &mut ResolverFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.content_hash.take(),
        not: filter.content_hash_not.take(),
        gt: filter.content_hash_gt.take(),
        lt: filter.content_hash_lt.take(),
        gte: filter.content_hash_gte.take(),
        lte: filter.content_hash_lte.take(),
        in_values: filter.content_hash_in.take(),
        not_in: filter.content_hash_not_in.take(),
        contains: filter.content_hash_contains.take(),
        not_contains: filter.content_hash_not_contains.take(),
        ..TextFieldFilter::default()
    }
}

fn push_record_array_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    mut filter: ResolverFilter,
) {
    push_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts.take(),
        false,
        false,
    );
    push_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts_not.take(),
        false,
        true,
    );
    push_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts_contains.take(),
        false,
        false,
    );
    push_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts_contains_nocase.take(),
        true,
        false,
    );
    push_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts_not_contains.take(),
        false,
        true,
    );
    push_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts_not_contains_nocase.take(),
        true,
        true,
    );
    push_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types.take(),
        false,
    );
    push_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_not.take(),
        true,
    );
    push_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_contains.take(),
        false,
    );
    push_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_contains_nocase.take(),
        false,
    );
    push_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_not_contains.take(),
        true,
    );
    push_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_not_contains_nocase.take(),
        true,
    );
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::*;
    use crate::filters::{AccountFilter, DomainFilter};

    #[test]
    fn resolver_filters_support_boolean_composition() {
        let mut query = QueryBuilder::<Postgres>::new("select id from resolvers");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_resolver_filters(
                &mut separated,
                &mut has_where,
                ResolverFilter {
                    address: Some("0xresolver".into()),
                    or: Some(vec![
                        ResolverFilter {
                            content_hash_contains: Some("0xabc".into()),
                            ..ResolverFilter::default()
                        },
                        ResolverFilter {
                            texts_contains: Some("email".into()),
                            ..ResolverFilter::default()
                        },
                    ]),
                    ..ResolverFilter::default()
                },
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from resolvers where address = $1 and (id in (select id from resolvers where content_hash like $2) or id in (select id from resolvers where texts @> array[$3]::text[])) "
        );
    }

    #[test]
    fn resolver_composition_supports_nested_relationship_filters() {
        let mut query = QueryBuilder::<Postgres>::new("select id from resolvers");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_resolver_filters(
                &mut separated,
                &mut has_where,
                ResolverFilter {
                    or: Some(vec![
                        ResolverFilter {
                            domain_filter: Some(Box::new(DomainFilter {
                                parent_filter: Some(Box::new(DomainFilter {
                                    name: Some("eth".into()),
                                    ..DomainFilter::default()
                                })),
                                ..DomainFilter::default()
                            })),
                            ..ResolverFilter::default()
                        },
                        ResolverFilter {
                            addr_filter: Some(Box::new(AccountFilter {
                                id: Some("0xaddr".into()),
                                ..AccountFilter::default()
                            })),
                            ..ResolverFilter::default()
                        },
                    ]),
                    ..ResolverFilter::default()
                },
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from resolvers where (id in (select id from resolvers where domain_id in (select id from domains where parent_id in (select id from domains where name = $1))) or id in (select id from resolvers where addr_id in (select id from accounts where id = $2))) "
        );
    }

    #[test]
    fn resolver_filters_support_change_block_predicate() {
        let mut query = QueryBuilder::<Postgres>::new("select id from resolvers");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_resolver_filters(
                &mut separated,
                &mut has_where,
                ResolverFilter {
                    change_block_number_gte: Some(100),
                    ..ResolverFilter::default()
                },
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from resolvers where exists (select 1 from entity_changes where entity_type = $1 and entity_id = resolvers.id and block_number >= $2) "
        );
    }
}
