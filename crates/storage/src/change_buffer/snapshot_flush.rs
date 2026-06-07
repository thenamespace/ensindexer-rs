use sqlx::{Postgres, QueryBuilder};

use crate::{EntityKind, Storage, StorageResult};

use super::{BufferedSnapshots, CHANGE_FLUSH_CHUNK_ROWS};

mod db;

impl Storage {
    pub(super) async fn write_buffered_snapshots(
        &self,
        snapshots: &BufferedSnapshots,
    ) -> StorageResult<()> {
        write_buffered_account_snapshots(self, snapshots).await?;
        write_buffered_domain_snapshots(self, snapshots).await?;
        write_buffered_registration_snapshots(self, snapshots).await?;
        write_buffered_resolver_snapshots(self, snapshots).await?;
        write_buffered_wrapped_domain_snapshots(self, snapshots).await?;
        Ok(())
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
}

async fn write_buffered_account_snapshots(
    storage: &Storage,
    snapshots: &BufferedSnapshots,
) -> StorageResult<()> {
    let rows = snapshots.accounts.iter().collect::<Vec<_>>();
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            "insert into account_snapshots (id, block_number, deleted) ",
        );
        query.push_values(chunk, |mut row, ((block_number, _), id)| {
            row.push_bind(id).push_bind(*block_number).push_bind(false);
        });
        query.push(" on conflict (id, block_number) do update set deleted = excluded.deleted");
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_buffered_domain_snapshots(
    storage: &Storage,
    snapshots: &BufferedSnapshots,
) -> StorageResult<()> {
    let rows = snapshots.domains.iter().collect::<Vec<_>>();
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into domain_snapshots (
                id, block_number, deleted, name, label_name, labelhash, parent_id,
                subdomain_count, resolved_address_id, resolver_id, ttl, is_migrated,
                created_at, owner_id, registrant_id, wrapped_owner_id, expiry_date
            ) "#,
        );
        query.push_values(chunk, |mut row, ((block_number, _), domain)| {
            row.push_bind(&domain.id)
                .push_bind(*block_number)
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
        push_domain_snapshot_conflict(&mut query);
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_buffered_registration_snapshots(
    storage: &Storage,
    snapshots: &BufferedSnapshots,
) -> StorageResult<()> {
    let rows = snapshots.registrations.iter().collect::<Vec<_>>();
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into registration_snapshots (
                id, block_number, deleted, domain_id, registration_date,
                expiry_date, cost, registrant_id, label_name
            ) "#,
        );
        query.push_values(chunk, |mut row, ((block_number, _), registration)| {
            row.push_bind(&registration.id)
                .push_bind(*block_number)
                .push_bind(false)
                .push_bind(&registration.domain_id)
                .push_bind(&registration.registration_date)
                .push_bind(&registration.expiry_date)
                .push_bind(&registration.cost)
                .push_bind(&registration.registrant_id)
                .push_bind(&registration.label_name);
        });
        push_registration_snapshot_conflict(&mut query);
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_buffered_resolver_snapshots(
    storage: &Storage,
    snapshots: &BufferedSnapshots,
) -> StorageResult<()> {
    let rows = snapshots.resolvers.iter().collect::<Vec<_>>();
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into resolver_snapshots (
                id, block_number, deleted, domain_id, address, addr_id,
                content_hash, texts, coin_types
            ) "#,
        );
        query.push_values(chunk, |mut row, ((block_number, _), resolver)| {
            row.push_bind(&resolver.id)
                .push_bind(*block_number)
                .push_bind(false)
                .push_bind(&resolver.domain_id)
                .push_bind(&resolver.address)
                .push_bind(&resolver.addr_id)
                .push_bind(&resolver.content_hash)
                .push_bind(&resolver.texts)
                .push_bind(&resolver.coin_types);
        });
        push_resolver_snapshot_conflict(&mut query);
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

async fn write_buffered_wrapped_domain_snapshots(
    storage: &Storage,
    snapshots: &BufferedSnapshots,
) -> StorageResult<()> {
    let rows = snapshots.wrapped_domains.iter().collect::<Vec<_>>();
    for chunk in rows.chunks(CHANGE_FLUSH_CHUNK_ROWS) {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"insert into wrapped_domain_snapshots (
                id, block_number, deleted, domain_id, expiry_date, fuses, owner_id, name
            ) "#,
        );
        query.push_values(chunk, |mut row, ((block_number, _), (id, wrapped))| {
            row.push_bind(id).push_bind(*block_number);
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
        push_wrapped_domain_snapshot_conflict(&mut query);
        query.build().execute(storage.pool()).await?;
    }
    Ok(())
}

fn push_domain_snapshot_conflict(query: &mut QueryBuilder<Postgres>) {
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
}

fn push_registration_snapshot_conflict(query: &mut QueryBuilder<Postgres>) {
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
}

fn push_resolver_snapshot_conflict(query: &mut QueryBuilder<Postgres>) {
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
}

fn push_wrapped_domain_snapshot_conflict(query: &mut QueryBuilder<Postgres>) {
    query.push(
        r#" on conflict (id, block_number) do update set
            deleted = excluded.deleted,
            domain_id = excluded.domain_id,
            expiry_date = excluded.expiry_date,
            fuses = excluded.fuses,
            owner_id = excluded.owner_id,
            name = excluded.name"#,
    );
}
