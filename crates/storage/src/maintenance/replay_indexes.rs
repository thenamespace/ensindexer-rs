mod current;
mod events;

pub(super) struct ReplayIndex {
    pub(super) name: &'static str,
    pub(super) create_sql: &'static str,
}

pub(super) const DEPRECATED_BULK_REPLAY_INDEXES: &[&str] =
    &["domains_name_lower_idx", "domains_label_name_lower_idx"];

pub(super) fn bulk_replay_indexes() -> impl Iterator<Item = &'static ReplayIndex> {
    current::CURRENT_STATE_INDEXES
        .iter()
        .chain(events::EVENT_INDEXES.iter())
}

#[cfg(test)]
mod tests {
    use super::bulk_replay_indexes;

    fn index_sql(name: &str) -> &'static str {
        bulk_replay_indexes()
            .find(|index| index.name == name)
            .map(|index| index.create_sql)
            .unwrap_or_else(|| panic!("missing replay index {name}"))
    }

    #[test]
    fn replay_indexes_include_ensnode_domain_lookup_indexes() {
        assert!(index_sql("domains_name_trgm_idx").contains("using gin"));
        assert!(index_sql("domains_name_md5_id_idx").contains("md5(name), id"));
        assert!(index_sql("domains_label_name_md5_id_idx").contains("md5(label_name), id"));
        assert!(index_sql("domains_name_lower_trgm_idx").contains("lower(name) gin_trgm_ops"));
        assert!(index_sql("domains_label_name_lower_trgm_idx").contains("gin_trgm_ops"));
        assert!(
            index_sql("domains_parent_name_lower_trgm_idx")
                .contains("parent_id, lower(name) gin_trgm_ops")
        );
        assert!(
            index_sql("domains_parent_label_name_lower_trgm_idx")
                .contains("parent_id, lower(label_name) gin_trgm_ops")
        );
        assert!(index_sql("domains_parent_label_name_sort_idx").contains("left(label_name, 256)"));
        assert!(index_sql("domains_parent_name_sort_idx").contains("left(name, 256)"));
        index_sql("domains_registrant_idx");
        index_sql("domains_wrapped_owner_idx");
        index_sql("domains_resolved_address_idx");
        assert!(
            index_sql("registrations_label_name_md5_expiry_idx")
                .contains("md5(label_name), expiry_date desc, id")
        );
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
