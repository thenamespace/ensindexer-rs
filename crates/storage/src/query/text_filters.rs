use sqlx::{Postgres, query_builder::Separated};

use super::{
    push_text_array_filter, push_text_comparison_filters, push_text_contains_filter,
    push_text_filter, push_text_not_array_filter, push_text_not_contains_filter,
    push_text_not_filter, push_text_not_prefix_filter, push_text_not_suffix_filter,
    push_text_prefix_filter, push_text_prefix_nocase_filter, push_text_suffix_filter,
    push_text_suffix_nocase_filter, push_where_prefix,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct TextFieldFilter {
    pub exact: Option<String>,
    pub not: Option<String>,
    pub gt: Option<String>,
    pub lt: Option<String>,
    pub gte: Option<String>,
    pub lte: Option<String>,
    pub in_values: Option<Vec<String>>,
    pub not_in: Option<Vec<String>>,
    pub contains: Option<String>,
    pub contains_nocase: Option<String>,
    pub not_contains: Option<String>,
    pub not_contains_nocase: Option<String>,
    pub starts_with: Option<String>,
    pub starts_with_nocase: Option<String>,
    pub not_starts_with: Option<String>,
    pub not_starts_with_nocase: Option<String>,
    pub ends_with: Option<String>,
    pub ends_with_nocase: Option<String>,
    pub not_ends_with: Option<String>,
    pub not_ends_with_nocase: Option<String>,
}

pub(crate) fn push_text_field_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
) {
    push_text_field_filters_inner(separated, has_where, column, filter, false);
}

pub(crate) fn push_text_field_filters_hashed_exact<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
) {
    push_text_field_filters_inner(separated, has_where, column, filter, true);
}

fn push_text_field_filters_inner<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
    hashed_exact: bool,
) {
    if hashed_exact {
        push_text_hashed_exact_filter(separated, has_where, column, filter.exact);
    } else {
        push_text_filter(separated, has_where, column, filter.exact);
    }
    push_text_not_filter(separated, has_where, column, filter.not);
    push_text_comparison_filters(
        separated, has_where, column, filter.gt, filter.lt, filter.gte, filter.lte,
    );
    if hashed_exact {
        push_text_hashed_exact_array_filter(separated, has_where, column, filter.in_values);
    } else {
        push_text_array_filter(separated, has_where, column, filter.in_values);
    }
    push_text_not_array_filter(separated, has_where, column, filter.not_in);
    push_text_contains_filter(separated, has_where, column, filter.contains, false);
    push_text_contains_filter(separated, has_where, column, filter.contains_nocase, true);
    push_text_not_contains_filter(separated, has_where, column, filter.not_contains, false);
    push_text_not_contains_filter(
        separated,
        has_where,
        column,
        filter.not_contains_nocase,
        true,
    );
    push_text_prefix_filter(separated, has_where, column, filter.starts_with);
    push_text_prefix_nocase_filter(separated, has_where, column, filter.starts_with_nocase);
    push_text_not_prefix_filter(separated, has_where, column, filter.not_starts_with, false);
    push_text_not_prefix_filter(
        separated,
        has_where,
        column,
        filter.not_starts_with_nocase,
        true,
    );
    push_text_suffix_filter(separated, has_where, column, filter.ends_with);
    push_text_suffix_nocase_filter(separated, has_where, column, filter.ends_with_nocase);
    push_text_not_suffix_filter(separated, has_where, column, filter.not_ends_with, false);
    push_text_not_suffix_filter(
        separated,
        has_where,
        column,
        filter.not_ends_with_nocase,
        true,
    );
}

fn push_text_hashed_exact_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push("md5(")
            .push_unseparated(column)
            .push_unseparated(") = md5(")
            .push_bind_unseparated(value.clone())
            .push_unseparated(") and ")
            .push_unseparated(column)
            .push_unseparated(" = ")
            .push_bind_unseparated(value);
    }
}

fn push_text_hashed_exact_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    values: Option<Vec<String>>,
) {
    let Some(values) = values.filter(|values| !values.is_empty()) else {
        return;
    };

    push_where_prefix(separated, has_where);
    separated
        .push("md5(")
        .push_unseparated(column)
        .push_unseparated(") = any(array[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            separated.push_unseparated(", ");
        }
        separated
            .push_unseparated("md5(")
            .push_bind_unseparated(value)
            .push_unseparated(")");
    }
    separated
        .push_unseparated("]) and ")
        .push_unseparated(column)
        .push_unseparated(" = any(")
        .push_bind_unseparated(values)
        .push_unseparated(")");
}
