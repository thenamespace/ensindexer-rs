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

pub(super) const CURRENT_STATE_INDEXES: &[ReplayIndex] = &[
    index!("domains_parent_idx", "domains", "parent_id"),
    ReplayIndex {
        name: "domains_parent_label_name_sort_idx",
        create_sql: "create index if not exists domains_parent_label_name_sort_idx on domains(parent_id, left(label_name, 256), id)",
    },
    ReplayIndex {
        name: "domains_parent_name_sort_idx",
        create_sql: "create index if not exists domains_parent_name_sort_idx on domains(parent_id, left(name, 256), id)",
    },
    index!(
        "domains_parent_created_idx",
        "domains",
        "parent_id, created_at desc, id"
    ),
    index!(
        "domains_parent_expiry_idx",
        "domains",
        "parent_id, expiry_date desc, id"
    ),
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
    index!("domains_name_md5_id_idx", "domains", "md5(name), id"),
    index!(
        "domains_label_name_md5_id_idx",
        "domains",
        "md5(label_name), id"
    ),
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
    ReplayIndex {
        name: "domains_parent_name_trgm_idx",
        create_sql: "create index if not exists domains_parent_name_trgm_idx on domains using gin (parent_id, name gin_trgm_ops)",
    },
    ReplayIndex {
        name: "domains_parent_name_lower_trgm_idx",
        create_sql: "create index if not exists domains_parent_name_lower_trgm_idx on domains using gin (parent_id, lower(name) gin_trgm_ops)",
    },
    ReplayIndex {
        name: "domains_parent_label_name_trgm_idx",
        create_sql: "create index if not exists domains_parent_label_name_trgm_idx on domains using gin (parent_id, label_name gin_trgm_ops)",
    },
    ReplayIndex {
        name: "domains_parent_label_name_lower_trgm_idx",
        create_sql: "create index if not exists domains_parent_label_name_lower_trgm_idx on domains using gin (parent_id, lower(label_name) gin_trgm_ops)",
    },
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
    index!(
        "registrations_label_name_md5_expiry_idx",
        "registrations",
        "md5(label_name), expiry_date desc, id"
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
];
