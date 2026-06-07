use std::collections::{BTreeMap, BTreeSet};

use sqlx::{Postgres, QueryBuilder};

use crate::{
    AccountRow, DomainRow, EntityCache, EntityPreloadIds, EntityPreloadStats, RegistrationRow,
    ResolverRow, Storage, StorageError, StorageResult, WrappedDomainRow,
};

const PRELOAD_CHUNK_ROWS: usize = 10_000;

impl Storage {
    pub async fn preload_entity_cache(
        &self,
        ids: EntityPreloadIds,
    ) -> StorageResult<EntityPreloadStats> {
        let pending = {
            let guard = self
                .entity_cache
                .lock()
                .map_err(|_| StorageError::EntityCachePoisoned)?;
            let Some(cache) = guard.as_ref() else {
                return Ok(EntityPreloadStats::default());
            };
            PendingPreloadIds {
                accounts: ids
                    .accounts
                    .into_iter()
                    .filter(|id| cache.get_account_state(id).is_none())
                    .collect(),
                domains: ids
                    .domains
                    .into_iter()
                    .filter(|id| !cache.domains.contains_key(id))
                    .collect(),
                registrations: ids
                    .registrations
                    .into_iter()
                    .filter(|id| !cache.registrations.contains_key(id))
                    .collect(),
                resolvers: ids
                    .resolvers
                    .into_iter()
                    .filter(|id| !cache.resolvers.contains_key(id))
                    .collect(),
                wrapped_domains: ids
                    .wrapped_domains
                    .into_iter()
                    .filter(|id| !cache.wrapped_domains.contains_key(id))
                    .collect(),
            }
        };

        let accounts = load_accounts(self, &pending.accounts).await?;
        let domains = load_domains(self, &pending.domains).await?;
        let registrations = load_registrations(self, &pending.registrations).await?;
        let resolvers = load_resolvers(self, &pending.resolvers).await?;
        let wrapped_domains = load_wrapped_domains(self, &pending.wrapped_domains).await?;

        let stats = EntityPreloadStats {
            accounts: pending.accounts.len(),
            domains: pending.domains.len(),
            registrations: pending.registrations.len(),
            resolvers: pending.resolvers.len(),
            wrapped_domains: pending.wrapped_domains.len(),
        };

        let mut guard = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = guard.as_mut() else {
            return Ok(EntityPreloadStats::default());
        };
        preload_accounts(cache, pending.accounts, accounts);
        preload_optional_rows(&mut cache.domains, pending.domains, domains);
        preload_optional_rows(
            &mut cache.registrations,
            pending.registrations,
            registrations,
        );
        preload_optional_rows(&mut cache.resolvers, pending.resolvers, resolvers);
        preload_optional_rows(
            &mut cache.wrapped_domains,
            pending.wrapped_domains,
            wrapped_domains,
        );
        Ok(stats)
    }
}

#[derive(Debug, Default)]
struct PendingPreloadIds {
    accounts: BTreeSet<String>,
    domains: BTreeSet<String>,
    registrations: BTreeSet<String>,
    resolvers: BTreeSet<String>,
    wrapped_domains: BTreeSet<String>,
}

fn preload_accounts(cache: &mut EntityCache, ids: BTreeSet<String>, rows: Vec<AccountRow>) {
    let found = rows
        .into_iter()
        .map(|row| (row.id.clone(), row))
        .collect::<BTreeMap<_, _>>();
    for id in ids {
        if let Some(row) = found.get(&id) {
            cache.remember_account(row.id.clone());
        } else {
            cache.remember_missing_account(id);
        }
    }
}

fn preload_optional_rows<T: Clone>(
    cache: &mut BTreeMap<String, Option<T>>,
    ids: BTreeSet<String>,
    rows: Vec<(String, T)>,
) {
    let found = rows.into_iter().collect::<BTreeMap<_, _>>();
    for id in ids {
        cache
            .entry(id.clone())
            .or_insert_with(|| found.get(&id).cloned());
    }
}

async fn load_accounts(
    storage: &Storage,
    ids: &BTreeSet<String>,
) -> StorageResult<Vec<AccountRow>> {
    let mut out = Vec::new();
    for chunk in chunk_ids(ids) {
        let mut query = QueryBuilder::<Postgres>::new("select id from accounts where id in (");
        push_id_binds(&mut query, &chunk);
        query.push(")");
        out.extend(
            query
                .build_query_as::<AccountRow>()
                .fetch_all(storage.pool())
                .await?,
        );
    }
    Ok(out)
}

async fn load_domains(
    storage: &Storage,
    ids: &BTreeSet<String>,
) -> StorageResult<Vec<(String, DomainRow)>> {
    let mut out = Vec::new();
    for chunk in chunk_ids(ids) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            select id, name, label_name, labelhash, parent_id, subdomain_count,
                   resolved_address_id, resolver_id, ttl, is_migrated, created_at,
                   owner_id, registrant_id, wrapped_owner_id, expiry_date
            from domains
            where id in (
            "#,
        );
        push_id_binds(&mut query, &chunk);
        query.push(")");
        out.extend(
            query
                .build_query_as::<DomainRow>()
                .fetch_all(storage.pool())
                .await?
                .into_iter()
                .map(|row| (row.id.clone(), row)),
        );
    }
    Ok(out)
}

async fn load_registrations(
    storage: &Storage,
    ids: &BTreeSet<String>,
) -> StorageResult<Vec<(String, RegistrationRow)>> {
    let mut out = Vec::new();
    for chunk in chunk_ids(ids) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            select id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name
            from registrations
            where id in (
            "#,
        );
        push_id_binds(&mut query, &chunk);
        query.push(")");
        out.extend(
            query
                .build_query_as::<RegistrationRow>()
                .fetch_all(storage.pool())
                .await?
                .into_iter()
                .map(|row| (row.id.clone(), row)),
        );
    }
    Ok(out)
}

async fn load_resolvers(
    storage: &Storage,
    ids: &BTreeSet<String>,
) -> StorageResult<Vec<(String, ResolverRow)>> {
    let mut out = Vec::new();
    for chunk in chunk_ids(ids) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            select id, domain_id, address, addr_id, content_hash, texts, coin_types
            from resolvers
            where id in (
            "#,
        );
        push_id_binds(&mut query, &chunk);
        query.push(")");
        out.extend(
            query
                .build_query_as::<ResolverRow>()
                .fetch_all(storage.pool())
                .await?
                .into_iter()
                .map(|row| (row.id.clone(), row)),
        );
    }
    Ok(out)
}

async fn load_wrapped_domains(
    storage: &Storage,
    ids: &BTreeSet<String>,
) -> StorageResult<Vec<(String, WrappedDomainRow)>> {
    let mut out = Vec::new();
    for chunk in chunk_ids(ids) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            select id, domain_id, expiry_date, fuses, owner_id, name
            from wrapped_domains
            where id in (
            "#,
        );
        push_id_binds(&mut query, &chunk);
        query.push(")");
        out.extend(
            query
                .build_query_as::<WrappedDomainRow>()
                .fetch_all(storage.pool())
                .await?
                .into_iter()
                .map(|row| (row.id.clone(), row)),
        );
    }
    Ok(out)
}

fn chunk_ids(ids: &BTreeSet<String>) -> Vec<Vec<&str>> {
    ids.iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .chunks(PRELOAD_CHUNK_ROWS)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn push_id_binds(query: &mut QueryBuilder<Postgres>, ids: &[&str]) {
    let mut separated = query.separated(", ");
    for id in ids {
        separated.push_bind(*id);
    }
}
