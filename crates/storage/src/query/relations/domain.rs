use sqlx::{Postgres, query_builder::Separated};

use crate::{filters::DomainFilter, query::relations::subquery::*};

pub(super) fn push_domain_scalar_filter_conditions<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: DomainFilter,
) {
    push_sub_text_filter(separated, has_where, "id", "=", filter.id);
    push_sub_text_filter(separated, has_where, "id", "!=", filter.id_not);
    push_sub_text_filter(separated, has_where, "id", ">", filter.id_gt);
    push_sub_text_filter(separated, has_where, "id", "<", filter.id_lt);
    push_sub_text_filter(separated, has_where, "id", ">=", filter.id_gte);
    push_sub_text_filter(separated, has_where, "id", "<=", filter.id_lte);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_in, false);
    push_sub_text_array_filter(separated, has_where, "id", filter.id_not_in, true);
    push_sub_text_filter(separated, has_where, "name", "=", filter.name);
    push_sub_text_contains_filter(separated, has_where, "name", filter.name_contains, false);
    push_sub_text_contains_filter(
        separated,
        has_where,
        "name",
        filter.name_contains_nocase,
        true,
    );
    push_sub_text_prefix_filter(separated, has_where, "name", filter.name_starts_with);
    push_sub_text_suffix_filter(separated, has_where, "name", filter.name_ends_with);
    push_sub_text_filter(separated, has_where, "label_name", "=", filter.label_name);
    push_sub_text_contains_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_contains,
        false,
    );
    push_sub_text_contains_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_contains_nocase,
        true,
    );
    push_sub_text_prefix_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_starts_with,
    );
    push_sub_text_suffix_filter(
        separated,
        has_where,
        "label_name",
        filter.label_name_ends_with,
    );
    push_sub_text_filter(separated, has_where, "labelhash", "=", filter.labelhash);
    push_sub_text_filter(
        separated,
        has_where,
        "labelhash",
        "!=",
        filter.labelhash_not,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "labelhash",
        filter.labelhash_in,
        false,
    );
    push_sub_text_array_filter(
        separated,
        has_where,
        "labelhash",
        filter.labelhash_not_in,
        true,
    );
    push_sub_text_filter(separated, has_where, "parent_id", "=", filter.parent_id);
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        "=",
        filter.subdomain_count,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        ">",
        filter.subdomain_count_gt,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        "<",
        filter.subdomain_count_lt,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        ">=",
        filter.subdomain_count_gte,
    );
    push_sub_i32_filter(
        separated,
        has_where,
        "subdomain_count",
        "<=",
        filter.subdomain_count_lte,
    );
    push_sub_text_filter(
        separated,
        has_where,
        "resolved_address_id",
        "=",
        filter.resolved_address_id,
    );
    push_sub_text_filter(separated, has_where, "owner_id", "=", filter.owner_id);
    push_sub_text_filter(separated, has_where, "resolver_id", "=", filter.resolver_id);
    push_sub_text_filter(
        separated,
        has_where,
        "registrant_id",
        "=",
        filter.registrant_id,
    );
    push_sub_text_filter(
        separated,
        has_where,
        "wrapped_owner_id",
        "=",
        filter.wrapped_owner_id,
    );
    push_sub_bool_filter(separated, has_where, "is_migrated", "=", filter.is_migrated);
    push_sub_bool_filter(
        separated,
        has_where,
        "is_migrated",
        "!=",
        filter.is_migrated_not,
    );
    push_sub_numeric_text_filter(separated, has_where, "created_at", "=", filter.created_at);
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        ">",
        filter.created_at_gt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        "<",
        filter.created_at_lt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        ">=",
        filter.created_at_gte,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "created_at",
        "<=",
        filter.created_at_lte,
    );
    push_sub_numeric_text_filter(separated, has_where, "expiry_date", "=", filter.expiry_date);
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        ">",
        filter.expiry_date_gt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "<",
        filter.expiry_date_lt,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        ">=",
        filter.expiry_date_gte,
    );
    push_sub_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "<=",
        filter.expiry_date_lte,
    );
    push_sub_numeric_text_filter(separated, has_where, "ttl", "=", filter.ttl);
    push_sub_numeric_text_filter(separated, has_where, "ttl", ">", filter.ttl_gt);
    push_sub_numeric_text_filter(separated, has_where, "ttl", "<", filter.ttl_lt);
    push_sub_numeric_text_filter(separated, has_where, "ttl", ">=", filter.ttl_gte);
    push_sub_numeric_text_filter(separated, has_where, "ttl", "<=", filter.ttl_lte);
}
