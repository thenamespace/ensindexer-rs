use std::sync::{Arc, Mutex};

use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{entity_cache::*, error::*, filters::*, models::*, query::*};

pub struct AccountsRepo<'a> {
    pub(crate) pool: &'a PgPool,
    pub(crate) entity_cache: Arc<Mutex<Option<EntityCache>>>,
}

impl AccountsRepo<'_> {
    pub async fn create_if_missing(&self, id: &str) -> StorageResult<bool> {
        if self.is_cached(CachedEntityKind::Account, id)? {
            return Ok(false);
        }
        let inserted = sqlx::query_scalar::<_, String>(
            "insert into accounts (id) values ($1) on conflict (id) do nothing returning id",
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        self.cache(CachedEntityKind::Account, id)?;
        Ok(inserted.is_some())
    }

    fn is_cached(&self, kind: CachedEntityKind, id: &str) -> StorageResult<bool> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache
            .as_ref()
            .is_some_and(|active| active.contains(kind, id)))
    }

    fn cache(&self, kind: CachedEntityKind, id: &str) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.insert(kind, id.to_owned());
        }
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<AccountRow>> {
        Ok(
            sqlx::query_as::<_, AccountRow>("select id from accounts where id = $1")
                .bind(id)
                .fetch_optional(self.pool)
                .await?,
        )
    }

    pub async fn find_by_id_at_block(
        &self,
        id: &str,
        block_number: i32,
    ) -> StorageResult<Option<AccountRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        push_historical_entity_ctes(&mut query, block_number);
        query
            .push("select id from accounts where id = ")
            .push_bind(id);
        Ok(query.build_query_as().fetch_optional(self.pool).await?)
    }

    pub async fn list(
        &self,
        first: i64,
        skip: i64,
        filter: AccountFilter,
        order_by: AccountOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AccountRow>> {
        self.list_for_block(None, first, skip, filter, order_by, direction)
            .await
    }

    pub async fn list_at_block(
        &self,
        block_number: i32,
        first: i64,
        skip: i64,
        filter: AccountFilter,
        order_by: AccountOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AccountRow>> {
        self.list_for_block(Some(block_number), first, skip, filter, order_by, direction)
            .await
    }

    async fn list_for_block(
        &self,
        block_number: Option<i32>,
        first: i64,
        skip: i64,
        filter: AccountFilter,
        order_by: AccountOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AccountRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        if let Some(block_number) = block_number {
            push_historical_entity_ctes(&mut query, block_number);
        }
        query.push("select id from accounts");
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_account_filters(&mut separated, &mut has_where, filter);

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(account_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(" limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }
}
