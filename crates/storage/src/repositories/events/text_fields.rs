use sqlx::Postgres;

use crate::{
    filters::{EventFilter, TextOperatorFilter},
    query::*,
};

pub(super) fn push_text_event_field<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
) {
    push_text_field_filters(separated, has_where, column, filter);
}

pub(super) fn text_field_name(filter: &EventFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.name.clone(),
        not: filter.name_not.clone(),
        gt: filter.name_gt.clone(),
        lt: filter.name_lt.clone(),
        gte: filter.name_gte.clone(),
        lte: filter.name_lte.clone(),
        in_values: filter.name_in.clone(),
        not_in: filter.name_not_in.clone(),
        contains: filter.name_contains.clone(),
        contains_nocase: filter.name_contains_nocase.clone(),
        not_contains: filter.name_not_contains.clone(),
        not_contains_nocase: filter.name_not_contains_nocase.clone(),
        starts_with: filter.name_starts_with.clone(),
        starts_with_nocase: filter.name_starts_with_nocase.clone(),
        not_starts_with: filter.name_not_starts_with.clone(),
        not_starts_with_nocase: filter.name_not_starts_with_nocase.clone(),
        ends_with: filter.name_ends_with.clone(),
        ends_with_nocase: filter.name_ends_with_nocase.clone(),
        not_ends_with: filter.name_not_ends_with.clone(),
        not_ends_with_nocase: filter.name_not_ends_with_nocase.clone(),
    }
}

pub(super) fn text_field_key(filter: &EventFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.key.clone(),
        not: filter.key_not.clone(),
        gt: filter.key_gt.clone(),
        lt: filter.key_lt.clone(),
        gte: filter.key_gte.clone(),
        lte: filter.key_lte.clone(),
        in_values: filter.key_in.clone(),
        not_in: filter.key_not_in.clone(),
        contains: filter.key_contains.clone(),
        contains_nocase: filter.key_contains_nocase.clone(),
        not_contains: filter.key_not_contains.clone(),
        not_contains_nocase: filter.key_not_contains_nocase.clone(),
        starts_with: filter.key_starts_with.clone(),
        starts_with_nocase: filter.key_starts_with_nocase.clone(),
        not_starts_with: filter.key_not_starts_with.clone(),
        not_starts_with_nocase: filter.key_not_starts_with_nocase.clone(),
        ends_with: filter.key_ends_with.clone(),
        ends_with_nocase: filter.key_ends_with_nocase.clone(),
        not_ends_with: filter.key_not_ends_with.clone(),
        not_ends_with_nocase: filter.key_not_ends_with_nocase.clone(),
    }
}

pub(super) fn text_field_value(filter: &EventFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.value.clone(),
        not: filter.value_not.clone(),
        gt: filter.value_gt.clone(),
        lt: filter.value_lt.clone(),
        gte: filter.value_gte.clone(),
        lte: filter.value_lte.clone(),
        in_values: filter.value_in.clone(),
        not_in: filter.value_not_in.clone(),
        contains: filter.value_contains.clone(),
        contains_nocase: filter.value_contains_nocase.clone(),
        not_contains: filter.value_not_contains.clone(),
        not_contains_nocase: filter.value_not_contains_nocase.clone(),
        starts_with: filter.value_starts_with.clone(),
        starts_with_nocase: filter.value_starts_with_nocase.clone(),
        not_starts_with: filter.value_not_starts_with.clone(),
        not_starts_with_nocase: filter.value_not_starts_with_nocase.clone(),
        ends_with: filter.value_ends_with.clone(),
        ends_with_nocase: filter.value_ends_with_nocase.clone(),
        not_ends_with: filter.value_not_ends_with.clone(),
        not_ends_with_nocase: filter.value_not_ends_with_nocase.clone(),
    }
}

pub(super) fn text_field_hash(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.hash.clone(),
        filter.hash_not.clone(),
        filter.hash_gt.clone(),
        filter.hash_lt.clone(),
        filter.hash_gte.clone(),
        filter.hash_lte.clone(),
        filter.hash_in.clone(),
        filter.hash_not_in.clone(),
        filter.hash_contains.clone(),
        filter.hash_not_contains.clone(),
    )
}

pub(super) fn text_field_x(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.x.clone(),
        filter.x_not.clone(),
        filter.x_gt.clone(),
        filter.x_lt.clone(),
        filter.x_gte.clone(),
        filter.x_lte.clone(),
        filter.x_in.clone(),
        filter.x_not_in.clone(),
        filter.x_contains.clone(),
        filter.x_not_contains.clone(),
    )
}

pub(super) fn text_field_y(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.y.clone(),
        filter.y_not.clone(),
        filter.y_gt.clone(),
        filter.y_lt.clone(),
        filter.y_gte.clone(),
        filter.y_lte.clone(),
        filter.y_in.clone(),
        filter.y_not_in.clone(),
        filter.y_contains.clone(),
        filter.y_not_contains.clone(),
    )
}

