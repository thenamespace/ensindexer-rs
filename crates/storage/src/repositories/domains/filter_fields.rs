use sqlx::{Postgres, query_builder::Separated};

use crate::{filters::DomainFilter, query::*};

macro_rules! text_filter {
    ($filter:expr, {
        exact: $exact:ident,
        not: $not:ident,
        gt: $gt:ident,
        lt: $lt:ident,
        gte: $gte:ident,
        lte: $lte:ident,
        in_values: $in_values:ident,
        not_in: $not_in:ident,
        contains: $contains:ident,
        contains_nocase: $contains_nocase:ident,
        not_contains: $not_contains:ident,
        not_contains_nocase: $not_contains_nocase:ident,
        starts_with: $starts_with:ident,
        starts_with_nocase: $starts_with_nocase:ident,
        not_starts_with: $not_starts_with:ident,
        not_starts_with_nocase: $not_starts_with_nocase:ident,
        ends_with: $ends_with:ident,
        ends_with_nocase: $ends_with_nocase:ident,
        not_ends_with: $not_ends_with:ident,
        not_ends_with_nocase: $not_ends_with_nocase:ident $(,)?
    }) => {
        TextFieldFilter {
            exact: $filter.$exact.take(),
            not: $filter.$not.take(),
            gt: $filter.$gt.take(),
            lt: $filter.$lt.take(),
            gte: $filter.$gte.take(),
            lte: $filter.$lte.take(),
            in_values: $filter.$in_values.take(),
            not_in: $filter.$not_in.take(),
            contains: $filter.$contains.take(),
            contains_nocase: $filter.$contains_nocase.take(),
            not_contains: $filter.$not_contains.take(),
            not_contains_nocase: $filter.$not_contains_nocase.take(),
            starts_with: $filter.$starts_with.take(),
            starts_with_nocase: $filter.$starts_with_nocase.take(),
            not_starts_with: $filter.$not_starts_with.take(),
            not_starts_with_nocase: $filter.$not_starts_with_nocase.take(),
            ends_with: $filter.$ends_with.take(),
            ends_with_nocase: $filter.$ends_with_nocase.take(),
            not_ends_with: $filter.$not_ends_with.take(),
            not_ends_with_nocase: $filter.$not_ends_with_nocase.take(),
        }
    };
}

pub(super) fn push_primary_text_fields<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: &mut DomainFilter,
) {
    push_text_field_filters_hashed_exact(
        separated,
        has_where,
        "name",
        text_filter!(filter, {
            exact: name,
            not: name_not,
            gt: name_gt,
            lt: name_lt,
            gte: name_gte,
            lte: name_lte,
            in_values: name_in,
            not_in: name_not_in,
            contains: name_contains,
            contains_nocase: name_contains_nocase,
            not_contains: name_not_contains,
            not_contains_nocase: name_not_contains_nocase,
            starts_with: name_starts_with,
            starts_with_nocase: name_starts_with_nocase,
            not_starts_with: name_not_starts_with,
            not_starts_with_nocase: name_not_starts_with_nocase,
            ends_with: name_ends_with,
            ends_with_nocase: name_ends_with_nocase,
            not_ends_with: name_not_ends_with,
            not_ends_with_nocase: name_not_ends_with_nocase,
        }),
    );
    push_text_field_filters_hashed_exact(
        separated,
        has_where,
        "label_name",
        text_filter!(filter, {
            exact: label_name,
            not: label_name_not,
            gt: label_name_gt,
            lt: label_name_lt,
            gte: label_name_gte,
            lte: label_name_lte,
            in_values: label_name_in,
            not_in: label_name_not_in,
            contains: label_name_contains,
            contains_nocase: label_name_contains_nocase,
            not_contains: label_name_not_contains,
            not_contains_nocase: label_name_not_contains_nocase,
            starts_with: label_name_starts_with,
            starts_with_nocase: label_name_starts_with_nocase,
            not_starts_with: label_name_not_starts_with,
            not_starts_with_nocase: label_name_not_starts_with_nocase,
            ends_with: label_name_ends_with,
            ends_with_nocase: label_name_ends_with_nocase,
            not_ends_with: label_name_not_ends_with,
            not_ends_with_nocase: label_name_not_ends_with_nocase,
        }),
    );
    push_text_field_filters(
        separated,
        has_where,
        "labelhash",
        TextFieldFilter {
            exact: filter.labelhash.take(),
            not: filter.labelhash_not.take(),
            gt: filter.labelhash_gt.take(),
            lt: filter.labelhash_lt.take(),
            gte: filter.labelhash_gte.take(),
            lte: filter.labelhash_lte.take(),
            in_values: filter.labelhash_in.take(),
            not_in: filter.labelhash_not_in.take(),
            contains: filter.labelhash_contains.take(),
            not_contains: filter.labelhash_not_contains.take(),
            ..TextFieldFilter::default()
        },
    );
}

