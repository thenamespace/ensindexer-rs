use super::ReplayIndex;

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

pub(super) const EVENT_INDEXES: &[ReplayIndex] = &[
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
