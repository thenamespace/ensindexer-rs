mod replay_indexes;

use std::collections::HashSet;

use sqlx::{AssertSqlSafe, PgPool};

use crate::StorageResult;
use replay_indexes::{DEPRECATED_BULK_REPLAY_INDEXES, bulk_replay_indexes};

pub struct MaintenanceRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl MaintenanceRepo<'_> {
    pub async fn drop_bulk_replay_indexes(&self) -> StorageResult<()> {
        for index in bulk_replay_indexes() {
            sqlx::query(AssertSqlSafe(format!(
                "drop index if exists {}",
                index.name
            )))
            .execute(self.pool)
            .await?;
        }
        for index_name in DEPRECATED_BULK_REPLAY_INDEXES {
            sqlx::query(AssertSqlSafe(format!("drop index if exists {index_name}")))
                .execute(self.pool)
                .await?;
        }
        Ok(())
    }

    pub async fn recreate_bulk_replay_indexes(&self) -> StorageResult<()> {
        sqlx::query("create extension if not exists btree_gin")
            .execute(self.pool)
            .await?;

        for index in bulk_replay_indexes() {
            sqlx::query(index.create_sql).execute(self.pool).await?;
        }
        self.analyze_query_planner_tables().await?;
        Ok(())
    }

    pub async fn ensure_bulk_replay_indexes(&self) -> StorageResult<()> {
        let expected_indexes = bulk_replay_indexes()
            .map(|index| index.name)
            .collect::<Vec<_>>();
        let existing_indexes = sqlx::query_scalar::<_, String>(
            r#"
            select indexname
            from pg_indexes
            where schemaname = 'public'
              and indexname = any($1)
            "#,
        )
        .bind(&expected_indexes)
        .fetch_all(self.pool)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

        let missing_indexes = expected_indexes
            .iter()
            .copied()
            .filter(|name| !existing_indexes.contains(*name))
            .collect::<Vec<_>>();

        if missing_indexes.is_empty() {
            tracing::debug!("bulk replay secondary indexes are present");
            return Ok(());
        }

        tracing::warn!(
            missing_indexes = ?missing_indexes,
            "bulk replay secondary indexes are missing; recreating before serving"
        );
        self.recreate_bulk_replay_indexes().await
    }

    pub async fn analyze_query_planner_tables(&self) -> StorageResult<()> {
        for table in QUERY_PLANNER_TABLES {
            sqlx::query(AssertSqlSafe(format!("analyze {table}")))
                .execute(self.pool)
                .await?;
        }
        Ok(())
    }
}

const QUERY_PLANNER_TABLES: &[&str] = &[
    "domains",
    "registrations",
    "wrapped_domains",
    "resolvers",
    "accounts",
    "entity_changes",
    "transfer_events",
    "new_owner_events",
    "new_resolver_events",
    "new_ttl_events",
    "wrapped_transfer_events",
    "name_wrapped_events",
    "name_unwrapped_events",
    "fuses_set_events",
    "expiry_extended_events",
    "name_registered_events",
    "name_renewed_events",
    "name_transferred_events",
    "addr_changed_events",
    "multicoin_addr_changed_events",
    "name_changed_events",
    "abi_changed_events",
    "pubkey_changed_events",
    "text_changed_events",
    "contenthash_changed_events",
    "interface_changed_events",
    "authorisation_changed_events",
    "version_changed_events",
];
