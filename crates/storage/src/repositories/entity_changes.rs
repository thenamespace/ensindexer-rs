use sqlx::PgPool;

use crate::StorageResult;

pub struct EntityChangesRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl EntityChangesRepo<'_> {
    pub async fn record(
        &self,
        entity_type: &'static str,
        entity_id: &str,
        block_number: i32,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into entity_changes (entity_type, entity_id, block_number)
            values ($1, $2, $3)
            on conflict do nothing
            "#,
        )
        .bind(entity_type)
        .bind(entity_id)
        .bind(block_number)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}
