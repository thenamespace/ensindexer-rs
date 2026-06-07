use sqlx::{Postgres, QueryBuilder};

use crate::{
    DomainRow, EntityKind, RegistrationRow, ResolverRow, Storage, StorageError, StorageResult,
    WrappedDomainRow,
};

use super::CHANGE_FLUSH_CHUNK_ROWS;

mod db;

impl Storage {
    pub(crate) fn entity_cache_is_active(&self) -> StorageResult<bool> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache.is_some())
    }

    pub(crate) async fn write_cached_snapshots_batch(
        &self,
        kind: EntityKind,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        match kind {
            EntityKind::Account => {
                let rows = self.cached_account_snapshot_rows(ids)?;
                write_account_snapshots(self, block_number, &rows).await
            }
            EntityKind::Domain => {
                let rows = self.cached_domain_snapshot_rows(ids)?;
                write_domain_snapshots(self, block_number, &rows).await
            }
            EntityKind::Registration => {
                let rows = self.cached_registration_snapshot_rows(ids)?;
                write_registration_snapshots(self, block_number, &rows).await
            }
            EntityKind::Resolver => {
                let rows = self.cached_resolver_snapshot_rows(ids)?;
                write_resolver_snapshots(self, block_number, &rows).await
            }
            EntityKind::WrappedDomain => {
                let rows = self.cached_wrapped_domain_snapshot_rows(ids)?;
                write_wrapped_domain_snapshots(self, block_number, &rows).await
            }
        }
    }

    pub(crate) async fn write_snapshots_batch(
        &self,
        kind: EntityKind,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        match kind {
            EntityKind::Account => self.write_account_snapshots_batch(block_number, ids).await,
            EntityKind::Domain => self.write_domain_snapshots_batch(block_number, ids).await,
            EntityKind::Registration => {
                self.write_registration_snapshots_batch(block_number, ids)
                    .await
            }
            EntityKind::Resolver => self.write_resolver_snapshots_batch(block_number, ids).await,
            EntityKind::WrappedDomain => {
                self.write_wrapped_domain_snapshots_batch(block_number, ids)
                    .await
            }
        }
    }

    fn cached_account_snapshot_rows(&self, ids: &[String]) -> StorageResult<Vec<String>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = cache.as_ref() else {
            return Ok(Vec::new());
        };
        Ok(ids
            .iter()
            .filter(|id| cache.accounts.contains_key(id.as_str()))
            .cloned()
            .collect())
    }

    fn cached_domain_snapshot_rows(&self, ids: &[String]) -> StorageResult<Vec<DomainRow>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = cache.as_ref() else {
            return Ok(Vec::new());
        };
        Ok(ids
            .iter()
            .filter_map(|id| cache.domains.get(id).and_then(Clone::clone))
            .collect())
    }

    fn cached_registration_snapshot_rows(
        &self,
        ids: &[String],
    ) -> StorageResult<Vec<RegistrationRow>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = cache.as_ref() else {
            return Ok(Vec::new());
        };
        Ok(ids
            .iter()
            .filter_map(|id| cache.registrations.get(id).and_then(Clone::clone))
            .collect())
    }

    fn cached_resolver_snapshot_rows(&self, ids: &[String]) -> StorageResult<Vec<ResolverRow>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = cache.as_ref() else {
            return Ok(Vec::new());
        };
        Ok(ids
            .iter()
            .filter_map(|id| cache.resolvers.get(id).and_then(Clone::clone))
            .collect())
    }

    fn cached_wrapped_domain_snapshot_rows(
        &self,
        ids: &[String],
    ) -> StorageResult<Vec<(String, Option<WrappedDomainRow>)>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = cache.as_ref() else {
            return Ok(Vec::new());
        };
        Ok(ids
            .iter()
            .filter_map(|id| {
                cache
                    .wrapped_domains
                    .get(id)
                    .cloned()
                    .map(|row| (id.clone(), row))
            })
            .collect())
    }
}

