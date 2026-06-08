pub(super) struct ReplayIndex {
    pub(super) name: &'static str,
    pub(super) create_sql: &'static str,
}

pub(super) const DEPRECATED_BULK_REPLAY_INDEXES: &[&str] =
    &["domains_name_lower_idx", "domains_label_name_lower_idx"];

macro_rules! index {
    ($name:literal, $table:literal, $column:literal) => {
        ReplayIndex {
            name: $name,
            create_sql: concat!(
                "create index if not exists ",
                $name,
                " on ",
                $table,
                "(",
                $column,
                ")"
            ),
        }
    };
}

macro_rules! expression_index {
    ($name:literal, $table:literal, $expression:literal) => {
        ReplayIndex {
            name: $name,
            create_sql: concat!(
                "create index if not exists ",
                $name,
                " on ",
                $table,
                "(",
                $expression,
                ")"
            ),
        }
    };
}

macro_rules! gin_expression_index {
    ($name:literal, $table:literal, $expression:literal) => {
        ReplayIndex {
            name: $name,
            create_sql: concat!(
                "create index if not exists ",
                $name,
                " on ",
                $table,
                " using gin (",
                $expression,
                ")"
            ),
        }
    };
}

pub(super) const BULK_REPLAY_INDEXES: &[ReplayIndex] = &[
    index!("domains_parent_idx", "domains", "parent_id"),
    index!("domains_owner_idx", "domains", "owner_id"),
    index!("domains_resolver_idx", "domains", "resolver_id"),
    index!("domains_registrant_idx", "domains", "registrant_id"),
    index!("domains_wrapped_owner_idx", "domains", "wrapped_owner_id"),
    index!(
        "domains_resolved_address_idx",
        "domains",
        "resolved_address_id"
    ),
    index!("domains_expiry_date_idx", "domains", "expiry_date"),
    index!("domains_created_at_idx", "domains", "created_at"),
    index!(
        "domains_owner_expiry_idx",
        "domains",
        "owner_id, expiry_date desc, id"
    ),
    index!(
        "domains_registrant_expiry_idx",
        "domains",
        "registrant_id, expiry_date desc, id"
    ),
    index!(
        "domains_wrapped_owner_expiry_idx",
        "domains",
        "wrapped_owner_id, expiry_date desc, id"
    ),
    index!(
        "domains_resolved_address_expiry_idx",
        "domains",
        "resolved_address_id, expiry_date desc, id"
    ),
    index!(
        "domains_owner_created_idx",
        "domains",
        "owner_id, created_at desc, id"
    ),
    index!(
        "domains_registrant_created_idx",
        "domains",
        "registrant_id, created_at desc, id"
    ),
    index!(
        "domains_wrapped_owner_created_idx",
        "domains",
        "wrapped_owner_id, created_at desc, id"
    ),
    index!(
        "domains_resolved_address_created_idx",
        "domains",
        "resolved_address_id, created_at desc, id"
    ),
    index!("domains_labelhash_idx", "domains", "labelhash"),
    ReplayIndex {
        name: "domains_bracketed_labelhash_idx",
        create_sql: "create index if not exists domains_bracketed_labelhash_idx on domains(labelhash) where label_name like '[%'",
    },
    expression_index!("domains_name_md5_idx", "domains", "md5(name)"),
    expression_index!("domains_label_name_md5_idx", "domains", "md5(label_name)"),
    gin_expression_index!("domains_name_trgm_idx", "domains", "name gin_trgm_ops"),
    gin_expression_index!(
        "domains_name_lower_trgm_idx",
        "domains",
        "lower(name) gin_trgm_ops"
    ),
    gin_expression_index!(
        "domains_label_name_trgm_idx",
        "domains",
        "label_name gin_trgm_ops"
    ),
    gin_expression_index!(
        "domains_label_name_lower_trgm_idx",
        "domains",
        "lower(label_name) gin_trgm_ops"
    ),
    index!("registrations_domain_idx", "registrations", "domain_id"),
    index!(
        "registrations_registrant_idx",
        "registrations",
        "registrant_id"
    ),
    index!(
        "registrations_registration_date_idx",
        "registrations",
        "registration_date"
    ),
    index!(
        "registrations_expiry_date_idx",
        "registrations",
        "expiry_date"
    ),
    index!("wrapped_domains_owner_idx", "wrapped_domains", "owner_id"),
    index!("wrapped_domains_domain_idx", "wrapped_domains", "domain_id"),
    index!("resolvers_domain_idx", "resolvers", "domain_id"),
    index!("resolvers_address_idx", "resolvers", "address"),
    index!(
        "entity_changes_lookup_idx",
        "entity_changes",
        "entity_type, entity_id, block_number"
    ),
    index!(
        "account_snapshots_block_idx",
        "account_snapshots",
        "block_number"
    ),
    index!(
        "domain_snapshots_block_idx",
        "domain_snapshots",
        "block_number"
    ),
    index!(
        "registration_snapshots_block_idx",
        "registration_snapshots",
        "block_number"
    ),
    index!(
        "wrapped_domain_snapshots_block_idx",
        "wrapped_domain_snapshots",
        "block_number"
    ),
    index!(
        "resolver_snapshots_block_idx",
        "resolver_snapshots",
        "block_number"
    ),
    index!("transfer_events_domain_idx", "transfer_events", "domain_id"),
    index!(
        "transfer_events_domain_id_idx",
        "transfer_events",
        "domain_id, id"
    ),
    index!(
        "transfer_events_block_idx",
        "transfer_events",
        "block_number"
    ),
    index!(
        "new_owner_events_domain_idx",
        "new_owner_events",
        "domain_id"
    ),
    index!(
        "new_owner_events_domain_id_idx",
        "new_owner_events",
        "domain_id, id"
    ),
    index!(
        "new_owner_events_block_idx",
        "new_owner_events",
        "block_number"
    ),
    index!(
        "new_resolver_events_domain_idx",
        "new_resolver_events",
        "domain_id"
    ),
    index!(
        "new_resolver_events_domain_id_idx",
        "new_resolver_events",
        "domain_id, id"
    ),
    index!(
        "new_resolver_events_block_idx",
        "new_resolver_events",
        "block_number"
    ),
    index!("new_ttl_events_domain_idx", "new_ttl_events", "domain_id"),
    index!(
        "new_ttl_events_domain_id_idx",
        "new_ttl_events",
        "domain_id, id"
    ),
    index!("new_ttl_events_block_idx", "new_ttl_events", "block_number"),
    index!(
        "wrapped_transfer_events_domain_idx",
        "wrapped_transfer_events",
        "domain_id"
    ),
    index!(
        "wrapped_transfer_events_domain_id_idx",
        "wrapped_transfer_events",
        "domain_id, id"
    ),
    index!(
        "wrapped_transfer_events_block_idx",
        "wrapped_transfer_events",
        "block_number"
    ),
    index!(
        "name_wrapped_events_domain_idx",
        "name_wrapped_events",
        "domain_id"
    ),
    index!(
        "name_wrapped_events_domain_id_idx",
        "name_wrapped_events",
        "domain_id, id"
    ),
    index!(
        "name_wrapped_events_block_idx",
        "name_wrapped_events",
        "block_number"
    ),
    index!(
        "name_unwrapped_events_domain_idx",
        "name_unwrapped_events",
        "domain_id"
    ),
    index!(
        "name_unwrapped_events_domain_id_idx",
        "name_unwrapped_events",
        "domain_id, id"
    ),
    index!(
        "name_unwrapped_events_block_idx",
        "name_unwrapped_events",
        "block_number"
    ),
    index!(
        "fuses_set_events_domain_idx",
        "fuses_set_events",
        "domain_id"
    ),
    index!(
        "fuses_set_events_domain_id_idx",
        "fuses_set_events",
        "domain_id, id"
    ),
    index!(
        "fuses_set_events_block_idx",
        "fuses_set_events",
        "block_number"
    ),
    index!(
        "expiry_extended_events_domain_idx",
        "expiry_extended_events",
        "domain_id"
    ),
    index!(
        "expiry_extended_events_domain_id_idx",
        "expiry_extended_events",
        "domain_id, id"
    ),
    index!(
        "expiry_extended_events_block_idx",
        "expiry_extended_events",
        "block_number"
    ),
    index!(
        "name_registered_events_registration_idx",
        "name_registered_events",
        "registration_id"
    ),
    index!(
        "name_registered_events_registration_id_idx",
        "name_registered_events",
        "registration_id, id"
    ),
    index!(
        "name_registered_events_block_idx",
        "name_registered_events",
        "block_number"
    ),
    index!(
        "name_renewed_events_registration_idx",
        "name_renewed_events",
        "registration_id"
    ),
    index!(
        "name_renewed_events_registration_id_idx",
        "name_renewed_events",
        "registration_id, id"
    ),
    index!(
        "name_renewed_events_block_idx",
        "name_renewed_events",
        "block_number"
    ),
    index!(
        "name_transferred_events_registration_idx",
        "name_transferred_events",
        "registration_id"
    ),
    index!(
        "name_transferred_events_registration_id_idx",
        "name_transferred_events",
        "registration_id, id"
    ),
    index!(
        "name_transferred_events_block_idx",
        "name_transferred_events",
        "block_number"
    ),
    index!(
        "addr_changed_events_resolver_idx",
        "addr_changed_events",
        "resolver_id"
    ),
    index!(
        "addr_changed_events_resolver_id_idx",
        "addr_changed_events",
        "resolver_id, id"
    ),
    index!(
        "addr_changed_events_block_idx",
        "addr_changed_events",
        "block_number"
    ),
    index!(
        "multicoin_addr_changed_events_resolver_idx",
        "multicoin_addr_changed_events",
        "resolver_id"
    ),
    index!(
        "multicoin_addr_changed_events_resolver_id_idx",
        "multicoin_addr_changed_events",
        "resolver_id, id"
    ),
    index!(
        "multicoin_addr_changed_events_block_idx",
        "multicoin_addr_changed_events",
        "block_number"
    ),
    index!(
        "name_changed_events_resolver_idx",
        "name_changed_events",
        "resolver_id"
    ),
    index!(
        "name_changed_events_resolver_id_idx",
        "name_changed_events",
        "resolver_id, id"
    ),
    index!(
        "name_changed_events_block_idx",
        "name_changed_events",
        "block_number"
    ),
    index!(
        "abi_changed_events_resolver_idx",
        "abi_changed_events",
        "resolver_id"
    ),
    index!(
        "abi_changed_events_resolver_id_idx",
        "abi_changed_events",
        "resolver_id, id"
    ),
    index!(
        "abi_changed_events_block_idx",
        "abi_changed_events",
        "block_number"
    ),
    index!(
        "pubkey_changed_events_resolver_idx",
        "pubkey_changed_events",
        "resolver_id"
    ),
    index!(
        "pubkey_changed_events_resolver_id_idx",
        "pubkey_changed_events",
        "resolver_id, id"
    ),
    index!(
        "pubkey_changed_events_block_idx",
        "pubkey_changed_events",
        "block_number"
    ),
    index!(
        "text_changed_events_resolver_idx",
        "text_changed_events",
        "resolver_id"
    ),
    index!(
        "text_changed_events_resolver_id_idx",
        "text_changed_events",
        "resolver_id, id"
    ),
    index!(
        "text_changed_events_block_idx",
        "text_changed_events",
        "block_number"
    ),
    index!(
        "contenthash_changed_events_resolver_idx",
        "contenthash_changed_events",
        "resolver_id"
    ),
    index!(
        "contenthash_changed_events_resolver_id_idx",
        "contenthash_changed_events",
        "resolver_id, id"
    ),
    index!(
        "contenthash_changed_events_block_idx",
        "contenthash_changed_events",
        "block_number"
    ),
    index!(
        "interface_changed_events_resolver_idx",
        "interface_changed_events",
        "resolver_id"
    ),
    index!(
        "interface_changed_events_resolver_id_idx",
        "interface_changed_events",
        "resolver_id, id"
    ),
    index!(
        "interface_changed_events_block_idx",
        "interface_changed_events",
        "block_number"
    ),
    index!(
        "authorisation_changed_events_resolver_idx",
        "authorisation_changed_events",
        "resolver_id"
    ),
    index!(
        "authorisation_changed_events_resolver_id_idx",
        "authorisation_changed_events",
        "resolver_id, id"
    ),
    index!(
        "authorisation_changed_events_block_idx",
        "authorisation_changed_events",
        "block_number"
    ),
    index!(
        "version_changed_events_resolver_idx",
        "version_changed_events",
        "resolver_id"
    ),
    index!(
        "version_changed_events_resolver_id_idx",
        "version_changed_events",
        "resolver_id, id"
    ),
    index!(
        "version_changed_events_block_idx",
        "version_changed_events",
        "block_number"
    ),
];

