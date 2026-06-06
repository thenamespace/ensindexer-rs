use sqlx::{Postgres, query_builder::Separated};

use crate::filters::{
    AccountFilter, AccountOrderField, DomainFilter, DomainOrderField, EventOrderField,
    RegistrationOrderField, ResolverFilter, ResolverOrderField, WrappedDomainOrderField,
};

pub(crate) fn push_where_prefix<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
) {
    if !*has_where {
        separated.push_unseparated(" where ");
        *has_where = true;
    }
}

pub(crate) fn push_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" = ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_not_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" != ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated(")");
    }
}

pub(crate) fn push_text_not_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        push_where_prefix(separated, has_where);
        separated
            .push("not (")
            .push_unseparated(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated("))");
    }
}

pub(crate) fn push_text_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        if nocase {
            separated
                .push("lower(")
                .push_unseparated(column)
                .push_unseparated(") like lower(")
                .push_bind_unseparated(format!("%{value}%"))
                .push_unseparated(")");
        } else {
            separated
                .push(column)
                .push_unseparated(" like ")
                .push_bind_unseparated(format!("%{value}%"));
        }
    }
}

pub(crate) fn push_text_prefix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("{value}%"));
    }
}

pub(crate) fn push_text_suffix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("%{value}"));
    }
}

pub(crate) fn push_numeric_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric");
    }
}

pub(crate) fn push_i32_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<i32>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_bool_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<bool>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("]::text[]");
    }
}

pub(crate) fn push_numeric_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric]");
    }
}

pub(crate) fn push_account_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    foreign_column: &'static str,
    filter: Option<Box<AccountFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !account_filter_has_conditions(&filter) {
        return;
    }

    push_where_prefix(separated, has_where);
    separated
        .push(foreign_column)
        .push_unseparated(" in (select id from accounts");
    let mut sub_has_where = false;
    push_account_filter_conditions(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

pub(crate) fn push_account_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: AccountFilter,
) {
    push_text_filter(separated, has_where, "id", filter.id);
    push_text_not_filter(separated, has_where, "id", filter.id_not);
    push_text_array_filter(separated, has_where, "id", filter.id_in);
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in);
    push_account_filter_group(separated, has_where, " and ", filter.and);
    push_account_filter_group(separated, has_where, " or ", filter.or);
}

pub(crate) fn push_domain_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    foreign_column: &'static str,
    filter: Option<Box<DomainFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !domain_filter_has_scalar_conditions(&filter) {
        return;
    }

    push_where_prefix(separated, has_where);
    separated
        .push(foreign_column)
        .push_unseparated(" in (select id from domains");
    let mut sub_has_where = false;
    push_domain_scalar_filter_conditions(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

pub(crate) fn push_resolver_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    foreign_column: &'static str,
    filter: Option<Box<ResolverFilter>>,
) {
    let Some(filter) = filter else {
        return;
    };
    if !resolver_filter_has_scalar_conditions(&filter) {
        return;
    }

    push_where_prefix(separated, has_where);
    separated
        .push(foreign_column)
        .push_unseparated(" in (select id from resolvers");
    let mut sub_has_where = false;
    push_resolver_scalar_filter_conditions(separated, &mut sub_has_where, *filter);
    separated.push_unseparated(")");
}

fn account_filter_has_conditions(filter: &AccountFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter
            .and
            .as_ref()
            .is_some_and(|filters| filters.iter().any(account_filter_has_conditions))
        || filter
            .or
            .as_ref()
            .is_some_and(|filters| filters.iter().any(account_filter_has_conditions))
}

