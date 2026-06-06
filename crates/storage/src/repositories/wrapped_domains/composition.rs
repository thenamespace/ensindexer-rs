use sqlx::{Postgres, query_builder::Separated};

use crate::{filters::WrappedDomainFilter, query::push_where_prefix};

pub(super) fn push_wrapped_domain_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    joiner: &'static str,
    filters: Option<Vec<WrappedDomainFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    let filters: Vec<_> = filters
        .into_iter()
        .filter(wrapped_domain_filter_has_conditions)
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
        separated.push_unseparated("id in (select id from wrapped_domains");
        let mut sub_has_where = false;
        push_wrapped_domain_subquery_filters(separated, &mut sub_has_where, filter);
        separated.push_unseparated(")");
    }
    separated.push_unseparated(")");
}

fn push_wrapped_domain_subquery_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: WrappedDomainFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_filter(separated, has_where, "id", ">", filter.id_gt);
    push_sub_text_filter(separated, has_where, "id", "<", filter.id_lt);
    push_sub_text_filter(separated, has_where, "domain_id", "=", filter.domain_id);
    push_sub_text_contains_filter(separated, has_where, "domain_id", filter.domain_id_contains);
    push_sub_text_filter(separated, has_where, "owner_id", "=", filter.owner_id);
    push_sub_text_contains_filter(separated, has_where, "owner_id", filter.owner_id_contains);
    push_sub_text_filter(separated, has_where, "name", "=", filter.name);
    push_sub_text_contains_filter(separated, has_where, "name", filter.name_contains);
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
    push_sub_i32_filter(separated, has_where, "fuses", "=", filter.fuses);
    push_sub_i32_filter(separated, has_where, "fuses", ">", filter.fuses_gt);
    push_sub_i32_filter(separated, has_where, "fuses", "<", filter.fuses_lt);
    push_wrapped_domain_filter_group(separated, has_where, " and ", filter.and);
    push_wrapped_domain_filter_group(separated, has_where, " or ", filter.or);
}

fn wrapped_domain_filter_has_conditions(filter: &WrappedDomainFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_gt.is_some()
        || filter.id_lt.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.domain_id.is_some()
        || filter.domain_id_contains.is_some()
        || filter.domain_filter.is_some()
        || filter.owner_id.is_some()
        || filter.owner_id_contains.is_some()
        || filter.owner_filter.is_some()
        || filter.name.is_some()
        || filter.name_contains.is_some()
        || filter.expiry_date.is_some()
        || filter.expiry_date_gt.is_some()
        || filter.expiry_date_lt.is_some()
        || filter.fuses.is_some()
        || filter.fuses_gt.is_some()
        || filter.fuses_lt.is_some()
        || filter
            .and
            .as_ref()
            .is_some_and(|filters| filters.iter().any(wrapped_domain_filter_has_conditions))
        || filter
            .or
            .as_ref()
            .is_some_and(|filters| filters.iter().any(wrapped_domain_filter_has_conditions))
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
