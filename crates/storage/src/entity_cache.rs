use std::collections::{BTreeMap, BTreeSet};

use crate::{
    AccountRow, DomainRow, RegistrationRow, ResolverRow, Storage, StorageError, StorageResult,
    WrappedDomainRow,
};

#[derive(Debug, Default)]
pub struct EntityPreloadIds {
    pub accounts: BTreeSet<String>,
    pub domains: BTreeSet<String>,
    pub registrations: BTreeSet<String>,
    pub resolvers: BTreeSet<String>,
    pub wrapped_domains: BTreeSet<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EntityPreloadStats {
    pub accounts: usize,
    pub domains: usize,
    pub registrations: usize,
    pub resolvers: usize,
    pub wrapped_domains: usize,
}

impl EntityPreloadStats {
    pub fn rows(self) -> usize {
        self.accounts + self.domains + self.registrations + self.resolvers + self.wrapped_domains
    }
}

#[derive(Debug, Default)]
pub struct EntityCache {
    pub(crate) accounts: BTreeMap<String, AccountRow>,
    account_misses: BTreeSet<String>,
    pub(crate) domains: BTreeMap<String, Option<DomainRow>>,
    pub(crate) registrations: BTreeMap<String, Option<RegistrationRow>>,
    pub(crate) resolvers: BTreeMap<String, Option<ResolverRow>>,
    pub(crate) wrapped_domains: BTreeMap<String, Option<WrappedDomainRow>>,
    pub(crate) dirty_accounts: BTreeSet<String>,
    pub(crate) dirty_domains: BTreeSet<String>,
    pub(crate) dirty_registrations: BTreeSet<String>,
    pub(crate) dirty_resolvers: BTreeSet<String>,
    pub(crate) dirty_wrapped_domains: BTreeSet<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EntityCacheFlushStats {
    pub rows: usize,
}

impl EntityCache {
    pub(crate) fn get_account_state(&self, id: &str) -> Option<bool> {
        if self.accounts.contains_key(id) {
            Some(true)
        } else if self.account_misses.contains(id) {
            Some(false)
        } else {
            None
        }
    }

    pub(crate) fn remember_account(&mut self, id: String) {
        self.account_misses.remove(&id);
        self.accounts
            .entry(id.clone())
            .or_insert_with(|| AccountRow { id });
    }

    pub(crate) fn remember_missing_account(&mut self, id: String) {
        if !self.accounts.contains_key(&id) {
            self.account_misses.insert(id);
        }
    }

    pub(crate) fn insert_account(&mut self, id: String) {
        self.account_misses.remove(&id);
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

    pub fn ensure_entity_cache(&self) -> StorageResult<bool> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if cache.is_some() {
            return Ok(false);
        }
        *cache = Some(EntityCache::default());
        Ok(true)
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
