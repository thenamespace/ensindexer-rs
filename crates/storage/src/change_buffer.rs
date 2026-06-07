use std::collections::{BTreeMap, BTreeSet};

use sqlx::{Postgres, QueryBuilder};

use crate::{
    DomainRow, RegistrationRow, ResolverRow, Storage, StorageError, StorageResult, WrappedDomainRow,
};

mod snapshot_flush;

const CHANGE_FLUSH_CHUNK_ROWS: usize = 3_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityKind {
    Account,
    Domain,
    Registration,
    Resolver,
    WrappedDomain,
}

#[derive(Debug, Default)]
pub(crate) struct ChangeBuffer {
    changes: BTreeSet<EntityChange>,
    snapshots: BufferedSnapshots,
}

#[derive(Debug, Default)]
pub(super) struct BufferedSnapshots {
    pub(super) accounts: BTreeMap<(i32, String), String>,
    pub(super) domains: BTreeMap<(i32, String), DomainRow>,
    pub(super) registrations: BTreeMap<(i32, String), RegistrationRow>,
    pub(super) resolvers: BTreeMap<(i32, String), ResolverRow>,
    pub(super) wrapped_domains: BTreeMap<(i32, String), (String, Option<WrappedDomainRow>)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EntityChange {
    kind: EntityKind,
    id: String,
    block_number: i32,
}

enum CapturedSnapshot {
    Account(String),
    Domain(DomainRow),
    Registration(RegistrationRow),
    Resolver(ResolverRow),
    WrappedDomain(String, Option<WrappedDomainRow>),
}

impl EntityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "Account",
            Self::Domain => "Domain",
            Self::Registration => "Registration",
            Self::Resolver => "Resolver",
            Self::WrappedDomain => "WrappedDomain",
        }
    }
}

impl Storage {
    pub fn begin_change_buffer(&self) -> StorageResult<()> {
        let mut buffer = self
            .change_buffer
            .lock()
            .map_err(|_| StorageError::ChangeBufferPoisoned)?;
        if buffer.is_some() {
            return Err(StorageError::ChangeBufferAlreadyActive);
        }
        *buffer = Some(ChangeBuffer::default());
        Ok(())
    }

    pub fn clear_change_buffer(&self) -> StorageResult<()> {
        let mut buffer = self
            .change_buffer
            .lock()
            .map_err(|_| StorageError::ChangeBufferPoisoned)?;
        *buffer = None;
        Ok(())
    }

    pub async fn record_entity_change(
        &self,
        kind: EntityKind,
        id: &str,
        block_number: i32,
    ) -> StorageResult<()> {
        if self.buffer_entity_change(kind, id, block_number)? {
            return Ok(());
        }
        self.write_entity_change(kind, id, block_number).await
    }

    pub async fn flush_change_buffer(&self) -> StorageResult<usize> {
        let buffer = {
            let mut buffer = self
                .change_buffer
                .lock()
                .map_err(|_| StorageError::ChangeBufferPoisoned)?;
            let Some(active) = buffer.as_mut() else {
                return Err(StorageError::ChangeBufferNotActive);
            };
            std::mem::take(active)
        };

        let count = buffer.changes.len();
        let use_buffered_snapshots = !buffer.snapshots.is_empty();
        if use_buffered_snapshots {
            self.write_entity_changes_buffered(&buffer.changes).await?;
            self.write_buffered_snapshots(&buffer.snapshots).await?;
        } else {
            let mut grouped = BTreeMap::<(EntityKind, i32), Vec<String>>::new();
            for change in buffer.changes {
                grouped
                    .entry((change.kind, change.block_number))
                    .or_default()
                    .push(change.id);
            }
            for ((kind, block_number), ids) in grouped {
                self.write_entity_changes_batch(kind, block_number, &ids)
                    .await?;
                self.write_snapshots_batch(kind, block_number, &ids).await?;
            }
        }
        Ok(count)
    }

    fn buffer_entity_change(
        &self,
        kind: EntityKind,
        id: &str,
        block_number: i32,
    ) -> StorageResult<bool> {
        let has_buffer = {
            let buffer = self
                .change_buffer
                .lock()
                .map_err(|_| StorageError::ChangeBufferPoisoned)?;
            buffer.is_some()
        };
        if !has_buffer {
            return Ok(false);
        };

        let snapshot = self.capture_entity_snapshot(kind, id)?;
        let mut buffer = self
            .change_buffer
            .lock()
            .map_err(|_| StorageError::ChangeBufferPoisoned)?;
        let Some(active) = buffer.as_mut() else {
            return Ok(false);
        };
        active.insert(EntityChange {
            kind,
            id: id.to_owned(),
            block_number,
        });
        active.snapshots.insert(block_number, id, snapshot);
        Ok(true)
    }