pub(super) fn text_field_owner(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.owner_id.clone(),
        filter.owner_id_not.clone(),
        filter.owner_id_gt.clone(),
        filter.owner_id_lt.clone(),
        filter.owner_id_gte.clone(),
        filter.owner_id_lte.clone(),
        filter.owner_id_in.clone(),
        filter.owner_id_not_in.clone(),
        filter.owner_id_contains.clone(),
        filter.owner_id_not_contains.clone(),
    )
}

pub(super) fn text_field_parent_domain(filter: &EventFilter) -> TextFieldFilter {
    operator_text_field(
        filter.parent_domain_id.clone(),
        &filter.parent_domain_id_ops,
    )
}

pub(super) fn text_field_resolver(filter: &EventFilter) -> TextFieldFilter {
    operator_text_field(filter.resolver_id.clone(), &filter.resolver_id_ops)
}

pub(super) fn text_field_registrant(filter: &EventFilter) -> TextFieldFilter {
    operator_text_field(filter.registrant_id.clone(), &filter.registrant_id_ops)
}

pub(super) fn text_field_new_owner(filter: &EventFilter) -> TextFieldFilter {
    operator_text_field(filter.new_owner_id.clone(), &filter.new_owner_id_ops)
}

pub(super) fn text_field_addr(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.addr_id.clone(),
        filter.addr_id_not.clone(),
        filter.addr_id_gt.clone(),
        filter.addr_id_lt.clone(),
        filter.addr_id_gte.clone(),
        filter.addr_id_lte.clone(),
        filter.addr_id_in.clone(),
        filter.addr_id_not_in.clone(),
        filter.addr_id_contains.clone(),
        filter.addr_id_not_contains.clone(),
    )
}

pub(super) fn text_field_interface_id(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.interface_id.clone(),
        filter.interface_id_not.clone(),
        filter.interface_id_gt.clone(),
        filter.interface_id_lt.clone(),
        filter.interface_id_gte.clone(),
        filter.interface_id_lte.clone(),
        filter.interface_id_in.clone(),
        filter.interface_id_not_in.clone(),
        filter.interface_id_contains.clone(),
        filter.interface_id_not_contains.clone(),
    )
}

pub(super) fn text_field_implementer(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.implementer.clone(),
        filter.implementer_not.clone(),
        filter.implementer_gt.clone(),
        filter.implementer_lt.clone(),
        filter.implementer_gte.clone(),
        filter.implementer_lte.clone(),
        filter.implementer_in.clone(),
        filter.implementer_not_in.clone(),
        filter.implementer_contains.clone(),
        filter.implementer_not_contains.clone(),
    )
}

pub(super) fn text_field_target(filter: &EventFilter) -> TextFieldFilter {
    limited_text_field(
        filter.target.clone(),
        filter.target_not.clone(),
        filter.target_gt.clone(),
        filter.target_lt.clone(),
        filter.target_gte.clone(),
        filter.target_lte.clone(),
        filter.target_in.clone(),
        filter.target_not_in.clone(),
        filter.target_contains.clone(),
        filter.target_not_contains.clone(),
    )
}

fn operator_text_field(exact: Option<String>, ops: &TextOperatorFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact,
        not: ops.not.clone(),
        gt: ops.gt.clone(),
        lt: ops.lt.clone(),
        gte: ops.gte.clone(),
        lte: ops.lte.clone(),
        in_values: ops.in_values.clone(),
        not_in: ops.not_in.clone(),
        contains: ops.contains.clone(),
        contains_nocase: ops.contains_nocase.clone(),
        not_contains: ops.not_contains.clone(),
        not_contains_nocase: ops.not_contains_nocase.clone(),
        starts_with: ops.starts_with.clone(),
        starts_with_nocase: ops.starts_with_nocase.clone(),
        not_starts_with: ops.not_starts_with.clone(),
        not_starts_with_nocase: ops.not_starts_with_nocase.clone(),
        ends_with: ops.ends_with.clone(),
        ends_with_nocase: ops.ends_with_nocase.clone(),
        not_ends_with: ops.not_ends_with.clone(),
        not_ends_with_nocase: ops.not_ends_with_nocase.clone(),
    }
}

#[allow(clippy::too_many_arguments)]
fn limited_text_field(
    exact: Option<String>,
    not: Option<String>,
    gt: Option<String>,
    lt: Option<String>,
    gte: Option<String>,
    lte: Option<String>,
    in_values: Option<Vec<String>>,
    not_in: Option<Vec<String>>,
    contains: Option<String>,
    not_contains: Option<String>,
) -> TextFieldFilter {
    TextFieldFilter {
        exact,
        not,
        gt,
        lt,
        gte,
        lte,
        in_values,
        not_in,
        contains,
        not_contains,
        ..TextFieldFilter::default()
    }
}
