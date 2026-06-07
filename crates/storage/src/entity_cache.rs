use std::collections::{BTreeMap, BTreeSet};

use sqlx::{Postgres, QueryBuilder};

use crate::{
    AccountRow, DomainRow, RegistrationRow, ResolverRow, Storage, StorageError, StorageResult,
    WrappedDomainRow,
};

const CURRENT_FLUSH_CHUNK_ROWS: usize = 3_000;

#[derive(Debug, Default)]
pub struct EntityCache {
    pub(crate) accounts: BTreeMap<String, AccountRow>,
    pub(crate) domains: BTreeMap<String, Option<DomainRow>>,
    pub(crate) registrations: BTreeMap<String, Option<RegistrationRow>>,
    pub(crate) resolvers: BTreeMap<String, Option<ResolverRow>>,
    pub(crate) wrapped_domains: BTreeMap<String, Option<WrappedDomainRow>>,
    dirty_accounts: BTreeSet<String>,
    dirty_domains: BTreeSet<String>,
    dirty_registrations: BTreeSet<String>,
    dirty_resolvers: BTreeSet<String>,
    dirty_wrapped_domains: BTreeSet<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EntityCacheFlushStats {
    pub rows: usize,
}

impl EntityCache {
    pub(crate) fn has_account(&self, id: &str) -> bool {
        self.accounts.contains_key(id)
    }

    pub(crate) fn remember_account(&mut self, id: String) {
        self.accounts
            .entry(id.clone())
            .or_insert_with(|| AccountRow { id });
    }

    pub(crate) fn insert_account(&mut self, id: String) {
        self.accounts
            .entry(id.clone())
            .or_insert_with(|| AccountRow { id: id.clone() });
        self.dirty_accounts.insert(id);
    }

    pub(crate) fn get_domain(&self, id: &str) -> Option<Option<DomainRow>> {
        self.domains.get(id).cloned()
    }

    pub(crate) fn put_domain(&mut self, row: DomainRow) {
        self.dirty_domains.insert(row.id.clone());
        self.domains.insert(row.id.clone(), Some(row));
    }

    pub(crate) fn get_registration(&self, id: &str) -> Option<Option<RegistrationRow>> {
        self.registrations.get(id).cloned()
    }

    pub(crate) fn put_registration(&mut self, row: RegistrationRow) {
        self.dirty_registrations.insert(row.id.clone());
        self.registrations.insert(row.id.clone(), Some(row));
    }

    pub(crate) fn get_resolver(&self, id: &str) -> Option<Option<ResolverRow>> {
        self.resolvers.get(id).cloned()
    }

    pub(crate) fn put_resolver(&mut self, row: ResolverRow) {
        self.dirty_resolvers.insert(row.id.clone());
        self.resolvers.insert(row.id.clone(), Some(row));
    }

    pub(crate) fn get_wrapped_domain(&self, id: &str) -> Option<Option<WrappedDomainRow>> {
        self.wrapped_domains.get(id).cloned()
    }

    pub(crate) fn put_wrapped_domain(&mut self, row: WrappedDomainRow) {
        self.dirty_wrapped_domains.insert(row.id.clone());
        self.wrapped_domains.insert(row.id.clone(), Some(row));
    }

    pub(crate) fn delete_wrapped_domain(&mut self, id: &str) {
        self.dirty_wrapped_domains.insert(id.to_owned());
        self.wrapped_domains.insert(id.to_owned(), None);
    }

    async fn flush(&mut self, storage: &Storage) -> StorageResult<EntityCacheFlushStats> {
        let account_ids = take_dirty_rows(&mut self.dirty_accounts, &self.accounts);
        let domains = take_dirty_optional_rows(&mut self.dirty_domains, &self.domains);
        let registrations =
            take_dirty_optional_rows(&mut self.dirty_registrations, &self.registrations);
        let resolvers = take_dirty_optional_rows(&mut self.dirty_resolvers, &self.resolvers);
        let wrapped_domains =
            take_dirty_optional_rows(&mut self.dirty_wrapped_domains, &self.wrapped_domains);

        flush_accounts(storage, &account_ids).await?;
        flush_domains(storage, &domains).await?;
        flush_registrations(storage, &registrations).await?;
        flush_resolvers(storage, &resolvers).await?;
        flush_wrapped_domains(storage, &wrapped_domains).await?;

        Ok(EntityCacheFlushStats {
            rows: account_ids.len()
                + domains.len()
                + registrations.len()
                + resolvers.len()
                + wrapped_domains.len(),
        })
    }
}

impl Storage {
    pub fn begin_entity_cache(&self) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if cache.is_some() {
            return Err(StorageError::EntityCacheAlreadyActive);
        }
        *cache = Some(EntityCache::default());
        Ok(())
    }

    pub fn clear_entity_cache(&self) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        *cache = None;
        Ok(())
    }

    pub async fn flush_entity_cache(&self) -> StorageResult<EntityCacheFlushStats> {
        let mut cache = {
            let mut guard = self
                .entity_cache
                .lock()
                .map_err(|_| StorageError::EntityCachePoisoned)?;
            let Some(cache) = guard.take() else {
                return Ok(EntityCacheFlushStats::default());
            };
            cache
        };

        let stats = cache.flush(self).await?;

        let mut guard = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        *guard = Some(cache);
        Ok(stats)
    }
}