#[cfg(test)]
mod tests {
    use super::BULK_REPLAY_INDEXES;

    fn index_sql(name: &str) -> &'static str {
        BULK_REPLAY_INDEXES
            .iter()
            .find(|index| index.name == name)
            .map(|index| index.create_sql)
            .unwrap_or_else(|| panic!("missing replay index {name}"))
    }

    #[test]
    fn replay_indexes_include_ensnode_domain_lookup_indexes() {
        assert!(index_sql("domains_name_trgm_idx").contains("using gin"));
        assert!(index_sql("domains_name_lower_trgm_idx").contains("lower(name) gin_trgm_ops"));
        assert!(index_sql("domains_label_name_lower_trgm_idx").contains("gin_trgm_ops"));
        index_sql("domains_registrant_idx");
        index_sql("domains_wrapped_owner_idx");
        index_sql("domains_resolved_address_idx");
    }

    #[test]
    fn replay_indexes_include_ensjs_address_sort_indexes() {
        for name in [
            "domains_owner_expiry_idx",
            "domains_registrant_expiry_idx",
            "domains_wrapped_owner_expiry_idx",
            "domains_resolved_address_expiry_idx",
            "domains_owner_created_idx",
            "domains_registrant_created_idx",
            "domains_wrapped_owner_created_idx",
            "domains_resolved_address_created_idx",
        ] {
            assert!(index_sql(name).contains(", id"));
        }
    }

    #[test]
    fn replay_indexes_include_derived_event_parent_sort_indexes() {
        for name in [
            "transfer_events_domain_id_idx",
            "new_owner_events_domain_id_idx",
            "name_registered_events_registration_id_idx",
            "addr_changed_events_resolver_id_idx",
            "text_changed_events_resolver_id_idx",
            "version_changed_events_resolver_id_idx",
        ] {
            assert!(index_sql(name).contains(", id"));
        }
    }
}
