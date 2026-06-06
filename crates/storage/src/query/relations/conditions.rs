use crate::filters::{AccountFilter, DomainFilter, ResolverFilter};

pub(super) fn account_filter_has_conditions(filter: &AccountFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_gt.is_some()
        || filter.id_lt.is_some()
        || filter.id_gte.is_some()
        || filter.id_lte.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter
            .and
            .as_ref()
            .is_some_and(|filters| filters.iter().any(account_filter_has_conditions))
        || filter
            .or
            .as_ref()
            .is_some_and(|filters| filters.iter().any(account_filter_has_conditions))
}

pub(super) fn domain_filter_has_scalar_conditions(filter: &DomainFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_gt.is_some()
        || filter.id_lt.is_some()
        || filter.id_gte.is_some()
        || filter.id_lte.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.name.is_some()
        || filter.name_contains.is_some()
        || filter.name_contains_nocase.is_some()
        || filter.name_starts_with.is_some()
        || filter.name_ends_with.is_some()
        || filter.label_name.is_some()
        || filter.label_name_contains.is_some()
        || filter.label_name_contains_nocase.is_some()
        || filter.label_name_starts_with.is_some()
        || filter.label_name_ends_with.is_some()
        || filter.labelhash.is_some()
        || filter.labelhash_not.is_some()
        || filter
            .labelhash_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter
            .labelhash_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.parent_id.is_some()
        || filter.subdomain_count.is_some()
        || filter.subdomain_count_gt.is_some()
        || filter.subdomain_count_lt.is_some()
        || filter.subdomain_count_gte.is_some()
        || filter.subdomain_count_lte.is_some()
        || filter.resolved_address_id.is_some()
        || filter.owner_id.is_some()
        || filter.resolver_id.is_some()
        || filter.registrant_id.is_some()
        || filter.wrapped_owner_id.is_some()
        || filter.is_migrated.is_some()
        || filter.is_migrated_not.is_some()
        || filter.created_at.is_some()
        || filter.created_at_gt.is_some()
        || filter.created_at_lt.is_some()
        || filter.created_at_gte.is_some()
        || filter.created_at_lte.is_some()
        || filter.expiry_date.is_some()
        || filter.expiry_date_gt.is_some()
        || filter.expiry_date_lt.is_some()
        || filter.expiry_date_gte.is_some()
        || filter.expiry_date_lte.is_some()
        || filter.ttl.is_some()
        || filter.ttl_gt.is_some()
        || filter.ttl_lt.is_some()
        || filter.ttl_gte.is_some()
        || filter.ttl_lte.is_some()
}

pub(super) fn resolver_filter_has_scalar_conditions(filter: &ResolverFilter) -> bool {
    filter.id.is_some()
        || filter.id_not.is_some()
        || filter.id_gt.is_some()
        || filter.id_lt.is_some()
        || filter.id_gte.is_some()
        || filter.id_lte.is_some()
        || filter.id_in.as_ref().is_some_and(|value| !value.is_empty())
        || filter
            .id_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.domain_id.is_some()
        || filter.address.is_some()
        || filter
            .address_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.addr_id.is_some()
        || filter.content_hash.is_some()
        || filter.content_hash_not.is_some()
        || filter
            .content_hash_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter
            .content_hash_not_in
            .as_ref()
            .is_some_and(|value| !value.is_empty())
        || filter.content_hash_contains.is_some()
        || filter.texts_contains.is_some()
        || filter.coin_types_contains.is_some()
}