fn domain_filter_has_scalar_conditions(filter: &DomainFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.name.is_some()
        || filter.name_contains.is_some()
        || filter.name_contains_nocase.is_some()
        || filter.name_starts_with.is_some()
        || filter.name_ends_with.is_some()
        || filter.label_name.is_some()
        || filter.label_name_contains.is_some()
        || filter.label_name_contains_nocase.is_some()
        || filter.label_name_starts_with.is_some()
        || filter.label_name_ends_with.is_some()
        || filter.labelhash.is_some()
        || filter.labelhash_not.is_some()
        || filter
            .labelhash_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter
            .labelhash_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.parent_id.is_some()
        || filter.subdomain_count.is_some()
        || filter.subdomain_count_gt.is_some()
        || filter.subdomain_count_lt.is_some()
        || filter.subdomain_count_gte.is_some()
        || filter.subdomain_count_lte.is_some()
        || filter.resolved_address_id.is_some()
        || filter.owner_id.is_some()
        || filter.resolver_id.is_some()
        || filter.registrant_id.is_some()
        || filter.wrapped_owner_id.is_some()
        || filter.is_migrated.is_some()
        || filter.is_migrated_not.is_some()
        || filter.created_at.is_some()
        || filter.created_at_gt.is_some()
        || filter.created_at_lt.is_some()
        || filter.created_at_gte.is_some()
        || filter.created_at_lte.is_some()
        || filter.expiry_date.is_some()
        || filter.expiry_date_gt.is_some()
        || filter.expiry_date_lt.is_some()
        || filter.expiry_date_gte.is_some()
        || filter.expiry_date_lte.is_some()
        || filter.ttl.is_some()
        || filter.ttl_gt.is_some()
        || filter.ttl_lt.is_some()
        || filter.ttl_gte.is_some()
        || filter.ttl_lte.is_some()
}

fn resolver_filter_has_scalar_conditions(filter: &ResolverFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.domain_id.is_some()
        || filter.address.is_some()
        || filter
            .address_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.addr_id.is_some()
        || filter.content_hash.is_some()
        || filter.content_hash_not.is_some()
        || filter
            .content_hash_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter
            .content_hash_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.content_hash_contains.is_some()
        || filter.texts_contains.is_some()
        || filter.coin_types_contains.is_some()
}

fn push_account_filter_conditions<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: AccountFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_in, false);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_not_in, true);
    push_sub_account_filter_group(separated, has_where, " and ", filter.and);
    push_sub_account_filter_group(separated, has_where, " or ", filter.or);
}

fn push_account_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    joiner: &'static str,
    filters: Option<Vec<AccountFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    let filters: Vec<_> = filters
        .into_iter()
        .filter(account_filter_has_conditions)
        .collect();
    if filters.is_empty() {
        return;
    }

    push_where_prefix(separated, has_where);
    separated.push("(");
    append_account_filter_list(separated, joiner, filters);
    separated.push_unseparated(")");
}

fn push_sub_account_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    joiner: &'static str,
    filters: Option<Vec<AccountFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    let filters: Vec<_> = filters
        .into_iter()
        .filter(account_filter_has_conditions)
        .collect();
    if filters.is_empty() {
        return;
    }

    push_sub_where_prefix(separated, has_where);
    separated.push_unseparated("(");
    append_account_filter_list(separated, joiner, filters);
    separated.push_unseparated(")");
}

fn append_account_filter_list<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    joiner: &'static str,
    filters: Vec<AccountFilter>,
) {
    for (index, filter) in filters.into_iter().enumerate() {
        if index > 0 {
            separated.push_unseparated(joiner);
        }
        separated.push_unseparated("(");
        let mut has_predicate = false;
        append_account_filter_predicates(separated, &mut has_predicate, filter);
        separated.push_unseparated(")");
    }
}

fn append_account_filter_predicates<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_predicate: &mut bool,
    filter: AccountFilter,
) {
    append_text_predicate(separated, has_predicate, "id", "=", filter.id);
    append_text_predicate(separated, has_predicate, "id", "!=", filter.id_not);
    append_text_array_predicate(separated, has_predicate, "id", filter.id_in, false);
    append_text_array_predicate(separated, has_predicate, "id", filter.id_not_in, true);
    append_account_composite_predicate(separated, has_predicate, " and ", filter.and);
    append_account_composite_predicate(separated, has_predicate, " or ", filter.or);
}

fn append_account_composite_predicate<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_predicate: &mut bool,
    joiner: &'static str,
    filters: Option<Vec<AccountFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    let filters: Vec<_> = filters
        .into_iter()
        .filter(account_filter_has_conditions)
        .collect();
    if filters.is_empty() {
        return;
    }

    append_predicate_prefix(separated, has_predicate);
    separated.push_unseparated("(");
    append_account_filter_list(separated, joiner, filters);
    separated.push_unseparated(")");
}

