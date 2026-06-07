use crate::{error::*, inserts::*};

use super::EventsRepo;

impl EventsRepo<'_> {
    pub async fn insert_addr_changed(&self, event: AddrChangedEventInsert) -> StorageResult<()> {
        if self.buffer_addr_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into addr_changed_events (id, resolver_id, block_number, transaction_id, addr_id)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.addr_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_multicoin_addr_changed(
        &self,
        event: MulticoinAddrChangedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_multicoin_addr_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into multicoin_addr_changed_events (
                id, resolver_id, block_number, transaction_id, coin_type, addr
            )
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.coin_type)
        .bind(event.addr)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_name_changed(&self, event: NameChangedEventInsert) -> StorageResult<()> {
        if self.buffer_name_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into name_changed_events (id, resolver_id, block_number, transaction_id, name)
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_abi_changed(&self, event: AbiChangedEventInsert) -> StorageResult<()> {
        if self.buffer_abi_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into abi_changed_events (
                id, resolver_id, block_number, transaction_id, content_type
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.content_type)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_pubkey_changed(
        &self,
        event: PubkeyChangedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_pubkey_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into pubkey_changed_events (id, resolver_id, block_number, transaction_id, x, y)
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.x)
        .bind(event.y)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_text_changed(&self, event: TextChangedEventInsert) -> StorageResult<()> {
        if self.buffer_text_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into text_changed_events (
                id, resolver_id, block_number, transaction_id, key, value
            )
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.key)
        .bind(event.value)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_contenthash_changed(
        &self,
        event: ContenthashChangedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_contenthash_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into contenthash_changed_events (
                id, resolver_id, block_number, transaction_id, hash
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.hash)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_interface_changed(
        &self,
        event: InterfaceChangedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_interface_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into interface_changed_events (
                id, resolver_id, block_number, transaction_id, interface_id, implementer
            )
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.interface_id)
        .bind(event.implementer)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_authorisation_changed(
        &self,
        event: AuthorisationChangedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_authorisation_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into authorisation_changed_events (
                id, resolver_id, block_number, transaction_id, owner, target, is_authorized
            )
            values ($1, $2, $3, $4, $5, $6, $7)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.owner)
        .bind(event.target)
        .bind(event.is_authorized)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_version_changed(
        &self,
        event: VersionChangedEventInsert,
    ) -> StorageResult<()> {
        if self.buffer_version_changed(event.clone())? {
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into version_changed_events (
                id, resolver_id, block_number, transaction_id, version
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do nothing
            "#,
        )
        .bind(event.id)
        .bind(event.resolver_id)
        .bind(event.block_number)
        .bind(event.transaction_id)
        .bind(event.version)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
