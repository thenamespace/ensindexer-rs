use sqlx::Postgres;

use crate::{filters::*, query::*};

pub(super) fn push_event_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    parent_column: &'static str,
    filter: &EventFilter,
) {
    push_text_filter(separated, has_where, "id", filter.id.clone());
    push_text_not_filter(separated, has_where, "id", filter.id_not.clone());
    push_text_comparison_filters(
        separated,
        has_where,
        "id",
        filter.id_gt.clone(),
        filter.id_lt.clone(),
        filter.id_gte.clone(),
        filter.id_lte.clone(),
    );
    push_text_array_filter(separated, has_where, "id", filter.id_in.clone());
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in.clone());
    push_text_field_filters(
        separated,
        has_where,
        parent_column,
        TextFieldFilter {
            exact: filter.parent_id.clone(),
            not: filter.parent_id_not.clone(),
            gt: filter.parent_id_gt.clone(),
            lt: filter.parent_id_lt.clone(),
            gte: filter.parent_id_gte.clone(),
            lte: filter.parent_id_lte.clone(),
            in_values: filter.parent_id_in.clone(),
            not_in: filter.parent_id_not_in.clone(),
            contains: filter.parent_id_contains.clone(),
            contains_nocase: filter.parent_id_contains_nocase.clone(),
            not_contains: filter.parent_id_not_contains.clone(),
            not_contains_nocase: filter.parent_id_not_contains_nocase.clone(),
            starts_with: filter.parent_id_starts_with.clone(),
            starts_with_nocase: filter.parent_id_starts_with_nocase.clone(),
            not_starts_with: filter.parent_id_not_starts_with.clone(),
            not_starts_with_nocase: filter.parent_id_not_starts_with_nocase.clone(),
            ends_with: filter.parent_id_ends_with.clone(),
            ends_with_nocase: filter.parent_id_ends_with_nocase.clone(),
            not_ends_with: filter.parent_id_not_ends_with.clone(),
            not_ends_with_nocase: filter.parent_id_not_ends_with_nocase.clone(),
        },
    );
    push_text_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id.clone(),
    );
    push_text_not_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_not.clone(),
    );
    push_text_comparison_filters(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_gt.clone(),
        filter.transaction_id_lt.clone(),
        filter.transaction_id_gte.clone(),
        filter.transaction_id_lte.clone(),
    );
    push_text_array_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_in.clone(),
    );
    push_text_not_array_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_not_in.clone(),
    );
    push_text_contains_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_contains.clone(),
        false,
    );
    push_text_not_contains_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_not_contains.clone(),
        false,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "=",
        filter.block_number,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "!=",
        filter.block_number_not,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        ">",
        filter.block_number_gt,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "<",
        filter.block_number_lt,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        ">=",
        filter.block_number_gte,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "<=",
        filter.block_number_lte,
    );
    push_i32_array_filter(
        separated,
        has_where,
        "block_number",
        filter.block_number_in.clone(),
        false,
    );
    push_i32_array_filter(
        separated,
        has_where,
        "block_number",
        filter.block_number_not_in.clone(),
        true,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        ">=",
        filter.change_block_number_gte,
    );
}

#[cfg(test)]
mod tests;