pub(super) fn push_relation_id_text_fields<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: &mut DomainFilter,
) {
    push_relation_text_field(separated, has_where, "parent_id", parent_filter(filter));
    push_relation_text_field(
        separated,
        has_where,
        "resolved_address_id",
        resolved_address_filter(filter),
    );
    push_relation_text_field(separated, has_where, "owner_id", owner_filter(filter));
    push_relation_text_field(separated, has_where, "resolver_id", resolver_filter(filter));
    push_relation_text_field(
        separated,
        has_where,
        "registrant_id",
        registrant_filter(filter),
    );
    push_relation_text_field(
        separated,
        has_where,
        "wrapped_owner_id",
        wrapped_owner_filter(filter),
    );
}

fn push_relation_text_field<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
) {
    push_text_field_filters(separated, has_where, column, filter);
}

fn parent_filter(filter: &mut DomainFilter) -> TextFieldFilter {
    text_filter!(filter, {
        exact: parent_id,
        not: parent_id_not,
        gt: parent_id_gt,
        lt: parent_id_lt,
        gte: parent_id_gte,
        lte: parent_id_lte,
        in_values: parent_id_in,
        not_in: parent_id_not_in,
        contains: parent_id_contains,
        contains_nocase: parent_id_contains_nocase,
        not_contains: parent_id_not_contains,
        not_contains_nocase: parent_id_not_contains_nocase,
        starts_with: parent_id_starts_with,
        starts_with_nocase: parent_id_starts_with_nocase,
        not_starts_with: parent_id_not_starts_with,
        not_starts_with_nocase: parent_id_not_starts_with_nocase,
        ends_with: parent_id_ends_with,
        ends_with_nocase: parent_id_ends_with_nocase,
        not_ends_with: parent_id_not_ends_with,
        not_ends_with_nocase: parent_id_not_ends_with_nocase,
    })
}

fn resolved_address_filter(filter: &mut DomainFilter) -> TextFieldFilter {
    text_filter!(filter, {
        exact: resolved_address_id,
        not: resolved_address_id_not,
        gt: resolved_address_id_gt,
        lt: resolved_address_id_lt,
        gte: resolved_address_id_gte,
        lte: resolved_address_id_lte,
        in_values: resolved_address_id_in,
        not_in: resolved_address_id_not_in,
        contains: resolved_address_id_contains,
        contains_nocase: resolved_address_id_contains_nocase,
        not_contains: resolved_address_id_not_contains,
        not_contains_nocase: resolved_address_id_not_contains_nocase,
        starts_with: resolved_address_id_starts_with,
        starts_with_nocase: resolved_address_id_starts_with_nocase,
        not_starts_with: resolved_address_id_not_starts_with,
        not_starts_with_nocase: resolved_address_id_not_starts_with_nocase,
        ends_with: resolved_address_id_ends_with,
        ends_with_nocase: resolved_address_id_ends_with_nocase,
        not_ends_with: resolved_address_id_not_ends_with,
        not_ends_with_nocase: resolved_address_id_not_ends_with_nocase,
    })
}

