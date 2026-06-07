use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{EventsRepo, Storage, StorageError, StorageResult, inserts::*};

const EVENT_INSERT_CHUNK_ROWS: usize = 5_000;

#[derive(Debug, Default)]
pub struct EventBuffer {
    pub transfer: Vec<TransferEventInsert>,
    pub new_owner: Vec<NewOwnerEventInsert>,
    pub new_resolver: Vec<NewResolverEventInsert>,
    pub new_ttl: Vec<NewTtlEventInsert>,
    pub name_registered: Vec<NameRegisteredEventInsert>,
    pub name_renewed: Vec<NameRenewedEventInsert>,
    pub name_transferred: Vec<NameTransferredEventInsert>,
    pub wrapped_transfer: Vec<WrappedTransferEventInsert>,
    pub name_wrapped: Vec<NameWrappedEventInsert>,
    pub name_unwrapped: Vec<NameUnwrappedEventInsert>,
    pub fuses_set: Vec<FusesSetEventInsert>,
    pub expiry_extended: Vec<ExpiryExtendedEventInsert>,
    pub addr_changed: Vec<AddrChangedEventInsert>,
    pub multicoin_addr_changed: Vec<MulticoinAddrChangedEventInsert>,
    pub name_changed: Vec<NameChangedEventInsert>,
    pub abi_changed: Vec<AbiChangedEventInsert>,
    pub pubkey_changed: Vec<PubkeyChangedEventInsert>,
    pub text_changed: Vec<TextChangedEventInsert>,
    pub contenthash_changed: Vec<ContenthashChangedEventInsert>,
    pub interface_changed: Vec<InterfaceChangedEventInsert>,
    pub authorisation_changed: Vec<AuthorisationChangedEventInsert>,
    pub version_changed: Vec<VersionChangedEventInsert>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EventBufferFlushStats {
    pub rows: usize,
}

macro_rules! bulk_insert_events {
    ($pool:expr, $rows:expr, $sql:literal, |$b:ident, $event:ident| $body:block) => {{
        let mut inserted = 0;
        for chunk in $rows.chunks(EVENT_INSERT_CHUNK_ROWS) {
            let mut query = QueryBuilder::<Postgres>::new($sql);
            query.push_values(chunk, |mut $b, $event| $body);
            query.push(" on conflict (id) do nothing");
            query.build().execute($pool).await?;
            inserted += chunk.len();
        }
        inserted
    }};
}

impl EventBuffer {
    fn len(&self) -> usize {
        self.transfer.len()
            + self.new_owner.len()
            + self.new_resolver.len()
            + self.new_ttl.len()
            + self.name_registered.len()
            + self.name_renewed.len()
            + self.name_transferred.len()
            + self.wrapped_transfer.len()
            + self.name_wrapped.len()
            + self.name_unwrapped.len()
            + self.fuses_set.len()
            + self.expiry_extended.len()
            + self.addr_changed.len()
            + self.multicoin_addr_changed.len()
            + self.name_changed.len()
            + self.abi_changed.len()
            + self.pubkey_changed.len()
            + self.text_changed.len()
            + self.contenthash_changed.len()
            + self.interface_changed.len()
            + self.authorisation_changed.len()
            + self.version_changed.len()
    }

    pub async fn flush(self, pool: &PgPool) -> StorageResult<EventBufferFlushStats> {
        let rows = self.len();
        insert_domain_events(pool, &self).await?;
        insert_registration_events(pool, &self).await?;
        insert_resolver_events(pool, &self).await?;
        Ok(EventBufferFlushStats { rows })
    }
}

impl Storage {
    pub fn begin_event_buffer(&self) -> StorageResult<()> {
        let mut buffer = self
            .event_buffer
            .lock()
            .map_err(|_| StorageError::EventBufferPoisoned)?;
        if buffer.is_some() {
            return Err(StorageError::EventBufferAlreadyActive);
        }
        *buffer = Some(EventBuffer::default());
        Ok(())
    }

