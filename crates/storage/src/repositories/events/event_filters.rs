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
    push_text_filter(
        separated,
        has_where,
        parent_column,
        filter.parent_id.clone(),
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
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::super::specific_filters::push_event_specific_filters;
    use super::*;

    #[test]
    fn base_event_filters_include_generated_operator_variants() {
        let filter = EventFilter {
            id_gt: Some("event-1".into()),
            block_number_not: Some(10),
            block_number_in: Some(vec![11, 12]),
            transaction_id_gte: Some("0xaaa".into()),
            transaction_id_contains: Some("abc".into()),
            transaction_id_not_contains: Some("def".into()),
            ..EventFilter::default()
        };
        let mut query = QueryBuilder::<Postgres>::new("select id from transfer_events");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_event_filters(&mut separated, &mut has_where, "domain_id", &filter);
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from transfer_events where id > $1 and transaction_id >= $2 and transaction_id like $3 and not (transaction_id like $4) and block_number != $5 and block_number = any($6) "
        );
    }

    #[test]
    fn event_specific_filters_are_added_only_for_matching_table_columns() {
        let filter = EventFilter {
            parent_id: Some("0xdomain".into()),
            owner_id: Some("0xowner".into()),
            fuses_gte: Some(32),
            expiry_date_lt: Some("1000".into()),
            expiry_date_not_in: Some(vec!["2000".into()]),
            ..EventFilter::default()
        };
        let mut query = QueryBuilder::<Postgres>::new("select id from name_wrapped_events");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_event_filters(&mut separated, &mut has_where, "domain_id", &filter);
            push_event_specific_filters(
                &mut separated,
                &mut has_where,
                "name_wrapped_events",
                &filter,
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from name_wrapped_events where domain_id = $1 and fuses >= $2 and owner_id = $3 and expiry_date < $4::numeric and not (expiry_date = any(array[$5::numeric])) "
        );
    }

    #[test]
    fn event_specific_filters_include_numeric_and_bool_variants() {
        let filter = EventFilter {
            content_type_not: Some("1".into()),
            content_type_gte: Some("2".into()),
            content_type_in: Some(vec!["3".into(), "4".into()]),
            is_authorized_not: Some(false),
            is_authorized_not_in: Some(vec![true]),
            ..EventFilter::default()
        };
        let mut query = QueryBuilder::<Postgres>::new("select id from abi_changed_events");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_event_specific_filters(
                &mut separated,
                &mut has_where,
                "abi_changed_events",
                &filter,
            );
            push_event_specific_filters(
                &mut separated,
                &mut has_where,
                "authorisation_changed_events",
                &filter,
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from abi_changed_events where content_type != $1::numeric and content_type >= $2::numeric and content_type = any(array[$3::numeric, $4::numeric]) and is_authorized != $5 and not (is_authorized = any($6)) "
        );
    }
}