fn owner_filter(filter: &mut DomainFilter) -> TextFieldFilter {
    text_filter!(filter, {
        exact: owner_id,
        not: owner_id_not,
        gt: owner_id_gt,
        lt: owner_id_lt,
        gte: owner_id_gte,
        lte: owner_id_lte,
        in_values: owner_id_in,
        not_in: owner_id_not_in,
        contains: owner_id_contains,
        contains_nocase: owner_id_contains_nocase,
        not_contains: owner_id_not_contains,
        not_contains_nocase: owner_id_not_contains_nocase,
        starts_with: owner_id_starts_with,
        starts_with_nocase: owner_id_starts_with_nocase,
        not_starts_with: owner_id_not_starts_with,
        not_starts_with_nocase: owner_id_not_starts_with_nocase,
        ends_with: owner_id_ends_with,
        ends_with_nocase: owner_id_ends_with_nocase,
        not_ends_with: owner_id_not_ends_with,
        not_ends_with_nocase: owner_id_not_ends_with_nocase,
    })
}

fn resolver_filter(filter: &mut DomainFilter) -> TextFieldFilter {
    text_filter!(filter, {
        exact: resolver_id,
        not: resolver_id_not,
        gt: resolver_id_gt,
        lt: resolver_id_lt,
        gte: resolver_id_gte,
        lte: resolver_id_lte,
        in_values: resolver_id_in,
        not_in: resolver_id_not_in,
        contains: resolver_id_contains,
        contains_nocase: resolver_id_contains_nocase,
        not_contains: resolver_id_not_contains,
        not_contains_nocase: resolver_id_not_contains_nocase,
        starts_with: resolver_id_starts_with,
        starts_with_nocase: resolver_id_starts_with_nocase,
        not_starts_with: resolver_id_not_starts_with,
        not_starts_with_nocase: resolver_id_not_starts_with_nocase,
        ends_with: resolver_id_ends_with,
        ends_with_nocase: resolver_id_ends_with_nocase,
        not_ends_with: resolver_id_not_ends_with,
        not_ends_with_nocase: resolver_id_not_ends_with_nocase,
    })
}

fn registrant_filter(filter: &mut DomainFilter) -> TextFieldFilter {
    text_filter!(filter, {
        exact: registrant_id,
        not: registrant_id_not,
        gt: registrant_id_gt,
        lt: registrant_id_lt,
        gte: registrant_id_gte,
        lte: registrant_id_lte,
        in_values: registrant_id_in,
        not_in: registrant_id_not_in,
        contains: registrant_id_contains,
        contains_nocase: registrant_id_contains_nocase,
        not_contains: registrant_id_not_contains,
        not_contains_nocase: registrant_id_not_contains_nocase,
        starts_with: registrant_id_starts_with,
        starts_with_nocase: registrant_id_starts_with_nocase,
        not_starts_with: registrant_id_not_starts_with,
        not_starts_with_nocase: registrant_id_not_starts_with_nocase,
        ends_with: registrant_id_ends_with,
        ends_with_nocase: registrant_id_ends_with_nocase,
        not_ends_with: registrant_id_not_ends_with,
        not_ends_with_nocase: registrant_id_not_ends_with_nocase,
    })
}

fn wrapped_owner_filter(filter: &mut DomainFilter) -> TextFieldFilter {
    text_filter!(filter, {
        exact: wrapped_owner_id,
        not: wrapped_owner_id_not,
        gt: wrapped_owner_id_gt,
        lt: wrapped_owner_id_lt,
        gte: wrapped_owner_id_gte,
        lte: wrapped_owner_id_lte,
        in_values: wrapped_owner_id_in,
        not_in: wrapped_owner_id_not_in,
        contains: wrapped_owner_id_contains,
        contains_nocase: wrapped_owner_id_contains_nocase,
        not_contains: wrapped_owner_id_not_contains,
        not_contains_nocase: wrapped_owner_id_not_contains_nocase,
        starts_with: wrapped_owner_id_starts_with,
        starts_with_nocase: wrapped_owner_id_starts_with_nocase,
        not_starts_with: wrapped_owner_id_not_starts_with,
        not_starts_with_nocase: wrapped_owner_id_not_starts_with_nocase,
        ends_with: wrapped_owner_id_ends_with,
        ends_with_nocase: wrapped_owner_id_ends_with_nocase,
        not_ends_with: wrapped_owner_id_not_ends_with,
        not_ends_with_nocase: wrapped_owner_id_not_ends_with_nocase,
    })
}
