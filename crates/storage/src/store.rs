use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    AccountsRepo, BlocksRepo, CheckpointsRepo, DomainsRepo, EntityChangesRepo, EventsRepo,
    MaintenanceRepo, RegistrationsRepo, ResolversRepo, SnapshotsRepo, StorageResult,
    WrappedDomainsRepo, change_buffer::EntityChange, event_buffer::EventBuffer,
};

#[derive(Clone)]
pub struct Storage {
    pool: PgPool,
    pub(crate) change_buffer: Arc<Mutex<Option<BTreeSet<EntityChange>>>>,
    pub(crate) event_buffer: Arc<Mutex<Option<EventBuffer>>>,
}

impl Storage {
    pub async fn connect(database_url: &str) -> StorageResult<Self> {
        Self::connect_with_max_connections(database_url, 10).await
    }

    pub async fn connect_with_max_connections(
        database_url: &str,
        max_connections: u32,
    ) -> StorageResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;

        Ok(Self::from_pool(pool))
    }

    pub fn from_pool(pool: PgPool) -> Self {
        Self {
            pool,
            change_buffer: Arc::new(Mutex::new(None)),
            event_buffer: Arc::new(Mutex::new(None)),
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> StorageResult<()> {
        sqlx::migrate!("../../migrations").run(&self.pool).await?;
        Ok(())
    }

    pub fn accounts(&self) -> AccountsRepo<'_> {
        AccountsRepo { pool: &self.pool }
    }

    pub fn domains(&self) -> DomainsRepo<'_> {
        DomainsRepo { pool: &self.pool }
    }

    pub fn registrations(&self) -> RegistrationsRepo<'_> {
        RegistrationsRepo { pool: &self.pool }
    }

    pub fn resolvers(&self) -> ResolversRepo<'_> {
        ResolversRepo { pool: &self.pool }
    }

    pub fn wrapped_domains(&self) -> WrappedDomainsRepo<'_> {
        WrappedDomainsRepo { pool: &self.pool }
    }

    pub fn events(&self) -> EventsRepo<'_> {
        EventsRepo {
            pool: &self.pool,
            event_buffer: Arc::clone(&self.event_buffer),
        }
    }

    pub fn entity_changes(&self) -> EntityChangesRepo<'_> {
        EntityChangesRepo { pool: &self.pool }
    }

    pub fn snapshots(&self) -> SnapshotsRepo<'_> {
        SnapshotsRepo { pool: &self.pool }
    }

    pub fn blocks(&self) -> BlocksRepo<'_> {
        BlocksRepo { pool: &self.pool }
    }

    pub fn checkpoints(&self) -> CheckpointsRepo<'_> {
        CheckpointsRepo { pool: &self.pool }
    }

    pub fn maintenance(&self) -> MaintenanceRepo<'_> {
        MaintenanceRepo { pool: &self.pool }
    }
}