    pub fn clear_event_buffer(&self) -> StorageResult<()> {
        let mut buffer = self
            .event_buffer
            .lock()
            .map_err(|_| StorageError::EventBufferPoisoned)?;
        *buffer = None;
        Ok(())
    }

    pub async fn flush_event_buffer(&self) -> StorageResult<EventBufferFlushStats> {
        let buffer = {
            let mut guard = self
                .event_buffer
                .lock()
                .map_err(|_| StorageError::EventBufferPoisoned)?;
            guard.take().ok_or(StorageError::EventBufferNotActive)?
        };
        let stats = buffer.flush(self.pool()).await?;

        let mut guard = self
            .event_buffer
            .lock()
            .map_err(|_| StorageError::EventBufferPoisoned)?;
        *guard = Some(EventBuffer::default());
        Ok(stats)
    }
}

macro_rules! buffer_event {
    ($name:ident, $event:ty, $field:ident) => {
        pub(crate) fn $name(&self, event: $event) -> StorageResult<bool> {
            let mut buffer = self
                .event_buffer
                .lock()
                .map_err(|_| StorageError::EventBufferPoisoned)?;
            let Some(active) = buffer.as_mut() else {
                return Ok(false);
            };
            active.$field.push(event);
            Ok(true)
        }
    };
}

impl EventsRepo<'_> {
    buffer_event!(buffer_transfer, TransferEventInsert, transfer);
    buffer_event!(buffer_new_owner, NewOwnerEventInsert, new_owner);
    buffer_event!(buffer_new_resolver, NewResolverEventInsert, new_resolver);
    buffer_event!(buffer_new_ttl, NewTtlEventInsert, new_ttl);
    buffer_event!(
        buffer_name_registered,
        NameRegisteredEventInsert,
        name_registered
    );
    buffer_event!(buffer_name_renewed, NameRenewedEventInsert, name_renewed);
    buffer_event!(
        buffer_name_transferred,
        NameTransferredEventInsert,
        name_transferred
    );
    buffer_event!(
        buffer_wrapped_transfer,
        WrappedTransferEventInsert,
        wrapped_transfer
    );
    buffer_event!(buffer_name_wrapped, NameWrappedEventInsert, name_wrapped);
    buffer_event!(
        buffer_name_unwrapped,
        NameUnwrappedEventInsert,
        name_unwrapped
    );
    buffer_event!(buffer_fuses_set, FusesSetEventInsert, fuses_set);
    buffer_event!(
        buffer_expiry_extended,
        ExpiryExtendedEventInsert,
        expiry_extended
    );
    buffer_event!(buffer_addr_changed, AddrChangedEventInsert, addr_changed);
    buffer_event!(
        buffer_multicoin_addr_changed,
        MulticoinAddrChangedEventInsert,
        multicoin_addr_changed
    );
    buffer_event!(buffer_name_changed, NameChangedEventInsert, name_changed);
    buffer_event!(buffer_abi_changed, AbiChangedEventInsert, abi_changed);
    buffer_event!(
        buffer_pubkey_changed,
        PubkeyChangedEventInsert,
        pubkey_changed
    );
    buffer_event!(buffer_text_changed, TextChangedEventInsert, text_changed);
    buffer_event!(
        buffer_contenthash_changed,
        ContenthashChangedEventInsert,
        contenthash_changed
    );
    buffer_event!(
        buffer_interface_changed,
        InterfaceChangedEventInsert,
        interface_changed
    );
    buffer_event!(
        buffer_authorisation_changed,
        AuthorisationChangedEventInsert,
        authorisation_changed
    );
    buffer_event!(
        buffer_version_changed,
        VersionChangedEventInsert,
        version_changed
    );
}

