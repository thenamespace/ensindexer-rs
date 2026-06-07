use crate::{error::*, inserts::*};

use super::EventsRepo;

impl EventsRepo<'_> {
    pub async fn insert_transfer(&self, event: TransferEventInsert) -> StorageResult<()> {
        if self.buffer_transfer(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into transfer_events (id, domain_id, block_number, transaction_id, owner_id)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_new_owner(&self, event: NewOwnerEventInsert) -> StorageResult<()> {
        if self.buffer_new_owner(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into new_owner_events (
                id, domain_id, block_number, transaction_id, parent_domain_id, owner_id
            )
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.parent_domain_id)
        .bind(event.owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_new_resolver(&self, event: NewResolverEventInsert) -> StorageResult<()> {
        if self.buffer_new_resolver(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into new_resolver_events (
                id, domain_id, block_number, transaction_id, resolver_id
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.resolver_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_new_ttl(&self, event: NewTtlEventInsert) -> StorageResult<()> {
        if self.buffer_new_ttl(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into new_ttl_events (
                id, domain_id, block_number, transaction_id, ttl
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.ttl)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_wrapped_transfer(
        &self,
        event: WrappedTransferEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_wrapped_transfer(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into wrapped_transfer_events (id, domain_id, block_number, transaction_id, owner_id)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_name_wrapped(&self, event: NameWrappedEventInsert) -> StorageResult<()> {
        if self.buffer_name_wrapped(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into name_wrapped_events (
                id, domain_id, block_number, transaction_id, name, fuses, owner_id, expiry_date
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.name)
        .bind(event.fuses)
        .bind(event.owner_id)
        .bind(event.expiry_date)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_name_unwrapped(
        &self,
        event: NameUnwrappedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_name_unwrapped(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into name_unwrapped_events (id, domain_id, block_number, transaction_id, owner_id)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_fuses_set(&self, event: FusesSetEventInsert) -> StorageResult<()> {
        if self.buffer_fuses_set(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into fuses_set_events (id, domain_id, block_number, transaction_id, fuses)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.fuses)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_expiry_extended(
        &self,
        event: ExpiryExtendedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_expiry_extended(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into expiry_extended_events (id, domain_id, block_number, transaction_id, expiry_date)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.domain_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.expiry_date)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
