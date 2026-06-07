use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{error::*, inserts::*, models::BlockRow};

const BLOCK_UPSERT_CHUNK_ROWS: usize = 10_000;

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

    pub async fn upsert_many(&self, blocks: Vec<BlockInsert>) -> StorageResult<()> {
        for chunk in blocks.chunks(BLOCK_UPSERT_CHUNK_ROWS) {
            let mut query = QueryBuilder::<Postgres>::new(
                "insert into blocks (number, hash, parent_hash, timestamp) ",
            );
            query.push_values(chunk, |mut row, block| {
                row.push_bind(block.number)
                    .push_bind(&block.hash)
                    .push_bind(&block.parent_hash)
                    .push_bind(block.timestamp);
            });
            query.push(
                r#"
                on conflict (number) do update
                set hash = excluded.hash,
                    parent_hash = excluded.parent_hash,
                    timestamp = excluded.timestamp
                "#,
            );
            query.build().execute(self.pool).await?;
        }
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
