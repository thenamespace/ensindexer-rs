use std::collections::{BTreeMap, BTreeSet};

use sqlx::{Postgres, QueryBuilder};

use crate::{Storage, StorageError, StorageResult};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct EntityChange {
    kind: EntityKind,
    id: String,
    block_number: i32,
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
        *buffer = Some(BTreeSet::new());
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
        let changes = {
            let mut buffer = self
                .change_buffer
                .lock()
                .map_err(|_| StorageError::ChangeBufferPoisoned)?;
            let Some(active) = buffer.as_mut() else {
                return Err(StorageError::ChangeBufferNotActive);
            };
            std::mem::take(active)
        };

        let count = changes.len();
        let mut grouped = BTreeMap::<(EntityKind, i32), Vec<String>>::new();
        for change in changes {
            grouped
                .entry((change.kind, change.block_number))
                .or_default()
                .push(change.id);
        }

        let use_cached_snapshots = self.entity_cache_is_active()?;
        for ((kind, block_number), ids) in grouped {
            self.write_entity_changes_batch(kind, block_number, &ids)
                .await?;
            if use_cached_snapshots {
                self.write_cached_snapshots_batch(kind, block_number, &ids)
                    .await?;
            } else {
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
        Ok(true)
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
}
