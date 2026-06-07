use std::collections::{BTreeMap, BTreeSet};

use sqlx::{Postgres, QueryBuilder};

use crate::{Storage, StorageError, StorageResult};

const CHANGE_FLUSH_CHUNK_ROWS: usize = 10_000;

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

        for ((kind, block_number), ids) in grouped {
            self.write_entity_changes_batch(kind, block_number, &ids)
                .await?;
            self.write_snapshots_batch(kind, block_number, &ids).await?;
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

    async fn write_snapshots_batch(
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

    async fn write_account_snapshots_batch(
        &self,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        for chunk in ids.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = changed_ids_query(chunk);
            query
                .push(
                    r#")
                    insert into account_snapshots (id, block_number, deleted)
                    select accounts.id, "#,
                )
                .push_bind(block_number)
                .push(
                    r#", false
                    from accounts
                    join changed on changed.id = accounts.id
                    on conflict (id, block_number) do update
                    set deleted = excluded.deleted"#,
                );
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }

    async fn write_domain_snapshots_batch(
        &self,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        for chunk in ids.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = changed_ids_query(chunk);
            query
                .push(
                    r#")
                    insert into domain_snapshots (
                        id, block_number, deleted, name, label_name, labelhash, parent_id,
                        subdomain_count, resolved_address_id, resolver_id, ttl, is_migrated,
                        created_at, owner_id, registrant_id, wrapped_owner_id, expiry_date
                    )
                    select domains.id, "#,
                )
                .push_bind(block_number)
                .push(
                    r#", false, name, label_name, labelhash, parent_id,
                       subdomain_count, resolved_address_id, resolver_id, ttl, is_migrated,
                       created_at, owner_id, registrant_id, wrapped_owner_id, expiry_date
                    from domains
                    join changed on changed.id = domains.id
                    on conflict (id, block_number) do update
                    set deleted = excluded.deleted,
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
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }

    async fn write_registration_snapshots_batch(
        &self,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        for chunk in ids.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = changed_ids_query(chunk);
            query
                .push(
                    r#")
                    insert into registration_snapshots (
                        id, block_number, deleted, domain_id, registration_date,
                        expiry_date, cost, registrant_id, label_name
                    )
                    select registrations.id, "#,
                )
                .push_bind(block_number)
                .push(
                    r#", false, domain_id, registration_date, expiry_date,
                       cost, registrant_id, label_name
                    from registrations
                    join changed on changed.id = registrations.id
                    on conflict (id, block_number) do update
                    set deleted = excluded.deleted,
                        domain_id = excluded.domain_id,
                        registration_date = excluded.registration_date,
                        expiry_date = excluded.expiry_date,
                        cost = excluded.cost,
                        registrant_id = excluded.registrant_id,
                        label_name = excluded.label_name"#,
                );
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }

    async fn write_resolver_snapshots_batch(
        &self,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        for chunk in ids.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = changed_ids_query(chunk);
            query
                .push(
                    r#")
                    insert into resolver_snapshots (
                        id, block_number, deleted, domain_id, address, addr_id,
                        content_hash, texts, coin_types
                    )
                    select resolvers.id, "#,
                )
                .push_bind(block_number)
                .push(
                    r#", false, domain_id, address, addr_id, content_hash, texts, coin_types
                    from resolvers
                    join changed on changed.id = resolvers.id
                    on conflict (id, block_number) do update
                    set deleted = excluded.deleted,
                        domain_id = excluded.domain_id,
                        address = excluded.address,
                        addr_id = excluded.addr_id,
                        content_hash = excluded.content_hash,
                        texts = excluded.texts,
                        coin_types = excluded.coin_types"#,
                );
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }

    async fn write_wrapped_domain_snapshots_batch(
        &self,
        block_number: i32,
        ids: &[String],
    ) -> StorageResult<()> {
        for chunk in ids.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
            let mut query = changed_ids_query(chunk);
            query
                .push(
                    r#")
                    insert into wrapped_domain_snapshots (
                        id, block_number, deleted, domain_id, expiry_date, fuses, owner_id, name
                    )
                    select changed.id, "#,
                )
                .push_bind(block_number)
                .push(
                    r#", wrapped_domains.id is null, domain_id, expiry_date, fuses, owner_id, name
                    from changed
                    left join wrapped_domains on wrapped_domains.id = changed.id
                    on conflict (id, block_number) do update
                    set deleted = excluded.deleted,
                        domain_id = excluded.domain_id,
                        expiry_date = excluded.expiry_date,
                        fuses = excluded.fuses,
                        owner_id = excluded.owner_id,
                        name = excluded.name"#,
                );
            query.build().execute(self.pool()).await?;
        }
        Ok(())
    }
}

fn changed_ids_query(ids: &[String]) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::<Postgres>::new("with changed(id) as (");
    query.push_values(ids, |mut row, id| {
        row.push_bind(id);
    });
    query
}
