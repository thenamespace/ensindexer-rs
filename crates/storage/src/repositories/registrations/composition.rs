use sqlx::{Postgres, query_builder::Separated};

use crate::{
    filters::{AccountFilter, DomainFilter, RegistrationFilter},
    query::{
        account_filter_has_conditions, domain_filter_has_conditions,
        push_account_filter_conditions, push_domain_scalar_filter_conditions,
        push_sub_change_block_filter, push_where_prefix,
    },
    repositories::events::push_registration_events_filter,
};

pub(super) fn push_registration_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    joiner: &'static str,
    filters: Option<Vec<RegistrationFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    let filters: Vec<_> = filters
        .into_iter()
        .filter(registration_filter_has_conditions)
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
        separated.push_unseparated("id in (select id from registrations");
        let mut sub_has_where = false;
        push_registration_subquery_filters(separated, &mut sub_has_where, filter);
        separated.push_unseparated(")");
    }
    separated.push_unseparated(")");
}

pub(crate) fn push_registration_subquery_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: RegistrationFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_filter(separated, has_where, "id", ">", filter.id_gt);
    push_sub_text_filter(separated, has_where, "id", "<", filter.id_lt);
    push_sub_text_filter(separated, has_where, "id", ">=", filter.id_gte);
    push_sub_text_filter(separated, has_where, "id", "<=", filter.id_lte);
    push_sub_text_filter(separated, has_where, "domain_id", "=", filter.domain_id);
    push_sub_domain_relation_filter(separated, has_where, "domain_id", filter.domain_filter);
    push_sub_text_filter(
        separated,
        has_where,
        "registrant_id",
        "=",
        filter.registrant_id,
    );
    push_sub_account_relation_filter(
        separated,
        has_where,
        "registrant_id",
        filter.registrant_filter,
    );
    push_sub_text_filter(separated, has_where, "label_name", "=", filter.label_name);
    push_sub_text_contains_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_contains,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "registration_date",
        "=",
        filter.registration_date,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "registration_date",
        ">",
        filter.registration_date_gt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "registration_date",
        "<",
        filter.registration_date_lt,
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
    push_sub_numeric_text_filter(separated, has_where, "cost", "=", filter.cost);
    push_sub_numeric_text_filter(separated, has_where, "cost", ">", filter.cost_gt);
    push_sub_numeric_text_filter(separated, has_where, "cost", "<", filter.cost_lt);
    push_sub_change_block_filter(
        separated,
        has_where,
        "Registration",
        "id",
        filter.change_block_number_gte,
    );
    push_registration_events_filter(separated, has_where, filter.events_filter);
    push_registration_filter_group(separated, has_where, " and ", filter.and);
    push_registration_filter_group(separated, has_where, " or ", filter.or);
}

pub(crate) fn registration_filter_has_conditions(filter: &RegistrationFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_gt.is_some()
        || filter.id_lt.is_some()
        || filter.id_gte.is_some()
        || filter.id_lte.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.change_block_number_gte.is_some()
        || filter.events_filter.is_some()
        || filter.domain_id.is_some()
        || filter.domain_id_not.is_some()
        || filter.domain_id_contains.is_some()
        || filter
            .domain_filter
            .as_ref()
            .is_some_and(|filter| domain_filter_has_conditions(filter))
        || filter.registrant_id.is_some()
        || filter.registrant_id_not.is_some()
        || filter.registrant_id_contains.is_some()
        || filter
            .registrant_filter
            .as_ref()
            .is_some_and(|filter| account_filter_has_conditions(filter))
        || filter.label_name.is_some()
        || filter.label_name_not.is_some()
        || filter.label_name_contains.is_some()
        || filter.registration_date.is_some()
        || filter.registration_date_gt.is_some()
        || filter.registration_date_lt.is_some()
        || filter.expiry_date.is_some()
        || filter.expiry_date_gt.is_some()
        || filter.expiry_date_lt.is_some()
        || filter.cost.is_some()
        || filter.cost_gt.is_some()
        || filter.cost_lt.is_some()
        || filter
            .and
            .as_ref()
            .is_some_and(|filters| filters.iter().any(registration_filter_has_conditions))
        || filter
            .or
            .as_ref()
            .is_some_and(|filters| filters.iter().any(registration_filter_has_conditions))
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
