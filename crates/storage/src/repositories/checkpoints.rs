use sqlx::PgPool;

use crate::{error::*, models::SourceCheckpointRow};

pub struct CheckpointsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl CheckpointsRepo<'_> {
    pub async fn list(&self) -> StorageResult<Vec<SourceCheckpointRow>> {
        Ok(sqlx::query_as::<_, SourceCheckpointRow>(
            r#"
            select source, block_number, block_hash
            from source_checkpoints
            order by source
            "#,
        )
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn upsert(
        &self,
        source: &str,
        block_number: i64,
        block_hash: &str,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into source_checkpoints (source, block_number, block_hash)
            values ($1, $2, $3)
            on conflict (source) do update
            set block_number = excluded.block_number,
                block_hash = excluded.block_hash,
                updated_at = now()
            "#,
        )
        .bind(source)
        .bind(block_number)
        .bind(block_hash)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_source(&self, source: &str) -> StorageResult<Option<SourceCheckpointRow>> {
        Ok(sqlx::query_as::<_, SourceCheckpointRow>(
            r#"
            select source, block_number, block_hash
            from source_checkpoints
            where source = $1
            "#,
        )
        .bind(source)
        .fetch_optional(self.pool)
        .await?)
    }
}
