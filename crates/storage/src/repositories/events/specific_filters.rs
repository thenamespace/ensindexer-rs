use sqlx::Postgres;

use super::interface_filters::push_interface_event_specific_filters;
use super::text_fields::{
    push_text_event_field, text_field_addr, text_field_hash, text_field_implementer,
    text_field_interface_id, text_field_key, text_field_name, text_field_owner, text_field_target,
    text_field_value, text_field_x, text_field_y,
};
use crate::{filters::EventFilter, query::*};

pub(super) fn push_event_specific_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    filter: &EventFilter,
) {
    if push_interface_event_specific_filters(separated, has_where, table, filter) {
        return;
    }

    match table {
        "transfer_events" | "wrapped_transfer_events" | "name_unwrapped_events" => {
            push_text_event_field(separated, has_where, "owner_id", text_field_owner(filter));
        }
        "new_owner_events" => {
            push_text_filter(
                separated,
                has_where,
                "parent_domain_id",
                filter.parent_domain_id.clone(),
            );
            push_text_event_field(separated, has_where, "owner_id", text_field_owner(filter));
        }
        "new_resolver_events" => {
            push_text_filter(
                separated,
                has_where,
                "resolver_id",
                filter.resolver_id.clone(),
            );
        }
        "new_ttl_events" => {
            push_numeric_event_filter(separated, has_where, "ttl", filter, NumericEventField::Ttl);
        }
        "name_wrapped_events" => {
            push_text_event_field(separated, has_where, "name", text_field_name(filter));
            push_i32_event_filter(separated, has_where, "fuses", filter);
            push_text_event_field(separated, has_where, "owner_id", text_field_owner(filter));
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "fuses_set_events" => {
            push_i32_event_filter(separated, has_where, "fuses", filter);
        }
        "expiry_extended_events" | "name_renewed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "name_registered_events" => {
            push_account_event_filter(
                separated,
                has_where,
                "registrant_id",
                filter.registrant_id.clone(),
            );
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "name_transferred_events" => {
            push_account_event_filter(
                separated,
                has_where,
                "new_owner_id",
                filter.new_owner_id.clone(),
            );
        }
        "addr_changed_events" => {
            push_text_event_field(separated, has_where, "addr_id", text_field_addr(filter));
        }
        "multicoin_addr_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "coin_type",
                filter,
                NumericEventField::CoinType,
            );
            push_text_event_field(separated, has_where, "addr", text_field_addr(filter));
        }
        "name_changed_events" => {
            push_text_event_field(separated, has_where, "name", text_field_name(filter));
        }
        "abi_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "content_type",
                filter,
                NumericEventField::ContentType,
            );
        }
        "pubkey_changed_events" => {
            push_text_event_field(separated, has_where, "x", text_field_x(filter));
            push_text_event_field(separated, has_where, "y", text_field_y(filter));
        }
        "text_changed_events" => {
            push_text_event_field(separated, has_where, "key", text_field_key(filter));
            push_text_event_field(separated, has_where, "value", text_field_value(filter));
        }
        "contenthash_changed_events" => {
            push_text_event_field(separated, has_where, "hash", text_field_hash(filter));
        }
        "interface_changed_events" => {
            push_text_event_field(
                separated,
                has_where,
                "interface_id",
                text_field_interface_id(filter),
            );
            push_text_event_field(
                separated,
                has_where,
                "implementer",
                text_field_implementer(filter),
            );
        }
        "authorisation_changed_events" => {
            push_text_event_field(separated, has_where, "owner", text_field_owner(filter));
            push_text_event_field(separated, has_where, "target", text_field_target(filter));
            push_bool_filter(
                separated,
                has_where,
                "is_authorized",
                "=",
                filter.is_authorized,
            );
            push_bool_filter(
                separated,
                has_where,
                "is_authorized",
                "!=",
                filter.is_authorized_not,
            );
            push_bool_array_filter(
                separated,
                has_where,
                "is_authorized",
                filter.is_authorized_in.clone(),
                false,
            );
            push_bool_array_filter(
                separated,
                has_where,
                "is_authorized",
                filter.is_authorized_not_in.clone(),
                true,
            );
        }
        "version_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "version",
                filter,
                NumericEventField::Version,
            );
        }
        _ => {}
    }
}

