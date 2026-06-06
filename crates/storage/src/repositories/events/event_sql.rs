pub(super) fn domain_event_ref_union_sql() -> &'static str {
    r#"
    select 'Transfer' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           null::text as name, null::integer as fuses, owner_id, null::numeric as expiry_date
    from transfer_events
    union all select 'NewOwner' as kind, id, block_number, transaction_id, domain_id as parent_id,
           parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           null::text as name, null::integer as fuses, owner_id, null::numeric as expiry_date
    from new_owner_events
    union all select 'NewResolver' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, resolver_id, null::numeric as ttl,
           null::text as name, null::integer as fuses, null::text as owner_id, null::numeric as expiry_date
    from new_resolver_events
    union all select 'NewTTL' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, ttl,
           null::text as name, null::integer as fuses, null::text as owner_id, null::numeric as expiry_date
    from new_ttl_events
    union all select 'WrappedTransfer' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           null::text as name, null::integer as fuses, owner_id, null::numeric as expiry_date
    from wrapped_transfer_events
    union all select 'NameWrapped' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           name, fuses, owner_id, expiry_date
    from name_wrapped_events
    union all select 'NameUnwrapped' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           null::text as name, null::integer as fuses, owner_id, null::numeric as expiry_date
    from name_unwrapped_events
    union all select 'FusesSet' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           null::text as name, fuses, null::text as owner_id, null::numeric as expiry_date
    from fuses_set_events
    union all select 'ExpiryExtended' as kind, id, block_number, transaction_id, domain_id as parent_id,
           null::text as parent_domain_id, null::text as resolver_id, null::numeric as ttl,
           null::text as name, null::integer as fuses, null::text as owner_id, expiry_date
    from expiry_extended_events
    "#
}

pub(super) fn registration_event_ref_union_sql() -> &'static str {
    r#"
    select 'NameRegistered' as kind, id, block_number, transaction_id, registration_id as parent_id,
           registrant_id, null::text as new_owner_id, expiry_date
    from name_registered_events
    union all select 'NameRenewed' as kind, id, block_number, transaction_id, registration_id as parent_id,
           null::text as registrant_id, null::text as new_owner_id, expiry_date
    from name_renewed_events
    union all select 'NameTransferred' as kind, id, block_number, transaction_id, registration_id as parent_id,
           null::text as registrant_id, new_owner_id, null::numeric as expiry_date
    from name_transferred_events
    "#
}

pub(super) fn resolver_event_ref_union_sql() -> &'static str {
    r#"
    select 'AddrChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from addr_changed_events
    union all select 'MulticoinAddrChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, coin_type, addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from multicoin_addr_changed_events
    union all select 'NameChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from name_changed_events
    union all select 'AbiChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from abi_changed_events
    union all select 'PubkeyChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, x, y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from pubkey_changed_events
    union all select 'TextChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, key,
           value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from text_changed_events
    union all select 'ContenthashChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from contenthash_changed_events
    union all select 'InterfaceChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, interface_id,
           implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, null::numeric as version
    from interface_changed_events
    union all select 'AuthorisationChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, owner, target, is_authorized, null::numeric as version
    from authorisation_changed_events
    union all select 'VersionChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id,
           null::text as addr_id, null::numeric as coin_type, null::text as addr, null::text as name,
           null::numeric as content_type, null::text as x, null::text as y, null::text as key,
           null::text as value, null::text as hash, null::text as interface_id,
           null::text as implementer, null::text as owner, null::text as target,
           null::boolean as is_authorized, version
    from version_changed_events
    "#
}
