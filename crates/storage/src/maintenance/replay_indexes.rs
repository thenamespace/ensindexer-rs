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

pub(super) const BULK_REPLAY_INDEXES: &[ReplayIndex] = &[
    index!("domains_parent_idx", "domains", "parent_id"),
    index!("domains_owner_idx", "domains", "owner_id"),
    index!("domains_resolver_idx", "domains", "resolver_id"),
    index!("domains_labelhash_idx", "domains", "labelhash"),
    expression_index!("domains_name_md5_idx", "domains", "md5(name)"),
    expression_index!("domains_label_name_md5_idx", "domains", "md5(label_name)"),
    index!("registrations_domain_idx", "registrations", "domain_id"),
    index!(
        "registrations_registrant_idx",
        "registrations",
        "registrant_id"
    ),
    index!("wrapped_domains_owner_idx", "wrapped_domains", "owner_id"),
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
        "new_resolver_events_block_idx",
        "new_resolver_events",
        "block_number"
    ),
    index!("new_ttl_events_domain_idx", "new_ttl_events", "domain_id"),
    index!("new_ttl_events_block_idx", "new_ttl_events", "block_number"),
    index!(
        "wrapped_transfer_events_domain_idx",
        "wrapped_transfer_events",
        "domain_id"
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
        "version_changed_events_block_idx",
        "version_changed_events",
        "block_number"
    ),
];
