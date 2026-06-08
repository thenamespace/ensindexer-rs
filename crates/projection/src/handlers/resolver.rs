use alloy_primitives::{Address, B256, FixedBytes, U256};
use storage::{
    AbiChangedEventInsert, AddrChangedEventInsert, AuthorisationChangedEventInsert,
    ContenthashChangedEventInsert, InterfaceChangedEventInsert, MulticoinAddrChangedEventInsert,
    NameChangedEventInsert, PubkeyChangedEventInsert, Storage, TextChangedEventInsert,
    VersionChangedEventInsert,
};
use types::{DomainId, hex_address, hex_bytes};

use crate::{
    ProjectionResult,
    support::{
        block_number, contains_postgres_null, decimal_from_u256, ensure_account, ensure_resolver,
        mark_domain_changed, mark_resolver_changed,
    },
};

pub(crate) async fn resolver_addr_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    addr: Address,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    let addr_id = ensure_account(storage, addr, block_number(ctx)?).await?;

    storage.resolvers().set_addr(&resolver_id, &addr_id).await?;
    mark_resolver_changed(storage, &resolver_id, block_number(ctx)?).await?;

    if storage
        .domains()
        .find_by_id(&domain_id)
        .await?
        .is_some_and(|domain| domain.resolver_id.as_deref() == Some(resolver_id.as_str()))
    {
        storage
            .domains()
            .set_resolver(&domain_id, Some(&resolver_id), Some(&addr_id))
            .await?;
        mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    }

    storage
        .events()
        .insert_addr_changed(AddrChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            addr_id,
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_multicoin_addr_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    coin_type: U256,
    addr: Vec<u8>,
) -> ProjectionResult<()> {
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    let coin_type = decimal_from_u256(coin_type)?;
    storage
        .resolvers()
        .add_coin_type(&resolver_id, coin_type.clone())
        .await?;
    mark_resolver_changed(storage, &resolver_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_multicoin_addr_changed(MulticoinAddrChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            coin_type,
            addr: hex_bytes(&addr),
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_name_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    name: String,
) -> ProjectionResult<()> {
    if contains_postgres_null(&name) {
        return Ok(());
    }

    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    storage
        .events()
        .insert_name_changed(NameChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            name,
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_abi_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    content_type: U256,
) -> ProjectionResult<()> {
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    storage
        .events()
        .insert_abi_changed(AbiChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            content_type: decimal_from_u256(content_type)?,
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_pubkey_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    x: B256,
    y: B256,
) -> ProjectionResult<()> {
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    storage
        .events()
        .insert_pubkey_changed(PubkeyChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            x: types::hex_b256(x),
            y: types::hex_b256(y),
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_text_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    key: String,
    value: Option<String>,
) -> ProjectionResult<()> {
    if contains_postgres_null(&key) {
        return Ok(());
    }
    let value = value.filter(|value| !contains_postgres_null(value));
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    storage.resolvers().add_text(&resolver_id, &key).await?;
    mark_resolver_changed(storage, &resolver_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_text_changed(TextChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            key,
            value,
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_contenthash_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    hash: Vec<u8>,
) -> ProjectionResult<()> {
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    let hash = hex_bytes(&hash);
    storage
        .resolvers()
        .set_content_hash(&resolver_id, &hash)
        .await?;
    mark_resolver_changed(storage, &resolver_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_contenthash_changed(ContenthashChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            hash,
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_interface_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    interface_id: FixedBytes<4>,
    implementer: Address,
) -> ProjectionResult<()> {
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    storage
        .events()
        .insert_interface_changed(InterfaceChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            interface_id: hex_bytes(interface_id.as_slice()),
            implementer: hex_address(implementer),
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_authorisation_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    owner: Address,
    target: Address,
    is_authorized: bool,
) -> ProjectionResult<()> {
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;
    storage
        .events()
        .insert_authorisation_changed(AuthorisationChangedEventInsert {
            id: ctx.event_id(),
            resolver_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            owner: hex_address(owner),
            target: hex_address(target),
            is_authorized,
        })
        .await?;
    Ok(())
}

pub(crate) async fn resolver_version_changed(
    storage: &Storage,
    ctx: &types::LogContext,
    resolver: Address,
    node: B256,
    version: U256,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    let resolver_id = ensure_resolver(
        storage,
        resolver,
        node,
        ctx.block_timestamp,
        block_number(ctx)?,
    )
    .await?;

    storage
        .events()
        .insert_version_changed(VersionChangedEventInsert {
            id: ctx.event_id(),
            resolver_id: resolver_id.clone(),
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            version: decimal_from_u256(version)?,
        })
        .await?;

    if storage
        .domains()
        .find_by_id(&domain_id)
        .await?
        .is_some_and(|domain| domain.resolver_id.as_deref() == Some(resolver_id.as_str()))
    {
        storage
            .domains()
            .set_resolver(&domain_id, Some(&resolver_id), None)
            .await?;
        mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    }

    storage.resolvers().reset_records(&resolver_id).await?;
    mark_resolver_changed(storage, &resolver_id, block_number(ctx)?).await?;
    Ok(())
}