fn append_predicate_prefix<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_predicate: &mut bool,
) {
    if *has_predicate {
        separated.push_unseparated(" and ");
    } else {
        *has_predicate = true;
    }
}

fn append_text_predicate<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_predicate: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        append_predicate_prefix(separated, has_predicate);
        separated
            .push_unseparated(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

fn append_text_array_predicate<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_predicate: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
    negate: bool,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        append_predicate_prefix(separated, has_predicate);
        if negate {
            separated.push_unseparated("not (");
        }
        separated
            .push_unseparated(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated(")");
        if negate {
            separated.push_unseparated(")");
        }
    }
}

fn push_domain_scalar_filter_conditions<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: DomainFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_in, false);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_not_in, true);
    push_sub_text_filter(separated, has_where, "name", "=", filter.name);
    push_sub_text_contains_filter(separated, has_where, "name", filter.name_contains, false);
    push_sub_text_contains_filter(
        separated,
        has_where,
        "name",
        filter.name_contains_nocase,
        true,
    );
    push_sub_text_prefix_filter(separated, has_where, "name", filter.name_starts_with);
    push_sub_text_suffix_filter(separated, has_where, "name", filter.name_ends_with);
    push_sub_text_filter(separated, has_where, "label_name", "=", filter.label_name);
    push_sub_text_contains_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_contains,
        false,
    );
    push_sub_text_contains_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_contains_nocase,
        true,
    );
    push_sub_text_prefix_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_starts_with,
    );
    push_sub_text_suffix_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_ends_with,
    );
    push_sub_text_filter(separated, has_where, "labelhash", "=", filter.labelhash);
    push_sub_text_filter(
        separated,
        has_where,
        "labelhash",
        "!=",
        filter.labelhash_not,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "labelhash",
        filter.labelhash_in,
        false,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "labelhash",
        filter.labelhash_not_in,
        true,
    );
    push_sub_text_filter(separated, has_where, "parent_id", "=", filter.parent_id);
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        "=",
        filter.subdomain_count,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        ">",
        filter.subdomain_count_gt,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        "<",
        filter.subdomain_count_lt,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        ">=",
        filter.subdomain_count_gte,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        "<=",
        filter.subdomain_count_lte,
    );
    push_sub_text_filter(
        separated,
        has_where,
        "resolved_address_id",
        "=",
        filter.resolved_address_id,
    );
    push_sub_text_filter(separated, has_where, "owner_id", "=", filter.owner_id);
    push_sub_text_filter(separated, has_where, "resolver_id", "=", filter.resolver_id);
    push_sub_text_filter(
        separated,
        has_where,
        "registrant_id",
        "=",
        filter.registrant_id,
    );
    push_sub_text_filter(
        separated,
        has_where,
        "wrapped_owner_id",
        "=",
        filter.wrapped_owner_id,
    );
    push_sub_bool_filter(separated, has_where, "is_migrated", "=", filter.is_migrated);
    push_sub_bool_filter(
        separated,
        has_where,
        "is_migrated",
        "!=",
        filter.is_migrated_not,
    );
    push_sub_numeric_text_filter(separated, has_where, "created_at", "=", filter.created_at);
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        ">",
        filter.created_at_gt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        "<",
        filter.created_at_lt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        ">=",
        filter.created_at_gte,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        "<=",
        filter.created_at_lte,
    );
    push_sub_numeric_text_filter(separated, has_where, "expiry_date", "=", filter.expiry_date);
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        ">",
        filter.expiry_date_gt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "<",
        filter.expiry_date_lt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        ">=",
        filter.expiry_date_gte,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "<=",
        filter.expiry_date_lte,
    );
    push_sub_numeric_text_filter(separated, has_where, "ttl", "=", filter.ttl);
    push_sub_numeric_text_filter(separated, has_where, "ttl", ">", filter.ttl_gt);
    push_sub_numeric_text_filter(separated, has_where, "ttl", "<", filter.ttl_lt);
    push_sub_numeric_text_filter(separated, has_where, "ttl", ">=", filter.ttl_gte);
    push_sub_numeric_text_filter(separated, has_where, "ttl", "<=", filter.ttl_lte);
}

