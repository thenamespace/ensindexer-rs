use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    AccountsRepo, BlocksRepo, CheckpointsRepo, DomainsRepo, EventsRepo, MaintenanceRepo,
    RegistrationsRepo, ResolversRepo, StorageResult, WrappedDomainsRepo,
};

#[derive(Clone)]
pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub async fn connect(database_url: &str) -> StorageResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
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
        EventsRepo { pool: &self.pool }
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
