use sqlx::{Postgres, query_builder::Separated};

use crate::{
    filters::{AccountFilter, DomainFilter, ResolverFilter},
    query::{
        account_filter_has_conditions, domain_filter_has_conditions,
        push_account_filter_conditions, push_domain_scalar_filter_conditions,
        push_sub_change_block_filter, push_where_prefix, resolver_filter_has_conditions,
    },
    repositories::events::push_resolver_events_filter,
};

pub(super) fn push_resolver_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    joiner: &'static str,
    filters: Option<Vec<ResolverFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    let filters: Vec<_> = filters
        .into_iter()
        .filter(resolver_filter_has_conditions)
        .collect();
    if filters.is_empty() {
        return;
    }

    push_where_prefix(separated, has_where);
    separated.push("(");
    for (index, filter) in filters.into_iter().enumerate() {
        if index > 0 {
            separated.push_unseparated(joiner);
        }
        separated.push_unseparated("id in (select id from resolvers");
        let mut sub_has_where = false;
        push_resolver_subquery_filters(separated, &mut sub_has_where, filter);
        separated.push_unseparated(")");
    }
    separated.push_unseparated(")");
}

fn push_resolver_subquery_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: ResolverFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_filter(separated, has_where, "id", ">", filter.id_gt);
    push_sub_text_filter(separated, has_where, "id", "<", filter.id_lt);
    push_sub_text_filter(separated, has_where, "id", ">=", filter.id_gte);
    push_sub_text_filter(separated, has_where, "id", "<=", filter.id_lte);
    push_sub_text_filter(separated, has_where, "domain_id", "=", filter.domain_id);
    push_sub_text_contains_filter(separated, has_where, "domain_id", filter.domain_id_contains);
    push_sub_domain_relation_filter(separated, has_where, "domain_id", filter.domain_filter);
    push_sub_text_filter(separated, has_where, "address", "=", filter.address);
    push_sub_text_contains_filter(separated, has_where, "address", filter.address_contains);
    push_sub_text_filter(separated, has_where, "addr_id", "=", filter.addr_id);
    push_sub_text_contains_filter(separated, has_where, "addr_id", filter.addr_id_contains);
    push_sub_account_relation_filter(separated, has_where, "addr_id", filter.addr_filter);
    push_sub_text_filter(
        separated,
        has_where,
        "content_hash",
        "=",
        filter.content_hash,
    );
    push_sub_text_contains_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_contains,
    );
    push_sub_text_element_filter(separated, has_where, "texts", filter.texts_contains, false);
    push_sub_text_element_filter(
        separated,
        has_where,
        "texts",
        filter.texts_contains_nocase,
        true,
    );
    push_sub_numeric_element_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_contains,
        false,
    );
    push_sub_change_block_filter(
        separated,
        has_where,
        "Resolver",
        "id",
        filter.change_block_number_gte,
    );
    push_resolver_events_filter(separated, has_where, filter.events_filter);
    push_resolver_filter_group(separated, has_where, " and ", filter.and);
    push_resolver_filter_group(separated, has_where, " or ", filter.or);
}

fn push_sub_domain_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: Option<Box<DomainFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !domain_filter_has_conditions(&filter) {
        return;
    }

    push_sub_where_prefix(separated, has_where);
    separated
        .push_unseparated(column)
        .push_unseparated(" in (select id from domains");
    let mut sub_has_where = false;
    push_domain_scalar_filter_conditions(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

fn push_sub_account_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: Option<Box<AccountFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !account_filter_has_conditions(&filter) {
        return;
    }

    push_sub_where_prefix(separated, has_where);
    separated
        .push_unseparated(column)
        .push_unseparated(" in (select id from accounts");
    let mut sub_has_where = false;
    push_account_filter_conditions(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

fn push_sub_where_prefix<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
) {
    if *has_where {
        separated.push_unseparated(" and ");
    } else {
        separated.push_unseparated(" where ");
        *has_where = true;
    }
}

fn push_sub_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

fn push_sub_text_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("%{value}%"));
    }
}

fn push_sub_text_element_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        if nocase {
            separated
                .push_unseparated("exists (select 1 from unnest(")
                .push_unseparated(column)
                .push_unseparated(") as value where lower(value) = lower(")
                .push_bind_unseparated(value)
                .push_unseparated("))");
        } else {
            separated
                .push_unseparated(column)
                .push_unseparated(" @> array[")
                .push_bind_unseparated(value)
                .push_unseparated("]::text[]");
        }
    }
}

fn push_sub_numeric_element_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    negate: bool,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        if negate {
            separated.push_unseparated("not (");
        }
        separated
            .push_unseparated(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric]");
        if negate {
            separated.push_unseparated(")");
        }
    }
}
