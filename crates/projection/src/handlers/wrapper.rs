use alloy_primitives::{Address, B256, U256};
use storage::{
    ExpiryExtendedEventInsert, FusesSetEventInsert, NameUnwrappedEventInsert,
    NameWrappedEventInsert, Storage, WrappedTransferEventInsert,
};
use types::{DomainId, constants::ETH_NODE};

use crate::{
    ProjectionResult,
    support::{
        block_number, check_pcc_burned, decimal_from_u256, decode_wrapped_name, ensure_account,
        ensure_domain, fuses_to_i32, mark_domain_changed, mark_wrapped_domain_changed,
    },
};

pub(crate) async fn name_wrapped(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    dns_name: Vec<u8>,
    owner: Address,
    fuses: u32,
    expiry: U256,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    let owner_id = ensure_account(storage, owner, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;

    let decoded = decode_wrapped_name(&dns_name);
    if let Some((label, name)) = decoded.as_ref() {
        storage
            .domains()
            .set_name_if_unknown(&domain_id, Some(label), Some(name))
            .await?;
    }

    let fuses = fuses_to_i32(fuses);
    let expiry = decimal_from_u256(expiry)?;
    if check_pcc_burned(fuses) {
        storage
            .domains()
            .set_expiry_if_newer(&domain_id, expiry.clone())
            .await?;
    }
    storage
        .domains()
        .set_wrapped_owner(&domain_id, &owner_id)
        .await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .wrapped_domains()
        .upsert_full(
            &domain_id,
            &domain_id,
            expiry.clone(),
            fuses,
            &owner_id,
            decoded.as_ref().map(|(_, name)| name.as_str()),
        )
        .await?;
    mark_wrapped_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_name_wrapped(NameWrappedEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            name: decoded.map(|(_, name)| name),
            fuses,
            owner_id,
            expiry_date: expiry,
        })
        .await?;

    Ok(())
}

pub(crate) async fn name_unwrapped(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    owner: Address,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    let owner_id = ensure_account(storage, owner, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;

    if let Some(domain) = storage.domains().find_by_id(&domain_id).await? {
        storage.domains().clear_wrapped_owner(&domain_id).await?;
        if domain.expiry_date.is_some() && domain.parent_id.as_deref() != Some(ETH_NODE) {
            storage.domains().clear_expiry(&domain_id).await?;
        }
        mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    }

    storage.wrapped_domains().delete(&domain_id).await?;
    storage
        .events()
        .insert_name_unwrapped(NameUnwrappedEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            owner_id,
        })
        .await?;

    Ok(())
}

pub(crate) async fn fuses_set(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    fuses: u32,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;
    let fuses = fuses_to_i32(fuses);

    if let Some(wrapped) = storage.wrapped_domains().find_by_id(&domain_id).await? {
        storage
            .wrapped_domains()
            .set_fuses(&domain_id, fuses)
            .await?;
        mark_wrapped_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
        if check_pcc_burned(fuses) {
            storage
                .domains()
                .set_expiry_if_newer(&domain_id, wrapped.expiry_date)
                .await?;
            mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
        }
    }

    storage
        .events()
        .insert_fuses_set(FusesSetEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            fuses,
        })
        .await?;

    Ok(())
}

pub(crate) async fn expiry_extended(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    expiry: U256,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;
    let expiry = decimal_from_u256(expiry)?;

    if let Some(wrapped) = storage.wrapped_domains().find_by_id(&domain_id).await? {
        storage
            .wrapped_domains()
            .set_expiry(&domain_id, expiry.clone())
            .await?;
        mark_wrapped_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
        if check_pcc_burned(wrapped.fuses) {
            storage
                .domains()
                .set_expiry_if_newer(&domain_id, expiry.clone())
                .await?;
            mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
        }
    }

    storage
        .events()
        .insert_expiry_extended(ExpiryExtendedEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            expiry_date: expiry,
        })
        .await?;

    Ok(())
}

pub(crate) async fn wrapped_transfer(
    storage: &Storage,
    ctx: &types::LogContext,
    token_id: U256,
    to: Address,
    index: usize,
) -> ProjectionResult<()> {
    let node = B256::from(token_id.to_be_bytes());
    let domain_id = DomainId(node).as_subgraph_id();
    let owner_id = ensure_account(storage, to, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;

    storage
        .wrapped_domains()
        .upsert_transfer_placeholder(&domain_id, &domain_id, &owner_id)
        .await?;
    mark_wrapped_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .domains()
        .set_wrapped_owner(&domain_id, &owner_id)
        .await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_wrapped_transfer(WrappedTransferEventInsert {
            id: ctx.batch_event_id(index),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            owner_id,
        })
        .await?;

    Ok(())
}
