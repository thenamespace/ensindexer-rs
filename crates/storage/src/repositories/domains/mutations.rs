use bigdecimal::BigDecimal;

use super::DomainsRepo;
use crate::{entity_cache::CachedEntityKind, error::*, inserts::DomainUpsert};

impl DomainsRepo<'_> {
    pub async fn create_if_missing(&self, input: DomainUpsert) -> StorageResult<bool> {
        if self.is_cached(CachedEntityKind::Domain, &input.id)? {
            return Ok(false);
        }
        let id = input.id.clone();
        let inserted = sqlx::query_scalar::<_, String>(
            r#"
        insert into domains (id, created_at, owner_id, is_migrated)
        values ($1, $2, $3, $4)
        on conflict (id) do nothing
        returning id
        "#,
        )
        .bind(input.id)
        .bind(input.created_at)
        .bind(input.owner_id)
        .bind(input.is_migrated)
        .fetch_optional(self.pool)
        .await?;

        self.cache(CachedEntityKind::Domain, &id)?;
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

    pub async fn set_owner(&self, id: &str, owner_id: &str) -> StorageResult<()> {
        sqlx::query("update domains set owner_id = $2 where id = $1")
            .bind(id)
            .bind(owner_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_parent_and_label(
        &self,
        id: &str,
        parent_id: &str,
        labelhash: &str,
        is_migrated: bool,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
        update domains
        set parent_id = $2,
            labelhash = $3,
            is_migrated = $4
        where id = $1
        "#,
        )
        .bind(id)
        .bind(parent_id)
        .bind(labelhash)
        .bind(is_migrated)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_name_if_unknown(
        &self,
        id: &str,
        label_name: Option<&str>,
        name: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
        update domains
        set label_name = coalesce(label_name, $2),
            name = coalesce(name, $3)
        where id = $1
        "#,
        )
        .bind(id)
        .bind(label_name)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_name(
        &self,
        id: &str,
        label_name: Option<&str>,
        name: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
        update domains
        set label_name = $2,
            name = $3
        where id = $1
        "#,
        )
        .bind(id)
        .bind(label_name)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn increment_subdomain_count(&self, id: &str) -> StorageResult<()> {
        sqlx::query("update domains set subdomain_count = subdomain_count + 1 where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_resolver(
        &self,
        id: &str,
        resolver_id: Option<&str>,
        resolved_address_id: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
        update domains
        set resolver_id = $2,
            resolved_address_id = $3
        where id = $1
        "#,
        )
        .bind(id)
        .bind(resolver_id)
        .bind(resolved_address_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_ttl(&self, id: &str, ttl: BigDecimal) -> StorageResult<()> {
        sqlx::query("update domains set ttl = $2 where id = $1")
            .bind(id)
            .bind(ttl)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant_and_expiry(
        &self,
        id: &str,
        registrant_id: &str,
        expiry_date: BigDecimal,
    ) -> StorageResult<()> {
        sqlx::query("update domains set registrant_id = $2, expiry_date = $3 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant(&self, id: &str, registrant_id: &str) -> StorageResult<()> {
        sqlx::query("update domains set registrant_id = $2 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_wrapped_owner(&self, id: &str, wrapped_owner_id: &str) -> StorageResult<()> {
        sqlx::query("update domains set wrapped_owner_id = $2 where id = $1")
            .bind(id)
            .bind(wrapped_owner_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn clear_wrapped_owner(&self, id: &str) -> StorageResult<()> {
        sqlx::query("update domains set wrapped_owner_id = null where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn clear_expiry(&self, id: &str) -> StorageResult<()> {
        sqlx::query("update domains set expiry_date = null where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_expiry_if_newer(
        &self,
        id: &str,
        expiry_date: BigDecimal,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
        update domains
        set expiry_date = $2
        where id = $1 and (expiry_date is null or expiry_date < $2)
        "#,
        )
        .bind(id)
        .bind(expiry_date)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