pub(super) fn push_account_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    push_text_filter(separated, has_where, column, value);
}

pub(super) fn push_i32_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
) {
    push_i32_filter(separated, has_where, column, "=", filter.fuses);
    push_i32_filter(separated, has_where, column, "!=", filter.fuses_not);
    push_i32_filter(separated, has_where, column, ">", filter.fuses_gt);
    push_i32_filter(separated, has_where, column, "<", filter.fuses_lt);
    push_i32_filter(separated, has_where, column, ">=", filter.fuses_gte);
    push_i32_filter(separated, has_where, column, "<=", filter.fuses_lte);
    push_i32_array_filter(separated, has_where, column, filter.fuses_in.clone(), false);
    push_i32_array_filter(
        separated,
        has_where,
        column,
        filter.fuses_not_in.clone(),
        true,
    );
}

pub(super) enum NumericEventField {
    Ttl,
    ExpiryDate,
    CoinType,
    ContentType,
    Version,
}

pub(super) fn push_numeric_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
    field: NumericEventField,
) {
    let (eq, not, gt, lt, gte, lte, in_values, not_in) = match field {
        NumericEventField::Ttl => (
            filter.ttl.clone(),
            filter.ttl_not.clone(),
            filter.ttl_gt.clone(),
            filter.ttl_lt.clone(),
            filter.ttl_gte.clone(),
            filter.ttl_lte.clone(),
            filter.ttl_in.clone(),
            filter.ttl_not_in.clone(),
        ),
        NumericEventField::ExpiryDate => (
            filter.expiry_date.clone(),
            filter.expiry_date_not.clone(),
            filter.expiry_date_gt.clone(),
            filter.expiry_date_lt.clone(),
            filter.expiry_date_gte.clone(),
            filter.expiry_date_lte.clone(),
            filter.expiry_date_in.clone(),
            filter.expiry_date_not_in.clone(),
        ),
        NumericEventField::CoinType => (
            filter.coin_type.clone(),
            filter.coin_type_not.clone(),
            filter.coin_type_gt.clone(),
            filter.coin_type_lt.clone(),
            filter.coin_type_gte.clone(),
            filter.coin_type_lte.clone(),
            filter.coin_type_in.clone(),
            filter.coin_type_not_in.clone(),
        ),
        NumericEventField::ContentType => (
            filter.content_type.clone(),
            filter.content_type_not.clone(),
            filter.content_type_gt.clone(),
            filter.content_type_lt.clone(),
            filter.content_type_gte.clone(),
            filter.content_type_lte.clone(),
            filter.content_type_in.clone(),
            filter.content_type_not_in.clone(),
        ),
        NumericEventField::Version => (
            filter.version.clone(),
            filter.version_not.clone(),
            filter.version_gt.clone(),
            filter.version_lt.clone(),
            filter.version_gte.clone(),
            filter.version_lte.clone(),
            filter.version_in.clone(),
            filter.version_not_in.clone(),
        ),
    };

    push_numeric_text_filter(separated, has_where, column, "=", eq);
    push_numeric_text_filter(separated, has_where, column, "!=", not);
    push_numeric_text_filter(separated, has_where, column, ">", gt);
    push_numeric_text_filter(separated, has_where, column, "<", lt);
    push_numeric_text_filter(separated, has_where, column, ">=", gte);
    push_numeric_text_filter(separated, has_where, column, "<=", lte);
    push_numeric_text_array_filter(separated, has_where, column, in_values, false);
    push_numeric_text_array_filter(separated, has_where, column, not_in, true);
}