fn push_resolver_scalar_filter_conditions<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: ResolverFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_in, false);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_not_in, true);
    push_sub_text_filter(separated, has_where, "domain_id", "=", filter.domain_id);
    push_sub_text_filter(separated, has_where, "address", "=", filter.address);
    push_sub_text_array_filter(separated, has_where, "address", filter.address_in, false);
    push_sub_text_filter(separated, has_where, "addr_id", "=", filter.addr_id);
    push_sub_text_filter(
        separated,
        has_where,
        "content_hash",
        "=",
        filter.content_hash,
    );
    push_sub_text_filter(
        separated,
        has_where,
        "content_hash",
        "!=",
        filter.content_hash_not,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_in,
        false,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_not_in,
        true,
    );
    push_sub_text_contains_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_contains,
        false,
    );
    push_sub_text_array_contains_filter(separated, has_where, "texts", filter.texts_contains);
    push_sub_numeric_array_contains_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_contains,
    );
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

fn push_sub_text_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
    negate: bool,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        push_sub_where_prefix(separated, has_where);
        if negate {
            separated.push_unseparated("not (");
        }
        separated
            .push_unseparated(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated(")");
        if negate {
            separated.push_unseparated(")");
        }
    }
}

fn push_sub_text_contains_filter<'qb>(
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
                .push_unseparated("lower(")
                .push_unseparated(column)
                .push_unseparated(") like lower(")
                .push_bind_unseparated(format!("%{value}%"))
                .push_unseparated(")");
        } else {
            separated
                .push_unseparated(column)
                .push_unseparated(" like ")
                .push_bind_unseparated(format!("%{value}%"));
        }
    }
}

fn push_sub_text_prefix_filter<'qb>(
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
            .push_bind_unseparated(format!("{value}%"));
    }
}

fn push_sub_text_suffix_filter<'qb>(
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
            .push_bind_unseparated(format!("%{value}"));
    }
}

fn push_sub_numeric_text_filter<'qb>(
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
            .push_bind_unseparated(value)
            .push_unseparated("::numeric");
    }
}

fn push_sub_i32_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<i32>,
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

fn push_sub_bool_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<bool>,
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

fn push_sub_text_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("]::text[]");
    }
}

fn push_sub_numeric_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric]");
    }
}

pub(crate) fn domain_select_sql() -> &'static str {
    r#"
    select id, name, label_name, labelhash, parent_id, subdomain_count,
           resolved_address_id, resolver_id, ttl, is_migrated, created_at,
           owner_id, registrant_id, wrapped_owner_id, expiry_date
    from domains
    "#
}

pub(crate) fn account_order_column(order_by: AccountOrderField) -> &'static str {
    match order_by {
        AccountOrderField::Id => "id",
    }
}

pub(crate) fn domain_order_column(order_by: DomainOrderField) -> &'static str {
    match order_by {
        DomainOrderField::Id => "id",
        DomainOrderField::Name => "name",
        DomainOrderField::LabelName => "label_name",
        DomainOrderField::SubdomainCount => "subdomain_count",
        DomainOrderField::CreatedAt => "created_at",
        DomainOrderField::ExpiryDate => "expiry_date",
    }
}

pub(crate) fn registration_order_column(order_by: RegistrationOrderField) -> &'static str {
    match order_by {
        RegistrationOrderField::Id => "id",
        RegistrationOrderField::RegistrationDate => "registration_date",
        RegistrationOrderField::ExpiryDate => "expiry_date",
        RegistrationOrderField::Cost => "cost",
        RegistrationOrderField::LabelName => "label_name",
    }
}

pub(crate) fn wrapped_domain_order_column(order_by: WrappedDomainOrderField) -> &'static str {
    match order_by {
        WrappedDomainOrderField::Id => "id",
        WrappedDomainOrderField::ExpiryDate => "expiry_date",
        WrappedDomainOrderField::Fuses => "fuses",
        WrappedDomainOrderField::Name => "name",
    }
}

pub(crate) fn resolver_order_column(order_by: ResolverOrderField) -> &'static str {
    match order_by {
        ResolverOrderField::Id => "id",
        ResolverOrderField::Address => "address",
    }
}

pub(crate) fn event_order_column(order_by: EventOrderField) -> &'static str {
    match order_by {
        EventOrderField::Id => "id",
        EventOrderField::BlockNumber => "block_number",
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

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
}
