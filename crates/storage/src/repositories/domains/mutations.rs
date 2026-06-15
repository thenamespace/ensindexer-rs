use bigdecimal::BigDecimal;

use super::DomainsRepo;
use crate::{error::*, inserts::DomainUpsert, models::DomainRow};

impl DomainsRepo<'_> {
    pub async fn create_if_missing(&self, input: DomainUpsert) -> StorageResult<bool> {
        if self.cache_active()? {
            match self.cached_domain(&input.id)? {
                Some(Some(_)) => return Ok(false),
                Some(None) => {}
                None => {
                    if let Some(row) = self.find_by_id_uncached(&input.id).await? {
                        self.remember_domain(&input.id, Some(row))?;
                        return Ok(false);
                    }
                }
            }
            let row = DomainRow {
                id: input.id.clone(),
                name: None,
                label_name: None,
                labelhash: None,
                parent_id: None,
                subdomain_count: 0,
                resolved_address_id: None,
                resolver_id: None,
                ttl: None,
                is_migrated: input.is_migrated,
                created_at: input.created_at,
                owner_id: input.owner_id,
                registrant_id: None,
                wrapped_owner_id: None,
                expiry_date: None,
            };
            self.put_cached_domain(row)?;
            return Ok(true);
        }
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

        Ok(inserted.is_some())
    }

    pub async fn set_owner(&self, id: &str, owner_id: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.owner_id = owner_id.to_owned();
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.parent_id = Some(parent_id.to_owned());
            row.labelhash = Some(labelhash.to_owned());
            row.is_migrated = is_migrated;
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            if row.label_name.is_none() {
                row.label_name = label_name.map(str::to_owned);
            }
            if row.name.is_none() {
                row.name = name.map(str::to_owned);
            }
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.label_name = label_name.map(str::to_owned);
            row.name = name.map(str::to_owned);
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.subdomain_count += 1;
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.resolver_id = resolver_id.map(str::to_owned);
            row.resolved_address_id = resolved_address_id.map(str::to_owned);
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.ttl = Some(ttl);
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.registrant_id = Some(registrant_id.to_owned());
            row.expiry_date = Some(expiry_date);
            return self.put_cached_domain(row);
        }
        sqlx::query("update domains set registrant_id = $2, expiry_date = $3 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant(&self, id: &str, registrant_id: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.registrant_id = Some(registrant_id.to_owned());
            return self.put_cached_domain(row);
        }
        sqlx::query("update domains set registrant_id = $2 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_wrapped_owner(&self, id: &str, wrapped_owner_id: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.wrapped_owner_id = Some(wrapped_owner_id.to_owned());
            return self.put_cached_domain(row);
        }
        sqlx::query("update domains set wrapped_owner_id = $2 where id = $1")
            .bind(id)
            .bind(wrapped_owner_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn clear_wrapped_owner(&self, id: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.wrapped_owner_id = None;
            return self.put_cached_domain(row);
        }
        sqlx::query("update domains set wrapped_owner_id = null where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn clear_expiry(&self, id: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            row.expiry_date = None;
            return self.put_cached_domain(row);
        }
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
        if let Some(mut row) = self.find_by_id(id).await?
            && self.cache_active()?
        {
            if row
                .expiry_date
                .as_ref()
                .is_none_or(|current| current < &expiry_date)
            {
                row.expiry_date = Some(expiry_date);
                return self.put_cached_domain(row);
            }
            return Ok(());
        }
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
