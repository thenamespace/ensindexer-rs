use std::sync::{Arc, Mutex};

mod derived_filters;
mod fast_address;
mod filter_fields;
mod mutations;
mod queries;

use sqlx::PgPool;

use crate::{entity_cache::EntityCache, error::*, models::DomainRow, query::*};

pub struct DomainsRepo<'a> {
    pub(crate) pool: &'a PgPool,
    pub(crate) entity_cache: Arc<Mutex<Option<EntityCache>>>,
}

impl DomainsRepo<'_> {
    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<DomainRow>> {
        if let Some(row) = self.cached_domain(id)? {
            return Ok(row);
        }
        let row = self.find_by_id_uncached(id).await?;
        self.remember_domain(id, row.clone())?;
        Ok(row)
    }

    pub(crate) async fn find_by_id_uncached(&self, id: &str) -> StorageResult<Option<DomainRow>> {
        Ok(sqlx::query_as::<_, DomainRow>(
            r#"
            select id, name, label_name, labelhash, parent_id, subdomain_count,
                   resolved_address_id, resolver_id, ttl, is_migrated, created_at,
                   owner_id, registrant_id, wrapped_owner_id, expiry_date
            from domains
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub(crate) fn cache_active(&self) -> StorageResult<bool> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache.is_some())
    }

    pub(crate) fn cached_domain(&self, id: &str) -> StorageResult<Option<Option<DomainRow>>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache.as_ref().and_then(|active| active.get_domain(id)))
    }

    pub(crate) fn remember_domain(&self, id: &str, row: Option<DomainRow>) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.domains.insert(id.to_owned(), row);
        }
        Ok(())
    }

    pub(crate) fn put_cached_domain(&self, row: DomainRow) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.put_domain(row);
        }
        Ok(())
    }

    pub async fn find_by_id_at_block(
        &self,
        id: &str,
        block_number: i32,
    ) -> StorageResult<Option<DomainRow>> {
        let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new("");
        push_historical_entity_ctes(&mut query, block_number);
        query
            .push(domain_select_sql())
            .push(" where id = ")
            .push_bind(id);
        Ok(query.build_query_as().fetch_optional(self.pool).await?)
    }
}
