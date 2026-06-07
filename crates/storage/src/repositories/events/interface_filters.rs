use sqlx::Postgres;

use super::{
    specific_filters::{NumericEventField, push_i32_event_filter, push_numeric_event_filter},
    text_fields::{
        push_text_event_field, text_field_addr, text_field_hash, text_field_implementer,
        text_field_interface_id, text_field_key, text_field_name, text_field_new_owner,
        text_field_owner, text_field_parent_domain, text_field_registrant, text_field_resolver,
        text_field_target, text_field_value, text_field_x, text_field_y,
    },
};
use crate::{filters::EventFilter, query::*};

pub(super) fn push_interface_event_specific_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    filter: &EventFilter,
) -> bool {
    match table {
        "domain_event_refs" => {
            push_domain_event_filters(separated, has_where, filter);
            true
        }
        "registration_event_refs" => {
            push_registration_event_filters(separated, has_where, filter);
            true
        }
        "resolver_event_refs" => {
            push_resolver_event_filters(separated, has_where, filter);
            true
        }
        _ => false,
    }
}

fn push_domain_event_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: &EventFilter,
) {
    push_text_event_field(
        separated,
        has_where,
        "parent_domain_id",
        text_field_parent_domain(filter),
    );
    push_text_event_field(
        separated,
        has_where,
        "resolver_id",
        text_field_resolver(filter),
    );
    push_numeric_event_filter(separated, has_where, "ttl", filter, NumericEventField::Ttl);
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

fn push_registration_event_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: &EventFilter,
) {
    push_text_event_field(
        separated,
        has_where,
        "registrant_id",
        text_field_registrant(filter),
    );
    push_text_event_field(
        separated,
        has_where,
        "new_owner_id",
        text_field_new_owner(filter),
    );
    push_numeric_event_filter(
        separated,
        has_where,
        "expiry_date",
        filter,
        NumericEventField::ExpiryDate,
    );
}

fn push_resolver_event_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: &EventFilter,
) {
    push_text_event_field(separated, has_where, "addr_id", text_field_addr(filter));
    push_numeric_event_filter(
        separated,
        has_where,
        "coin_type",
        filter,
        NumericEventField::CoinType,
    );
    push_text_event_field(separated, has_where, "addr", text_field_addr(filter));
    push_text_event_field(separated, has_where, "name", text_field_name(filter));
    push_numeric_event_filter(
        separated,
        has_where,
        "content_type",
        filter,
        NumericEventField::ContentType,
    );
    push_text_event_field(separated, has_where, "x", text_field_x(filter));
    push_text_event_field(separated, has_where, "y", text_field_y(filter));
    push_text_event_field(separated, has_where, "key", text_field_key(filter));
    push_text_event_field(separated, has_where, "value", text_field_value(filter));
    push_text_event_field(separated, has_where, "hash", text_field_hash(filter));
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
    push_numeric_event_filter(
        separated,
        has_where,
        "version",
        filter,
        NumericEventField::Version,
    );
}
