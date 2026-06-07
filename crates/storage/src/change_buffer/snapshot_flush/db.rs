use sqlx::{Postgres, QueryBuilder};

use crate::{Storage, StorageResult};

use super::CHANGE_FLUSH_CHUNK_ROWS;

impl Storage {
    pub(super) async fn write_account_snapshots_batch(
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

    pub(super) async fn write_domain_snapshots_batch(
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

    pub(super) async fn write_registration_snapshots_batch(
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

    pub(super) async fn write_resolver_snapshots_batch(
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

    pub(super) async fn write_wrapped_domain_snapshots_batch(
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
