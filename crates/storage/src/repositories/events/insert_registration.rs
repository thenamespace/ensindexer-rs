use crate::{error::*, inserts::*};

use super::EventsRepo;

impl EventsRepo<'_> {
    pub async fn insert_name_registered(
        &self,
        event: NameRegisteredEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_name_registered(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into name_registered_events (
                id, registration_id, block_number, transaction_id, registrant_id, expiry_date
            )
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.registration_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.registrant_id)
        .bind(event.expiry_date)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_name_renewed(&self, event: NameRenewedEventInsert) -> StorageResult<()> {
        if self.buffer_name_renewed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into name_renewed_events (
                id, registration_id, block_number, transaction_id, expiry_date
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.registration_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.expiry_date)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_name_transferred(
        &self,
        event: NameTransferredEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_name_transferred(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into name_transferred_events (
                id, registration_id, block_number, transaction_id, new_owner_id
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.registration_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.new_owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
