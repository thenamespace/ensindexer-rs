use sqlx::{Postgres, query_builder::Separated};

use crate::{
    filters::AccountFilter,
    query::{
        push_where_prefix,
        relations::{conditions::account_filter_has_conditions, subquery::*},
    },
};

pub(super) fn push_account_filter_conditions<'qb>(
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

pub(super) fn push_account_filter_group<'qb>(
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
