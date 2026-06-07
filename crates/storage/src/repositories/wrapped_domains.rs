#![allow(clippy::collapsible_if)]

use std::sync::{Arc, Mutex};

use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub(crate) use self::composition::{
    push_wrapped_domain_subquery_filters, wrapped_domain_filter_has_conditions,
};
use self::filtering::push_wrapped_domain_filters;
use crate::{entity_cache::EntityCache, error::*, filters::*, models::*, query::*};

mod composition;
mod filtering;

pub struct WrappedDomainsRepo<'a> {
    pub(crate) pool: &'a PgPool,
    pub(crate) entity_cache: Arc<Mutex<Option<EntityCache>>>,
}

impl WrappedDomainsRepo<'_> {
    pub async fn upsert_full(
        &self,
        id: &str,
        domain_id: &str,
        expiry_date: BigDecimal,
        fuses: i32,
        owner_id: &str,
        name: Option<&str>,
    ) -> StorageResult<()> {
        if self.cache_active()? {
            self.put_cached_wrapped_domain(WrappedDomainRow {
                id: id.to_owned(),
                domain_id: domain_id.to_owned(),
                expiry_date,
                fuses,
                owner_id: owner_id.to_owned(),
                name: name.map(str::to_owned),
            })?;
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into wrapped_domains (id, domain_id, expiry_date, fuses, owner_id, name)
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do update
            set domain_id = excluded.domain_id,
                expiry_date = excluded.expiry_date,
                fuses = excluded.fuses,
                owner_id = excluded.owner_id,
                name = excluded.name
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(expiry_date)
        .bind(fuses)
        .bind(owner_id)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_transfer_placeholder(
        &self,
        id: &str,
        domain_id: &str,
        owner_id: &str,
    ) -> StorageResult<()> {
        if self.cache_active()? {
            let mut row = self
                .find_by_id(id)
                .await?
                .unwrap_or_else(|| WrappedDomainRow {
                    id: id.to_owned(),
                    domain_id: domain_id.to_owned(),
                    expiry_date: BigDecimal::from(0),
                    fuses: 0,
                    owner_id: owner_id.to_owned(),
                    name: None,
                });
            row.owner_id = owner_id.to_owned();
            self.put_cached_wrapped_domain(row)?;
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into wrapped_domains (id, domain_id, expiry_date, fuses, owner_id)
            values ($1, $2, 0, 0, $3)
            on conflict (id) do update
            set owner_id = excluded.owner_id
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<WrappedDomainRow>> {
        if let Some(row) = self.cached_wrapped_domain(id)? {
            return Ok(row);
        }
        let row = self.find_by_id_uncached(id).await?;
        self.remember_wrapped_domain(id, row.clone())?;
        Ok(row)
    }

    async fn find_by_id_uncached(&self, id: &str) -> StorageResult<Option<WrappedDomainRow>> {
        Ok(sqlx::query_as::<_, WrappedDomainRow>(
            r#"
            select id, domain_id, expiry_date, fuses, owner_id, name
            from wrapped_domains
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?)
    }

    fn cache_active(&self) -> StorageResult<bool> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache.is_some())
    }

    fn cached_wrapped_domain(&self, id: &str) -> StorageResult<Option<Option<WrappedDomainRow>>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache
            .as_ref()
            .and_then(|active| active.get_wrapped_domain(id)))
    }

    fn remember_wrapped_domain(
        &self,
        id: &str,
        row: Option<WrappedDomainRow>,
    ) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.wrapped_domains.insert(id.to_owned(), row);
        }
        Ok(())
    }

    fn put_cached_wrapped_domain(&self, row: WrappedDomainRow) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.put_wrapped_domain(row);
        }
        Ok(())
    }

    pub async fn find_by_id_at_block(
        &self,
        id: &str,
        block_number: i32,
    ) -> StorageResult<Option<WrappedDomainRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        push_historical_entity_ctes(&mut query, block_number);
        query
            .push("select id, domain_id, expiry_date, fuses, owner_id, name from wrapped_domains")
            .push(" where id = ")
            .push_bind(id);
        Ok(query.build_query_as().fetch_optional(self.pool).await?)
    }

    pub async fn find_by_domain_id(
        &self,
        domain_id: &str,
    ) -> StorageResult<Option<WrappedDomainRow>> {
        Ok(sqlx::query_as::<_, WrappedDomainRow>(
            r#"
            select id, domain_id, expiry_date, fuses, owner_id, name
            from wrapped_domains
            where domain_id = $1
            "#,
        )
        .bind(domain_id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn list(
        &self,
        first: i64,
        skip: i64,
        filter: WrappedDomainFilter,
        order_by: WrappedDomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<WrappedDomainRow>> {
        self.list_for_block(None, first, skip, filter, order_by, direction)
            .await
    }

    pub async fn list_at_block(
        &self,
        block_number: i32,
        first: i64,
        skip: i64,
        filter: WrappedDomainFilter,
        order_by: WrappedDomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<WrappedDomainRow>> {
        self.list_for_block(Some(block_number), first, skip, filter, order_by, direction)
            .await
    }

    async fn list_for_block(
        &self,
        block_number: Option<i32>,
        first: i64,
        skip: i64,
        filter: WrappedDomainFilter,
        order_by: WrappedDomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<WrappedDomainRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        if let Some(block_number) = block_number {
            push_historical_entity_ctes(&mut query, block_number);
        }
        query.push("select id, domain_id, expiry_date, fuses, owner_id, name from wrapped_domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_wrapped_domain_filters(&mut separated, &mut has_where, filter);

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(wrapped_domain_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id asc limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub async fn set_fuses(&self, id: &str, fuses: i32) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await? {
            if self.cache_active()? {
                row.fuses = fuses;
                return self.put_cached_wrapped_domain(row);
            }
        }
        sqlx::query("update wrapped_domains set fuses = $2 where id = $1")
            .bind(id)
            .bind(fuses)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_expiry(&self, id: &str, expiry_date: BigDecimal) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await? {
            if self.cache_active()? {
                row.expiry_date = expiry_date;
                return self.put_cached_wrapped_domain(row);
            }
        }
        sqlx::query("update wrapped_domains set expiry_date = $2 where id = $1")
            .bind(id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> StorageResult<()> {
        if self.cache_active()? {
            let mut cache = self
                .entity_cache
                .lock()
                .map_err(|_| StorageError::EntityCachePoisoned)?;
            if let Some(active) = cache.as_mut() {
                active.delete_wrapped_domain(id);
            }
            return Ok(());
        }
        sqlx::query("delete from wrapped_domains where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
