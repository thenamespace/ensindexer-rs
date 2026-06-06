use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, postgres::PgRow};

use crate::{error::*, filters::*, inserts::*, models::*, query::*};

pub struct EventsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl EventsRepo<'_> {
    pub async fn insert_transfer(&self, event: TransferEventInsert) -> StorageResult<()> {
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

    pub async fn insert_name_registered(
        &self,
        event: NameRegisteredEventInsert,
    ) -> StorageResult<()> {
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

    pub async fn insert_wrapped_transfer(
        &self,
        event: WrappedTransferEventInsert,
    ) -> StorageResult<()> {
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

    pub async fn insert_addr_changed(&self, event: AddrChangedEventInsert) -> StorageResult<()> {
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

    pub async fn find_transfer_by_id(&self, id: &str) -> StorageResult<Option<TransferEventRow>> {
        self.find_event("transfer_events", transfer_event_columns(), id)
            .await
    }

    pub async fn list_transfers(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<TransferEventRow>> {
        self.list_events(
            "transfer_events",
            transfer_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_new_owner_by_id(&self, id: &str) -> StorageResult<Option<NewOwnerEventRow>> {
        self.find_event("new_owner_events", new_owner_event_columns(), id)
            .await
    }

    pub async fn list_new_owners(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NewOwnerEventRow>> {
        self.list_events(
            "new_owner_events",
            new_owner_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_new_resolver_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NewResolverEventRow>> {
        self.find_event("new_resolver_events", new_resolver_event_columns(), id)
            .await
    }

    pub async fn list_new_resolvers(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NewResolverEventRow>> {
        self.list_events(
            "new_resolver_events",
            new_resolver_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_new_ttl_by_id(&self, id: &str) -> StorageResult<Option<NewTtlEventRow>> {
        self.find_event("new_ttl_events", new_ttl_event_columns(), id)
            .await
    }

    pub async fn list_new_ttls(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NewTtlEventRow>> {
        self.list_events(
            "new_ttl_events",
            new_ttl_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_registered_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameRegisteredEventRow>> {
        self.find_event(
            "name_registered_events",
            name_registered_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_name_registered(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameRegisteredEventRow>> {
        self.list_events(
            "name_registered_events",
            name_registered_event_columns(),
            "registration_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_renewed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameRenewedEventRow>> {
        self.find_event("name_renewed_events", name_renewed_event_columns(), id)
            .await
    }

    pub async fn list_name_renewed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameRenewedEventRow>> {
        self.list_events(
            "name_renewed_events",
            name_renewed_event_columns(),
            "registration_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_transferred_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameTransferredEventRow>> {
        self.find_event(
            "name_transferred_events",
            name_transferred_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_name_transferred(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameTransferredEventRow>> {
        self.list_events(
            "name_transferred_events",
            name_transferred_event_columns(),
            "registration_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_wrapped_transfer_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<WrappedTransferEventRow>> {
        self.find_event(
            "wrapped_transfer_events",
            wrapped_transfer_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_wrapped_transfers(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<WrappedTransferEventRow>> {
        self.list_events(
            "wrapped_transfer_events",
            wrapped_transfer_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_wrapped_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameWrappedEventRow>> {
        self.find_event("name_wrapped_events", name_wrapped_event_columns(), id)
            .await
    }

    pub async fn list_name_wrapped(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameWrappedEventRow>> {
        self.list_events(
            "name_wrapped_events",
            name_wrapped_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_unwrapped_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameUnwrappedEventRow>> {
        self.find_event("name_unwrapped_events", name_unwrapped_event_columns(), id)
            .await
    }

    pub async fn list_name_unwrapped(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameUnwrappedEventRow>> {
        self.list_events(
            "name_unwrapped_events",
            name_unwrapped_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_fuses_set_by_id(&self, id: &str) -> StorageResult<Option<FusesSetEventRow>> {
        self.find_event("fuses_set_events", fuses_set_event_columns(), id)
            .await
    }

    pub async fn list_fuses_set(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<FusesSetEventRow>> {
        self.list_events(
            "fuses_set_events",
            fuses_set_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_expiry_extended_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<ExpiryExtendedEventRow>> {
        self.find_event(
            "expiry_extended_events",
            expiry_extended_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_expiry_extended(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<ExpiryExtendedEventRow>> {
        self.list_events(
            "expiry_extended_events",
            expiry_extended_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_addr_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<AddrChangedEventRow>> {
        self.find_event("addr_changed_events", addr_changed_event_columns(), id)
            .await
    }

    pub async fn list_addr_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AddrChangedEventRow>> {
        self.list_events(
            "addr_changed_events",
            addr_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_multicoin_addr_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<MulticoinAddrChangedEventRow>> {
        self.find_event(
            "multicoin_addr_changed_events",
            multicoin_addr_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_multicoin_addr_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<MulticoinAddrChangedEventRow>> {
        self.list_events(
            "multicoin_addr_changed_events",
            multicoin_addr_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameChangedEventRow>> {
        self.find_event("name_changed_events", name_changed_event_columns(), id)
            .await
    }

    pub async fn list_name_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameChangedEventRow>> {
        self.list_events(
            "name_changed_events",
            name_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_abi_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<AbiChangedEventRow>> {
        self.find_event("abi_changed_events", abi_changed_event_columns(), id)
            .await
    }

    pub async fn list_abi_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AbiChangedEventRow>> {
        self.list_events(
            "abi_changed_events",
            abi_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_pubkey_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<PubkeyChangedEventRow>> {
        self.find_event("pubkey_changed_events", pubkey_changed_event_columns(), id)
            .await
    }

    pub async fn list_pubkey_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<PubkeyChangedEventRow>> {
        self.list_events(
            "pubkey_changed_events",
            pubkey_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_text_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<TextChangedEventRow>> {
        self.find_event("text_changed_events", text_changed_event_columns(), id)
            .await
    }

    pub async fn list_text_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<TextChangedEventRow>> {
        self.list_events(
            "text_changed_events",
            text_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_contenthash_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<ContenthashChangedEventRow>> {
        self.find_event(
            "contenthash_changed_events",
            contenthash_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_contenthash_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<ContenthashChangedEventRow>> {
        self.list_events(
            "contenthash_changed_events",
            contenthash_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_interface_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<InterfaceChangedEventRow>> {
        self.find_event(
            "interface_changed_events",
            interface_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_interface_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<InterfaceChangedEventRow>> {
        self.list_events(
            "interface_changed_events",
            interface_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_authorisation_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<AuthorisationChangedEventRow>> {
        self.find_event(
            "authorisation_changed_events",
            authorisation_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_authorisation_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AuthorisationChangedEventRow>> {
        self.list_events(
            "authorisation_changed_events",
            authorisation_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_version_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<VersionChangedEventRow>> {
        self.find_event(
            "version_changed_events",
            version_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_version_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<VersionChangedEventRow>> {
        self.list_events(
            "version_changed_events",
            version_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_domain_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            domain_event_ref_union_sql(),
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_registration_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            registration_event_ref_union_sql(),
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_resolver_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            resolver_event_ref_union_sql(),
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    async fn find_event<T>(
        &self,
        table: &'static str,
        columns: &'static str,
        id: &str,
    ) -> StorageResult<Option<T>>
    where
        for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
    {
        let mut query = QueryBuilder::<Postgres>::new("select ");
        query
            .push(columns)
            .push(" from ")
            .push(table)
            .push(" where id = ")
            .push_bind(id);
        Ok(query.build_query_as().fetch_optional(self.pool).await?)
    }

    #[allow(clippy::too_many_arguments)]
    async fn list_events<T>(
        &self,
        table: &'static str,
        columns: &'static str,
        parent_column: &'static str,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<T>>
    where
        for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
    {
        let mut query = QueryBuilder::<Postgres>::new("select ");
        query.push(columns).push(" from ").push(table);

        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, parent_column, &filter);
        push_event_specific_filters(&mut separated, &mut has_where, table, &filter);
        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(event_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(" limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    async fn list_event_refs(
        &self,
        union_sql: &'static str,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "select kind, id, block_number, transaction_id, parent_id from (",
        );
        query.push(union_sql).push(") event_refs");

        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "parent_id", &filter);
        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(event_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id ")
            .push(direction.sql())
            .push(" limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }
}

fn push_event_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    parent_column: &'static str,
    filter: &EventFilter,
) {
    push_text_filter(separated, has_where, "id", filter.id.clone());
    push_text_not_filter(separated, has_where, "id", filter.id_not.clone());
    push_text_array_filter(separated, has_where, "id", filter.id_in.clone());
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in.clone());
    push_text_filter(
        separated,
        has_where,
        parent_column,
        filter.parent_id.clone(),
    );
    push_text_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id.clone(),
    );
    push_text_not_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_not.clone(),
    );
    push_text_array_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_in.clone(),
    );
    push_text_not_array_filter(
        separated,
        has_where,
        "transaction_id",
        filter.transaction_id_not_in.clone(),
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "=",
        filter.block_number,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        ">",
        filter.block_number_gt,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "<",
        filter.block_number_lt,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        ">=",
        filter.block_number_gte,
    );
    push_i32_filter(
        separated,
        has_where,
        "block_number",
        "<=",
        filter.block_number_lte,
    );
}

fn push_event_specific_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    filter: &EventFilter,
) {
    match table {
        "transfer_events" | "wrapped_transfer_events" | "name_unwrapped_events" => {
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
        }
        "new_owner_events" => {
            push_text_filter(
                separated,
                has_where,
                "parent_domain_id",
                filter.parent_domain_id.clone(),
            );
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
        }
        "new_resolver_events" => {
            push_text_filter(
                separated,
                has_where,
                "resolver_id",
                filter.resolver_id.clone(),
            );
        }
        "new_ttl_events" => {
            push_numeric_event_filter(separated, has_where, "ttl", filter, NumericEventField::Ttl);
        }
        "name_wrapped_events" => {
            push_text_filter(separated, has_where, "name", filter.name.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains.clone(),
                false,
            );
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains_nocase.clone(),
                true,
            );
            push_i32_event_filter(separated, has_where, "fuses", filter);
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "fuses_set_events" => {
            push_i32_event_filter(separated, has_where, "fuses", filter);
        }
        "expiry_extended_events" | "name_renewed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "name_registered_events" => {
            push_account_event_filter(
                separated,
                has_where,
                "registrant_id",
                filter.registrant_id.clone(),
            );
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "name_transferred_events" => {
            push_account_event_filter(
                separated,
                has_where,
                "new_owner_id",
                filter.new_owner_id.clone(),
            );
        }
        "addr_changed_events" => {
            push_account_event_filter(separated, has_where, "addr_id", filter.addr_id.clone());
        }
        "multicoin_addr_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "coin_type",
                filter,
                NumericEventField::CoinType,
            );
            push_text_filter(separated, has_where, "addr", filter.addr_id.clone());
        }
        "name_changed_events" => {
            push_text_filter(separated, has_where, "name", filter.name.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains.clone(),
                false,
            );
            push_text_contains_filter(
                separated,
                has_where,
                "name",
                filter.name_contains_nocase.clone(),
                true,
            );
        }
        "abi_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "content_type",
                filter,
                NumericEventField::ContentType,
            );
        }
        "pubkey_changed_events" => {
            push_text_filter(separated, has_where, "x", filter.x.clone());
            push_text_filter(separated, has_where, "y", filter.y.clone());
        }
        "text_changed_events" => {
            push_text_filter(separated, has_where, "key", filter.key.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "key",
                filter.key_contains.clone(),
                false,
            );
            push_text_filter(separated, has_where, "value", filter.value.clone());
            push_text_contains_filter(
                separated,
                has_where,
                "value",
                filter.value_contains.clone(),
                false,
            );
        }
        "contenthash_changed_events" => {
            push_text_filter(separated, has_where, "hash", filter.hash.clone());
        }
        "interface_changed_events" => {
            push_text_filter(
                separated,
                has_where,
                "interface_id",
                filter.interface_id.clone(),
            );
            push_text_filter(
                separated,
                has_where,
                "implementer",
                filter.implementer.clone(),
            );
        }
        "authorisation_changed_events" => {
            push_text_filter(separated, has_where, "owner", filter.owner_id.clone());
            push_text_filter(separated, has_where, "target", filter.target.clone());
            push_bool_filter(
                separated,
                has_where,
                "is_authorized",
                "=",
                filter.is_authorized,
            );
        }
        "version_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "version",
                filter,
                NumericEventField::Version,
            );
        }
        _ => {}
    }
}

fn push_account_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    push_text_filter(separated, has_where, column, value);
}

fn push_i32_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
) {
    push_i32_filter(separated, has_where, column, "=", filter.fuses);
    push_i32_filter(separated, has_where, column, ">", filter.fuses_gt);
    push_i32_filter(separated, has_where, column, "<", filter.fuses_lt);
    push_i32_filter(separated, has_where, column, ">=", filter.fuses_gte);
    push_i32_filter(separated, has_where, column, "<=", filter.fuses_lte);
}

enum NumericEventField {
    Ttl,
    ExpiryDate,
    CoinType,
    ContentType,
    Version,
}

fn push_numeric_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
    field: NumericEventField,
) {
    let (eq, gt, lt, gte, lte) = match field {
        NumericEventField::Ttl => (
            filter.ttl.clone(),
            filter.ttl_gt.clone(),
            filter.ttl_lt.clone(),
            filter.ttl_gte.clone(),
            filter.ttl_lte.clone(),
        ),
        NumericEventField::ExpiryDate => (
            filter.expiry_date.clone(),
            filter.expiry_date_gt.clone(),
            filter.expiry_date_lt.clone(),
            filter.expiry_date_gte.clone(),
            filter.expiry_date_lte.clone(),
        ),
        NumericEventField::CoinType => (
            filter.coin_type.clone(),
            filter.coin_type_gt.clone(),
            filter.coin_type_lt.clone(),
            None,
            None,
        ),
        NumericEventField::ContentType => (
            filter.content_type.clone(),
            filter.content_type_gt.clone(),
            filter.content_type_lt.clone(),
            None,
            None,
        ),
        NumericEventField::Version => (
            filter.version.clone(),
            filter.version_gt.clone(),
            filter.version_lt.clone(),
            None,
            None,
        ),
    };

    push_numeric_text_filter(separated, has_where, column, "=", eq);
    push_numeric_text_filter(separated, has_where, column, ">", gt);
    push_numeric_text_filter(separated, has_where, column, "<", lt);
    push_numeric_text_filter(separated, has_where, column, ">=", gte);
    push_numeric_text_filter(separated, has_where, column, "<=", lte);
}

fn domain_event_ref_union_sql() -> &'static str {
    r#"
    select 'Transfer' as kind, id, block_number, transaction_id, domain_id as parent_id from transfer_events
    union all select 'NewOwner' as kind, id, block_number, transaction_id, domain_id as parent_id from new_owner_events
    union all select 'NewResolver' as kind, id, block_number, transaction_id, domain_id as parent_id from new_resolver_events
    union all select 'NewTTL' as kind, id, block_number, transaction_id, domain_id as parent_id from new_ttl_events
    union all select 'WrappedTransfer' as kind, id, block_number, transaction_id, domain_id as parent_id from wrapped_transfer_events
    union all select 'NameWrapped' as kind, id, block_number, transaction_id, domain_id as parent_id from name_wrapped_events
    union all select 'NameUnwrapped' as kind, id, block_number, transaction_id, domain_id as parent_id from name_unwrapped_events
    union all select 'FusesSet' as kind, id, block_number, transaction_id, domain_id as parent_id from fuses_set_events
    union all select 'ExpiryExtended' as kind, id, block_number, transaction_id, domain_id as parent_id from expiry_extended_events
    "#
}

fn registration_event_ref_union_sql() -> &'static str {
    r#"
    select 'NameRegistered' as kind, id, block_number, transaction_id, registration_id as parent_id from name_registered_events
    union all select 'NameRenewed' as kind, id, block_number, transaction_id, registration_id as parent_id from name_renewed_events
    union all select 'NameTransferred' as kind, id, block_number, transaction_id, registration_id as parent_id from name_transferred_events
    "#
}

fn resolver_event_ref_union_sql() -> &'static str {
    r#"
    select 'AddrChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from addr_changed_events
    union all select 'MulticoinAddrChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from multicoin_addr_changed_events
    union all select 'NameChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from name_changed_events
    union all select 'AbiChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from abi_changed_events
    union all select 'PubkeyChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from pubkey_changed_events
    union all select 'TextChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from text_changed_events
    union all select 'ContenthashChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from contenthash_changed_events
    union all select 'InterfaceChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from interface_changed_events
    union all select 'AuthorisationChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from authorisation_changed_events
    union all select 'VersionChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from version_changed_events
    "#
}

fn transfer_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, owner_id"
}

fn new_owner_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, parent_domain_id, owner_id"
}

fn new_resolver_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, resolver_id"
}

fn new_ttl_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, ttl"
}

fn wrapped_transfer_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, owner_id"
}

fn name_wrapped_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, name, fuses, owner_id, expiry_date"
}

fn name_unwrapped_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, owner_id"
}

fn fuses_set_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, fuses"
}

fn expiry_extended_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, expiry_date"
}

fn name_registered_event_columns() -> &'static str {
    "id, registration_id, block_number, transaction_id, registrant_id, expiry_date"
}

fn name_renewed_event_columns() -> &'static str {
    "id, registration_id, block_number, transaction_id, expiry_date"
}

fn name_transferred_event_columns() -> &'static str {
    "id, registration_id, block_number, transaction_id, new_owner_id"
}

fn addr_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, addr_id"
}

fn multicoin_addr_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, coin_type, addr"
}

fn name_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, name"
}

fn abi_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, content_type"
}

fn pubkey_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, x, y"
}

fn text_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, key, value"
}

fn contenthash_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, hash"
}

fn interface_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, interface_id, implementer"
}

fn authorisation_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, owner, target, is_authorized"
}

fn version_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, version"
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::*;

    #[test]
    fn event_specific_filters_are_added_only_for_matching_table_columns() {
        let filter = EventFilter {
            parent_id: Some("0xdomain".into()),
            owner_id: Some("0xowner".into()),
            fuses_gte: Some(32),
            expiry_date_lt: Some("1000".into()),
            ..EventFilter::default()
        };
        let mut query = QueryBuilder::<Postgres>::new("select id from name_wrapped_events");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_event_filters(&mut separated, &mut has_where, "domain_id", &filter);
            push_event_specific_filters(
                &mut separated,
                &mut has_where,
                "name_wrapped_events",
                &filter,
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from name_wrapped_events where domain_id = $1 and fuses >= $2 and owner_id = $3 and expiry_date < $4::numeric "
        );
    }
}
