use sqlx::PgPool;

use crate::StorageResult;

pub struct SnapshotsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl SnapshotsRepo<'_> {
    pub async fn record_account(&self, id: &str, block_number: i32) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into account_snapshots (id, block_number, deleted)
            select id, $2, false from accounts where id = $1
            on conflict (id, block_number) do update
            set deleted = excluded.deleted
            "#,
        )
        .bind(id)
        .bind(block_number)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn record_domain(&self, id: &str, block_number: i32) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into domain_snapshots (
                id, block_number, deleted, name, label_name, labelhash, parent_id,
                subdomain_count, resolved_address_id, resolver_id, ttl, is_migrated,
                created_at, owner_id, registrant_id, wrapped_owner_id, expiry_date
            )
            select id, $2, false, name, label_name, labelhash, parent_id,
                   subdomain_count, resolved_address_id, resolver_id, ttl, is_migrated,
                   created_at, owner_id, registrant_id, wrapped_owner_id, expiry_date
            from domains
            where id = $1
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
                expiry_date = excluded.expiry_date
            "#,
        )
        .bind(id)
        .bind(block_number)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn record_registration(&self, id: &str, block_number: i32) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into registration_snapshots (
                id, block_number, deleted, domain_id, registration_date,
                expiry_date, cost, registrant_id, label_name
            )
            select id, $2, false, domain_id, registration_date, expiry_date,
                   cost, registrant_id, label_name
            from registrations
            where id = $1
            on conflict (id, block_number) do update
            set deleted = excluded.deleted,
                domain_id = excluded.domain_id,
                registration_date = excluded.registration_date,
                expiry_date = excluded.expiry_date,
                cost = excluded.cost,
                registrant_id = excluded.registrant_id,
                label_name = excluded.label_name
            "#,
        )
        .bind(id)
        .bind(block_number)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn record_resolver(&self, id: &str, block_number: i32) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into resolver_snapshots (
                id, block_number, deleted, domain_id, address, addr_id,
                content_hash, texts, coin_types
            )
            select id, $2, false, domain_id, address, addr_id,
                   content_hash, texts, coin_types
            from resolvers
            where id = $1
            on conflict (id, block_number) do update
            set deleted = excluded.deleted,
                domain_id = excluded.domain_id,
                address = excluded.address,
                addr_id = excluded.addr_id,
                content_hash = excluded.content_hash,
                texts = excluded.texts,
                coin_types = excluded.coin_types
            "#,
        )
        .bind(id)
        .bind(block_number)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn record_wrapped_domain(&self, id: &str, block_number: i32) -> StorageResult<()> {
        let rows = sqlx::query(
            r#"
            insert into wrapped_domain_snapshots (
                id, block_number, deleted, domain_id, expiry_date, fuses, owner_id, name
            )
            select id, $2, false, domain_id, expiry_date, fuses, owner_id, name
            from wrapped_domains
            where id = $1
            on conflict (id, block_number) do update
            set deleted = excluded.deleted,
                domain_id = excluded.domain_id,
                expiry_date = excluded.expiry_date,
                fuses = excluded.fuses,
                owner_id = excluded.owner_id,
                name = excluded.name
            "#,
        )
        .bind(id)
        .bind(block_number)
        .execute(self.pool)
        .await?;

        if rows.rows_affected() == 0 {
            self.record_wrapped_domain_deleted(id, block_number).await?;
        }
        Ok(())
    }

    async fn record_wrapped_domain_deleted(
        &self,
        id: &str,
        block_number: i32,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into wrapped_domain_snapshots (id, block_number, deleted)
            values ($1, $2, true)
            on conflict (id, block_number) do update
            set deleted = excluded.deleted,
                domain_id = null,
                expiry_date = null,
                fuses = null,
                owner_id = null,
                name = null
            "#,
        )
        .bind(id)
        .bind(block_number)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