async fn insert_domain_events(pool: &PgPool, buffer: &EventBuffer) -> StorageResult<()> {
    bulk_insert_events!(
        pool,
        &buffer.transfer,
        "insert into transfer_events (id, domain_id, block_number, transaction_id, owner_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.owner_id);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.new_owner,
        "insert into new_owner_events (id, domain_id, block_number, transaction_id, parent_domain_id, owner_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.parent_domain_id)
                .push_bind(&event.owner_id);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.new_resolver,
        "insert into new_resolver_events (id, domain_id, block_number, transaction_id, resolver_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.resolver_id);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.new_ttl,
        "insert into new_ttl_events (id, domain_id, block_number, transaction_id, ttl) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.ttl);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.wrapped_transfer,
        "insert into wrapped_transfer_events (id, domain_id, block_number, transaction_id, owner_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.owner_id);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.name_wrapped,
        "insert into name_wrapped_events (id, domain_id, block_number, transaction_id, name, fuses, owner_id, expiry_date) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.name)
                .push_bind(event.fuses)
                .push_bind(&event.owner_id)
                .push_bind(&event.expiry_date);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.name_unwrapped,
        "insert into name_unwrapped_events (id, domain_id, block_number, transaction_id, owner_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.owner_id);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.fuses_set,
        "insert into fuses_set_events (id, domain_id, block_number, transaction_id, fuses) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(event.fuses);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.expiry_extended,
        "insert into expiry_extended_events (id, domain_id, block_number, transaction_id, expiry_date) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.domain_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.expiry_date);
        }
    );
    Ok(())
}

async fn insert_registration_events(pool: &PgPool, buffer: &EventBuffer) -> StorageResult<()> {
    bulk_insert_events!(
        pool,
        &buffer.name_registered,
        "insert into name_registered_events (id, registration_id, block_number, transaction_id, registrant_id, expiry_date) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.registration_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.registrant_id)
                .push_bind(&event.expiry_date);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.name_renewed,
        "insert into name_renewed_events (id, registration_id, block_number, transaction_id, expiry_date) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.registration_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.expiry_date);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.name_transferred,
        "insert into name_transferred_events (id, registration_id, block_number, transaction_id, new_owner_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.registration_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.new_owner_id);
        }
    );
    Ok(())
}

async fn insert_resolver_events(pool: &PgPool, buffer: &EventBuffer) -> StorageResult<()> {
    bulk_insert_events!(
        pool,
        &buffer.addr_changed,
        "insert into addr_changed_events (id, resolver_id, block_number, transaction_id, addr_id) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.addr_id);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.multicoin_addr_changed,
        "insert into multicoin_addr_changed_events (id, resolver_id, block_number, transaction_id, coin_type, addr) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.coin_type)
                .push_bind(&event.addr);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.name_changed,
        "insert into name_changed_events (id, resolver_id, block_number, transaction_id, name) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.name);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.abi_changed,
        "insert into abi_changed_events (id, resolver_id, block_number, transaction_id, content_type) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.content_type);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.pubkey_changed,
        "insert into pubkey_changed_events (id, resolver_id, block_number, transaction_id, x, y) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.x)
                .push_bind(&event.y);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.text_changed,
        "insert into text_changed_events (id, resolver_id, block_number, transaction_id, key, value) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.key)
                .push_bind(&event.value);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.contenthash_changed,
        "insert into contenthash_changed_events (id, resolver_id, block_number, transaction_id, hash) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.hash);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.interface_changed,
        "insert into interface_changed_events (id, resolver_id, block_number, transaction_id, interface_id, implementer) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.interface_id)
                .push_bind(&event.implementer);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.authorisation_changed,
        "insert into authorisation_changed_events (id, resolver_id, block_number, transaction_id, owner, target, is_authorized) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.owner)
                .push_bind(&event.target)
                .push_bind(event.is_authorized);
        }
    );
    bulk_insert_events!(
        pool,
        &buffer.version_changed,
        "insert into version_changed_events (id, resolver_id, block_number, transaction_id, version) ",
        |b, event| {
            b.push_bind(&event.id)
                .push_bind(&event.resolver_id)
                .push_bind(event.block_number)
                .push_bind(&event.transaction_id)
                .push_bind(&event.version);
        }
    );
    Ok(())
}
