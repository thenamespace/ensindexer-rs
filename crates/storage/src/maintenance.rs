use sqlx::PgPool;

use crate::StorageResult;

pub struct MaintenanceRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl MaintenanceRepo<'_> {
    pub async fn reset_indexed_data(&self) -> StorageResult<()> {
        sqlx::query(
            r#"
            truncate table
              source_checkpoints,
              blocks,
              transfer_events,
              new_owner_events,
              new_resolver_events,
              new_ttl_events,
              wrapped_transfer_events,
              name_wrapped_events,
              name_unwrapped_events,
              fuses_set_events,
              expiry_extended_events,
              name_registered_events,
              name_renewed_events,
              name_transferred_events,
              addr_changed_events,
              multicoin_addr_changed_events,
              name_changed_events,
              abi_changed_events,
              pubkey_changed_events,
              text_changed_events,
              contenthash_changed_events,
              interface_changed_events,
              authorisation_changed_events,
              version_changed_events,
              account_snapshots,
              domain_snapshots,
              registration_snapshots,
              wrapped_domain_snapshots,
              resolver_snapshots,
              entity_changes,
              wrapped_domains,
              registrations,
              resolvers,
              domains,
              accounts
            restart identity cascade
            "#,
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
