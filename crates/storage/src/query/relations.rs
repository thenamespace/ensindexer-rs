use sqlx::{Postgres, query_builder::Separated};

mod account;
mod conditions;
mod domain;
mod resolver;
mod subquery;

use crate::{
    filters::{AccountFilter, DomainFilter, ResolverFilter},
    query::{
        push_text_array_filter, push_text_comparison_filters, push_text_filter,
        push_text_not_array_filter, push_text_not_filter, push_where_prefix,
    },
};

pub(crate) use self::{
    account::push_account_filter_conditions,
    conditions::{
        account_filter_has_conditions, domain_filter_has_conditions,
        resolver_filter_has_conditions, resolver_filter_has_scalar_conditions,
    },
    domain::{push_domain_filter_group, push_domain_scalar_filter_conditions},
    resolver::push_resolver_scalar_filter_conditions,
    subquery::push_sub_change_block_filter,
};

use self::account::push_account_filter_group;

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
    let change_block_number_gte = filter.change_block_number_gte;
    push_text_filter(separated, has_where, "id", filter.id);
    push_text_not_filter(separated, has_where, "id", filter.id_not);
    push_text_comparison_filters(
        separated,
        has_where,
        "id",
        filter.id_gt,
        filter.id_lt,
        filter.id_gte,
        filter.id_lte,
    );
    push_text_array_filter(separated, has_where, "id", filter.id_in);
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in);
    push_change_block_filter(
        separated,
        has_where,
        "Account",
        "accounts.id",
        change_block_number_gte,
    );
    push_account_filter_group(separated, has_where, " and ", filter.and);
    push_account_filter_group(separated, has_where, " or ", filter.or);
}

pub(crate) fn push_change_block_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    entity_type: &'static str,
    id_column: &'static str,
    number_gte: Option<i32>,
) {
    let Some(number_gte) = number_gte else {
        return;
    };

    push_where_prefix(separated, has_where);
    separated
        .push("exists (select 1 from entity_changes where entity_type = ")
        .push_bind_unseparated(entity_type)
        .push_unseparated(" and entity_id = ")
        .push_unseparated(id_column)
        .push_unseparated(" and block_number >= ")
        .push_bind_unseparated(number_gte)
        .push_unseparated(")");
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
    if !domain_filter_has_conditions(&filter) {
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