    fn capture_entity_snapshot(
        &self,
        kind: EntityKind,
        id: &str,
    ) -> StorageResult<Option<CapturedSnapshot>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        let Some(cache) = cache.as_ref() else {
            return Ok(None);
        };
        Ok(match kind {
            EntityKind::Account => cache
                .accounts
                .contains_key(id)
                .then(|| CapturedSnapshot::Account(id.to_owned())),
            EntityKind::Domain => cache
                .domains
                .get(id)
                .and_then(Clone::clone)
                .map(CapturedSnapshot::Domain),
            EntityKind::Registration => cache
                .registrations
                .get(id)
                .and_then(Clone::clone)
                .map(CapturedSnapshot::Registration),
            EntityKind::Resolver => cache
                .resolvers
                .get(id)
                .and_then(Clone::clone)
                .map(CapturedSnapshot::Resolver),
            EntityKind::WrappedDomain => cache
                .wrapped_domains
                .get(id)
                .cloned()
                .map(|row| CapturedSnapshot::WrappedDomain(id.to_owned(), row)),
        })
    }

    async fn write_entity_change(
        &self,
        kind: EntityKind,
        id: &str,
        block_number: i32,
    ) -> StorageResult<()> {
        self.entity_changes()
            .record(kind.as_str(), id, block_number)
            .await?;
        match kind {
            EntityKind::Account => self.snapshots().record_account(id, block_number).await?,
            EntityKind::Domain => self.snapshots().record_domain(id, block_number).await?,
            EntityKind::Registration => {
                self.snapshots()
                    .record_registration(id, block_number)
                    .await?
            }
            EntityKind::Resolver => self.snapshots().record_resolver(id, block_number).await?,
            EntityKind::WrappedDomain => {
                self.snapshots()
                    .record_wrapped_domain(id, block_number)
                    .await?
            }
        }
        Ok(())
    }

    async fn write_entity_changes_batch(
        &self,
        kind: EntityKind,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        for chunk in ids.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = QueryBuilder::<Postgres>::new(
                "insert into entity_changes (entity_type, entity_id, block_number) ",
            );
            query.push_values(chunk, |mut row, id| {
                row.push_bind(kind.as_str())
                    .push_bind(id)
                    .push_bind(block_number);
            });
            query.push(" on conflict do nothing");
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }

    async fn write_entity_changes_buffered(
        &self,
        changes: &BTreeSet<EntityChange>,
    ) -> StorageResult<()> {
        let changes = changes.iter().collect::<Vec<_>>();
        for chunk in changes.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = QueryBuilder::<Postgres>::new(
                "insert into entity_changes (entity_type, entity_id, block_number) ",
            );
            query.push_values(chunk, |mut row, change| {
                row.push_bind(change.kind.as_str())
                    .push_bind(&change.id)
                    .push_bind(change.block_number);
            });
            query.push(" on conflict do nothing");
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }
}

impl ChangeBuffer {
    fn insert(&mut self, change: EntityChange) {
        self.changes.insert(change);
    }
}

impl BufferedSnapshots {
    fn is_empty(&self) -> bool {
        self.accounts.is_empty()
            && self.domains.is_empty()
            && self.registrations.is_empty()
            && self.resolvers.is_empty()
            && self.wrapped_domains.is_empty()
    }

    fn insert(&mut self, block_number: i32, id: &str, snapshot: Option<CapturedSnapshot>) {
        let Some(snapshot) = snapshot else {
            return;
        };
        let key = (block_number, id.to_owned());
        match snapshot {
            CapturedSnapshot::Account(account_id) => {
                self.accounts.insert(key, account_id);
            }
            CapturedSnapshot::Domain(row) => {
                self.domains.insert(key, row);
            }
            CapturedSnapshot::Registration(row) => {
                self.registrations.insert(key, row);
            }
            CapturedSnapshot::Resolver(row) => {
                self.resolvers.insert(key, row);
            }
            CapturedSnapshot::WrappedDomain(id, row) => {
                self.wrapped_domains.insert(key, (id, row));
            }
        }
    }
}
