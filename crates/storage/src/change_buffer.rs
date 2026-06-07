use std::collections::BTreeSet;

use crate::{Storage, StorageError, StorageResult};

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
        for change in changes {
            self.write_entity_change(change.kind, &change.id, change.block_number)
                .await?;
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
}
