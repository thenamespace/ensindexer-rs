use sqlx::PgPool;

use crate::{error::*, inserts::*, models::BlockRow};

pub struct BlocksRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl BlocksRepo<'_> {
    pub async fn upsert(&self, block: BlockInsert) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into blocks (number, hash, parent_hash, timestamp)
            values ($1, $2, $3, $4)
            on conflict (number) do update
            set hash = excluded.hash,
                parent_hash = excluded.parent_hash,
                timestamp = excluded.timestamp
            "#,
        )
        .bind(block.number)
        .bind(block.hash)
        .bind(block.parent_hash)
        .bind(block.timestamp)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_number(&self, number: i64) -> StorageResult<Option<BlockRow>> {
        Ok(sqlx::query_as::<_, BlockRow>(
            r#"
            select number, hash, parent_hash, timestamp
            from blocks
            where number = $1
            "#,
        )
        .bind(number)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn find_by_hash(&self, hash: &str) -> StorageResult<Option<BlockRow>> {
        Ok(sqlx::query_as::<_, BlockRow>(
            r#"
            select number, hash, parent_hash, timestamp
            from blocks
            where hash = $1
            "#,
        )
        .bind(hash)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn find_latest(&self) -> StorageResult<Option<BlockRow>> {
        Ok(sqlx::query_as::<_, BlockRow>(
            r#"
            select number, hash, parent_hash, timestamp
            from blocks
            order by number desc
            limit 1
            "#,
        )
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn find_latest_at_or_after(&self, number: i64) -> StorageResult<Option<BlockRow>> {
        Ok(sqlx::query_as::<_, BlockRow>(
            r#"
            select number, hash, parent_hash, timestamp
            from blocks
            where number >= $1
            order by number asc
            limit 1
            "#,
        )
        .bind(number)
        .fetch_optional(self.pool)
        .await?)
    }
}