async fn write_account_snapshots(
    storage: &Storage,
    block_number: i32,
    rows: &[String],
) -> StorageResult<()> {
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            "insert into account_snapshots (id, block_number, deleted) ",
        );
        query.push_values(chunk, |mut row, id| {
            row.push_bind(id).push_bind(block_number).push_bind(false);
        });
        query.push(" on conflict (id, block_number) do update set deleted = excluded.deleted");
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_domain_snapshots(
    storage: &Storage,
    block_number: i32,
    rows: &[DomainRow],
) -> StorageResult<()> {
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into domain_snapshots (
                id, block_number, deleted, name, label_name, labelhash, parent_id,
                subdomain_count, resolved_address_id, resolver_id, ttl, is_migrated,
                created_at, owner_id, registrant_id, wrapped_owner_id, expiry_date
            ) "#,
        );
        query.push_values(chunk, |mut row, domain| {
            row.push_bind(&domain.id)
                .push_bind(block_number)
                .push_bind(false)
                .push_bind(&domain.name)
                .push_bind(&domain.label_name)
                .push_bind(&domain.labelhash)
                .push_bind(&domain.parent_id)
                .push_bind(domain.subdomain_count)
                .push_bind(&domain.resolved_address_id)
                .push_bind(&domain.resolver_id)
                .push_bind(&domain.ttl)
                .push_bind(domain.is_migrated)
                .push_bind(&domain.created_at)
                .push_bind(&domain.owner_id)
                .push_bind(&domain.registrant_id)
                .push_bind(&domain.wrapped_owner_id)
                .push_bind(&domain.expiry_date);
        });
        query.push(
            r#" on conflict (id, block_number) do update set
                deleted = excluded.deleted,
                name = excluded.name,
                label_name = excluded.label_name,
                labelhash = excluded.labelhash,
                parent_id = excluded.parent_id,
                subdomain_count = excluded.subdomain_count,
                resolved_address_id = excluded.resolved_address_id,
                resolver_id = excluded.resolver_id,
                ttl = excluded.ttl,
                is_migrated = excluded.is_migrated,
                created_at = excluded.created_at,
                owner_id = excluded.owner_id,
                registrant_id = excluded.registrant_id,
                wrapped_owner_id = excluded.wrapped_owner_id,
                expiry_date = excluded.expiry_date"#,
        );
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_registration_snapshots(
    storage: &Storage,
    block_number: i32,
    rows: &[RegistrationRow],
) -> StorageResult<()> {
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into registration_snapshots (
                id, block_number, deleted, domain_id, registration_date,
                expiry_date, cost, registrant_id, label_name
            ) "#,
        );
        query.push_values(chunk, |mut row, registration| {
            row.push_bind(&registration.id)
                .push_bind(block_number)
                .push_bind(false)
                .push_bind(&registration.domain_id)
                .push_bind(&registration.registration_date)
                .push_bind(&registration.expiry_date)
                .push_bind(&registration.cost)
                .push_bind(&registration.registrant_id)
                .push_bind(&registration.label_name);
        });
        query.push(
            r#" on conflict (id, block_number) do update set
                deleted = excluded.deleted,
                domain_id = excluded.domain_id,
                registration_date = excluded.registration_date,
                expiry_date = excluded.expiry_date,
                cost = excluded.cost,
                registrant_id = excluded.registrant_id,
                label_name = excluded.label_name"#,
        );
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_resolver_snapshots(
    storage: &Storage,
    block_number: i32,
    rows: &[ResolverRow],
) -> StorageResult<()> {
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into resolver_snapshots (
                id, block_number, deleted, domain_id, address, addr_id,
                content_hash, texts, coin_types
            ) "#,
        );
        query.push_values(chunk, |mut row, resolver| {
            row.push_bind(&resolver.id)
                .push_bind(block_number)
                .push_bind(false)
                .push_bind(&resolver.domain_id)
                .push_bind(&resolver.address)
                .push_bind(&resolver.addr_id)
                .push_bind(&resolver.content_hash)
                .push_bind(&resolver.texts)
                .push_bind(&resolver.coin_types);
        });
        query.push(
            r#" on conflict (id, block_number) do update set
                deleted = excluded.deleted,
                domain_id = excluded.domain_id,
                address = excluded.address,
                addr_id = excluded.addr_id,
                content_hash = excluded.content_hash,
                texts = excluded.texts,
                coin_types = excluded.coin_types"#,
        );
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_wrapped_domain_snapshots(
    storage: &Storage,
    block_number: i32,
    rows: &[(String, Option<WrappedDomainRow>)],
) -> StorageResult<()> {
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into wrapped_domain_snapshots (
                id, block_number, deleted, domain_id, expiry_date, fuses, owner_id, name
            ) "#,
        );
        query.push_values(chunk, |mut row, (id, wrapped)| {
            row.push_bind(id).push_bind(block_number);
            match wrapped {
                Some(wrapped) => {
                    row.push_bind(false)
                        .push_bind(Some(&wrapped.domain_id))
                        .push_bind(Some(&wrapped.expiry_date))
                        .push_bind(Some(wrapped.fuses))
                        .push_bind(Some(&wrapped.owner_id))
                        .push_bind(&wrapped.name);
                }
                None => {
                    row.push_bind(true)
                        .push_bind(None::<String>)
                        .push_bind(None::<bigdecimal::BigDecimal>)
                        .push_bind(None::<i32>)
                        .push_bind(None::<String>)
                        .push_bind(None::<String>);
                }
            }
        });
        query.push(
            r#" on conflict (id, block_number) do update set
                deleted = excluded.deleted,
                domain_id = excluded.domain_id,
                expiry_date = excluded.expiry_date,
                fuses = excluded.fuses,
                owner_id = excluded.owner_id,
                name = excluded.name"#,
        );
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}
