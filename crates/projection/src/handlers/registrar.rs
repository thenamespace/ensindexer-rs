use alloy_primitives::{Address, B256, U256};
use storage::{
    NameRegisteredEventInsert, NameRenewedEventInsert, NameTransferredEventInsert, Storage,
};
use types::{LabelHash, validate_label};

use crate::{
    ProjectionResult,
    support::{
        block_number, decimal_from_i64, decimal_from_u256, ensure_account, ensure_domain,
        ensure_eth_parent, eth_2ld_domain_id, expiry_with_grace, known_label, mark_domain_changed,
        mark_registration_changed,
    },
};

pub(crate) async fn base_name_registered(
    storage: &Storage,
    ctx: &types::LogContext,
    labelhash: B256,
    owner: Address,
    expires: U256,
) -> ProjectionResult<()> {
    let label = LabelHash(labelhash);
    let registration_id = label.as_subgraph_id();
    let domain_id = eth_2ld_domain_id(labelhash)?;
    let owner_id = ensure_account(storage, owner, block_number(ctx)?).await?;

    ensure_eth_parent(storage, ctx.block_timestamp, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;

    let expiry = decimal_from_u256(expires)?;
    storage
        .registrations()
        .upsert_registered(
            &registration_id,
            &domain_id,
            decimal_from_i64(ctx.block_timestamp)?,
            expiry.clone(),
            &owner_id,
        )
        .await?;
    mark_registration_changed(storage, &registration_id, block_number(ctx)?).await?;
    if let Some(label) = known_label(labelhash) {
        let name = format!("{label}.eth");
        storage
            .domains()
            .set_name(&domain_id, Some(label), Some(&name))
            .await?;
        storage
            .registrations()
            .set_label_name(&registration_id, label)
            .await?;
    }
    storage
        .domains()
        .set_registrant_and_expiry(&domain_id, &owner_id, expiry_with_grace(expires)?)
        .await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_name_registered(NameRegisteredEventInsert {
            id: ctx.event_id(),
            registration_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            registrant_id: owner_id,
            expiry_date: expiry,
        })
        .await?;

    Ok(())
}

pub(crate) async fn base_name_renewed(
    storage: &Storage,
    ctx: &types::LogContext,
    labelhash: B256,
    expires: U256,
) -> ProjectionResult<()> {
    let registration_id = LabelHash(labelhash).as_subgraph_id();
    let Some(registration) = storage.registrations().find_by_id(&registration_id).await? else {
        return Ok(());
    };

    let expiry = decimal_from_u256(expires)?;
    storage
        .registrations()
        .set_expiry(&registration_id, expiry.clone())
        .await?;
    mark_registration_changed(storage, &registration_id, block_number(ctx)?).await?;
    storage
        .domains()
        .set_registrant_and_expiry(
            &registration.domain_id,
            &registration.registrant_id,
            expiry_with_grace(expires)?,
        )
        .await?;
    mark_domain_changed(storage, &registration.domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_name_renewed(NameRenewedEventInsert {
            id: ctx.event_id(),
            registration_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            expiry_date: expiry,
        })
        .await?;

    Ok(())
}

pub(crate) async fn base_name_transferred(
    storage: &Storage,
    ctx: &types::LogContext,
    labelhash: B256,
    new_owner: Address,
) -> ProjectionResult<()> {
    let registration_id = LabelHash(labelhash).as_subgraph_id();
    let Some(registration) = storage.registrations().find_by_id(&registration_id).await? else {
        return Ok(());
    };

    let owner_id = ensure_account(storage, new_owner, block_number(ctx)?).await?;
    storage
        .registrations()
        .set_registrant(&registration_id, &owner_id)
        .await?;
    mark_registration_changed(storage, &registration_id, block_number(ctx)?).await?;
    storage
        .domains()
        .set_registrant(&registration.domain_id, &owner_id)
        .await?;
    mark_domain_changed(storage, &registration.domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_name_transferred(NameTransferredEventInsert {
            id: ctx.event_id(),
            registration_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            new_owner_id: owner_id,
        })
        .await?;

    Ok(())
}

pub(crate) async fn controller_name_preimage(
    storage: &Storage,
    ctx: &types::LogContext,
    label: String,
    labelhash: B256,
    cost: U256,
) -> ProjectionResult<()> {
    if validate_label(&label).is_err() {
        return Ok(());
    }

    let registration_id = LabelHash(labelhash).as_subgraph_id();
    let domain_id = eth_2ld_domain_id(labelhash)?;
    let name = format!("{label}.eth");

    ensure_eth_parent(storage, ctx.block_timestamp, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        true,
        block_number(ctx)?,
    )
    .await?;
    storage
        .domains()
        .set_name(&domain_id, Some(&label), Some(&name))
        .await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;

    if storage
        .registrations()
        .find_by_id(&registration_id)
        .await?
        .is_some()
    {
        storage
            .registrations()
            .set_preimage(&registration_id, &label, decimal_from_u256(cost)?)
            .await?;
        mark_registration_changed(storage, &registration_id, block_number(ctx)?).await?;
    }

    Ok(())
}