fn take_dirty_rows<T: Clone>(dirty: &mut BTreeSet<String>, rows: &BTreeMap<String, T>) -> Vec<T> {
    let ids = std::mem::take(dirty);
    ids.into_iter()
        .filter_map(|id| rows.get(&id).cloned())
        .collect()
}

fn take_dirty_optional_rows<T: Clone>(
    dirty: &mut BTreeSet<String>,
    rows: &BTreeMap<String, Option<T>>,
) -> Vec<(String, Option<T>)> {
    let ids = std::mem::take(dirty);
    ids.into_iter()
        .filter_map(|id| rows.get(&id).cloned().map(|row| (id, row)))
        .collect()
}

async fn flush_accounts(storage: &Storage, rows: &[AccountRow]) -> StorageResult<()> {
    for chunk in rows.chunks(CURRENT_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new("insert into accounts (id) ");
        query.push_values(chunk, |mut row, account| {
            row.push_bind(&account.id);
        });
        query.push(" on conflict (id) do nothing");
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn flush_domains(
    storage: &Storage,
    rows: &[(String, Option<DomainRow>)],
) -> StorageResult<()> {
    let rows = rows
        .iter()
        .filter_map(|(_, row)| row.clone())
        .collect::<Vec<_>>();
    for chunk in rows.chunks(CURRENT_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into domains (
                id, name, label_name, labelhash, parent_id, subdomain_count,
                resolved_address_id, resolver_id, ttl, is_migrated, created_at,
                owner_id, registrant_id, wrapped_owner_id, expiry_date
            ) "#,
        );
        query.push_values(chunk, |mut row, domain| {
            row.push_bind(&domain.id)
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
            r#" on conflict (id) do update set
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

async fn flush_registrations(
    storage: &Storage,
    rows: &[(String, Option<RegistrationRow>)],
) -> StorageResult<()> {
    let rows = rows
        .iter()
        .filter_map(|(_, row)| row.clone())
        .collect::<Vec<_>>();
    for chunk in rows.chunks(CURRENT_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into registrations (
                id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name
            ) "#,
        );
        query.push_values(chunk, |mut row, registration| {
            row.push_bind(&registration.id)
                .push_bind(&registration.domain_id)
                .push_bind(&registration.registration_date)
                .push_bind(&registration.expiry_date)
                .push_bind(&registration.cost)
                .push_bind(&registration.registrant_id)
                .push_bind(&registration.label_name);
        });
        query.push(
            r#" on conflict (id) do update set
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

async fn flush_resolvers(
    storage: &Storage,
    rows: &[(String, Option<ResolverRow>)],
) -> StorageResult<()> {
    let rows = rows
        .iter()
        .filter_map(|(_, row)| row.clone())
        .collect::<Vec<_>>();
    for chunk in rows.chunks(CURRENT_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into resolvers (
                id, domain_id, address, addr_id, content_hash, texts, coin_types
            ) "#,
        );
        query.push_values(chunk, |mut row, resolver| {
            row.push_bind(&resolver.id)
                .push_bind(&resolver.domain_id)
                .push_bind(&resolver.address)
                .push_bind(&resolver.addr_id)
                .push_bind(&resolver.content_hash)
                .push_bind(&resolver.texts)
                .push_bind(&resolver.coin_types);
        });
        query.push(
            r#" on conflict (id) do update set
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

async fn flush_wrapped_domains(
    storage: &Storage,
    rows: &[(String, Option<WrappedDomainRow>)],
) -> StorageResult<()> {
    let upserts = rows
        .iter()
        .filter_map(|(_, row)| row.clone())
        .collect::<Vec<_>>();
    let delete_ids = rows
        .iter()
        .filter_map(|(id, row)| row.is_none().then_some(id.as_str()))
        .collect::<Vec<_>>();

    if !delete_ids.is_empty() {
        for chunk in delete_ids.chunks(CURRENT_FLUSH_CHUNK_ROWS) {
            let mut query =
                QueryBuilder::<Postgres>::new("delete from wrapped_domains where id in (");
            let mut separated = query.separated(", ");
            for id in chunk {
                separated.push_bind(id);
            }
            query.push(")");
            query.build().execute(storage.pool()).await?;
        }
    }

    for chunk in upserts.chunks(CURRENT_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into wrapped_domains (
                id, domain_id, expiry_date, fuses, owner_id, name
            ) "#,
        );
        query.push_values(chunk, |mut row, wrapped| {
            row.push_bind(&wrapped.id)
                .push_bind(&wrapped.domain_id)
                .push_bind(&wrapped.expiry_date)
                .push_bind(wrapped.fuses)
                .push_bind(&wrapped.owner_id)
                .push_bind(&wrapped.name);
        });
        query.push(
            r#" on conflict (id) do update set
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
