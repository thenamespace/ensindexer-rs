use std::collections::BTreeSet;

use crate::{Storage, StorageError, StorageResult};

#[derive(Debug, Default)]
pub struct EntityCache {
    accounts: BTreeSet<String>,
    domains: BTreeSet<String>,
    resolvers: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum CachedEntityKind {
    Account,
    Domain,
    Resolver,
}

impl EntityCache {
    pub(crate) fn contains(&self, kind: CachedEntityKind, id: &str) -> bool {
        match kind {
            CachedEntityKind::Account => self.accounts.contains(id),
            CachedEntityKind::Domain => self.domains.contains(id),
            CachedEntityKind::Resolver => self.resolvers.contains(id),
        }
    }

    pub(crate) fn insert(&mut self, kind: CachedEntityKind, id: String) {
        match kind {
            CachedEntityKind::Account => self.accounts.insert(id),
            CachedEntityKind::Domain => self.domains.insert(id),
            CachedEntityKind::Resolver => self.resolvers.insert(id),
        };
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
}
