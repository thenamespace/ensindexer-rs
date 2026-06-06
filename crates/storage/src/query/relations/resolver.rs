use sqlx::{Postgres, query_builder::Separated};

use crate::{filters::ResolverFilter, query::relations::subquery::*};

pub(super) fn push_resolver_scalar_filter_conditions<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: ResolverFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_in, false);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_not_in, true);
    push_sub_text_filter(separated, has_where, "domain_id", "=", filter.domain_id);
    push_sub_text_filter(separated, has_where, "address", "=", filter.address);
    push_sub_text_array_filter(separated, has_where, "address", filter.address_in, false);
    push_sub_text_filter(separated, has_where, "addr_id", "=", filter.addr_id);
    push_sub_text_filter(
        separated,
        has_where,
        "content_hash",
        "=",
        filter.content_hash,
    );
    push_sub_text_filter(
        separated,
        has_where,
        "content_hash",
        "!=",
        filter.content_hash_not,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_in,
        false,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_not_in,
        true,
    );
    push_sub_text_contains_filter(
        separated,
        has_where,
        "content_hash",
        filter.content_hash_contains,
        false,
    );
    push_sub_text_array_contains_filter(separated, has_where, "texts", filter.texts_contains);
    push_sub_numeric_array_contains_filter(
        separated,
        has_where,
        "coin_types",
        filter.coin_types_contains,
    );
}
