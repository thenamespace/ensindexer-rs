use sqlx::Postgres;

use crate::{filters::EventFilter, query::*};

pub(super) fn push_event_specific_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    filter: &EventFilter,
) {
    match table {
        "transfer_events" | "wrapped_transfer_events" | "name_unwrapped_events" => {
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
        }
        "new_owner_events" => {
            push_text_filter(
                separated,
                has_where,
                "parent_domain_id",
                filter.parent_domain_id.clone(),
            );
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
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
            push_text_filter(separated, has_where, "name", filter.name.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains.clone(),
                false,
            );
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains_nocase.clone(),
                true,
            );
            push_i32_event_filter(separated, has_where, "fuses", filter);
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
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
            push_account_event_filter(separated, has_where, "addr_id", filter.addr_id.clone());
        }
        "multicoin_addr_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "coin_type",
                filter,
                NumericEventField::CoinType,
            );
            push_text_filter(separated, has_where, "addr", filter.addr_id.clone());
        }
        "name_changed_events" => {
            push_text_filter(separated, has_where, "name", filter.name.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains.clone(),
                false,
            );
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains_nocase.clone(),
                true,
            );
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
            push_text_filter(separated, has_where, "x", filter.x.clone());
            push_text_filter(separated, has_where, "y", filter.y.clone());
        }
        "text_changed_events" => {
            push_text_filter(separated, has_where, "key", filter.key.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "key",
                filter.key_contains.clone(),
                false,
            );
            push_text_filter(separated, has_where, "value", filter.value.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "value",
                filter.value_contains.clone(),
                false,
            );
        }
        "contenthash_changed_events" => {
            push_text_filter(separated, has_where, "hash", filter.hash.clone());
        }
        "interface_changed_events" => {
            push_text_filter(
                separated,
                has_where,
                "interface_id",
                filter.interface_id.clone(),
            );
            push_text_filter(
                separated,
                has_where,
                "implementer",
                filter.implementer.clone(),
            );
        }
        "authorisation_changed_events" => {
            push_text_filter(separated, has_where, "owner", filter.owner_id.clone());
            push_text_filter(separated, has_where, "target", filter.target.clone());
            push_bool_filter(
                separated,
                has_where,
                "is_authorized",
                "=",
                filter.is_authorized,
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

fn push_account_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    push_text_filter(separated, has_where, column, value);
}

fn push_i32_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
) {
    push_i32_filter(separated, has_where, column, "=", filter.fuses);
    push_i32_filter(separated, has_where, column, ">", filter.fuses_gt);
    push_i32_filter(separated, has_where, column, "<", filter.fuses_lt);
    push_i32_filter(separated, has_where, column, ">=", filter.fuses_gte);
    push_i32_filter(separated, has_where, column, "<=", filter.fuses_lte);
}

enum NumericEventField {
    Ttl,
    ExpiryDate,
    CoinType,
    ContentType,
    Version,
}

fn push_numeric_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
    field: NumericEventField,
) {
    let (eq, gt, lt, gte, lte) = match field {
        NumericEventField::Ttl => (
            filter.ttl.clone(),
            filter.ttl_gt.clone(),
            filter.ttl_lt.clone(),
            filter.ttl_gte.clone(),
            filter.ttl_lte.clone(),
        ),
        NumericEventField::ExpiryDate => (
            filter.expiry_date.clone(),
            filter.expiry_date_gt.clone(),
            filter.expiry_date_lt.clone(),
            filter.expiry_date_gte.clone(),
            filter.expiry_date_lte.clone(),
        ),
        NumericEventField::CoinType => (
            filter.coin_type.clone(),
            filter.coin_type_gt.clone(),
            filter.coin_type_lt.clone(),
            None,
            None,
        ),
        NumericEventField::ContentType => (
            filter.content_type.clone(),
            filter.content_type_gt.clone(),
            filter.content_type_lt.clone(),
            None,
            None,
        ),
        NumericEventField::Version => (
            filter.version.clone(),
            filter.version_gt.clone(),
            filter.version_lt.clone(),
            None,
            None,
        ),
    };

    push_numeric_text_filter(separated, has_where, column, "=", eq);
    push_numeric_text_filter(separated, has_where, column, ">", gt);
    push_numeric_text_filter(separated, has_where, column, "<", lt);
    push_numeric_text_filter(separated, has_where, column, ">=", gte);
    push_numeric_text_filter(separated, has_where, column, "<=", lte);
}
