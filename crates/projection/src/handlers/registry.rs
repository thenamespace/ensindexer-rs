use alloy_primitives::{Address, B256, U256};
use storage::{
    NewOwnerEventInsert, NewResolverEventInsert, NewTtlEventInsert, Storage, TransferEventInsert,
};
use types::{
    DomainId, LabelHash, RegistrySource, ResolverId,
    constants::{EMPTY_ADDRESS, ROOT_NODE},
    hex_address, make_subnode,
};

use crate::{
    ProjectionResult,
    support::{
        block_number, decimal_from_u256, ensure_account, ensure_domain, known_label,
        label_from_hash, mark_domain_changed, mark_resolver_changed,
    },
};

pub(crate) async fn registry_transfer(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    owner: Address,
    source: RegistrySource,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    if should_skip_old_registry(storage, &domain_id, source).await? {
        return Ok(());
    }

    let owner_id = ensure_account(storage, owner, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        false,
        block_number(ctx)?,
    )
    .await?;
    storage.domains().set_owner(&domain_id, &owner_id).await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_transfer(TransferEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            owner_id,
        })
        .await?;

    Ok(())
}

pub(crate) async fn registry_new_owner(
    storage: &Storage,
    ctx: &types::LogContext,
    parent_node: B256,
    labelhash: B256,
    owner: Address,
    source: RegistrySource,
) -> ProjectionResult<()> {
    let is_migrated = matches!(source, RegistrySource::Current);
    let node = make_subnode(parent_node, labelhash);
    let domain_id = DomainId(node).as_subgraph_id();

    if should_skip_old_registry(storage, &domain_id, source).await? {
        return Ok(());
    }

    let parent_id = DomainId(parent_node).as_subgraph_id();
    let owner_id = ensure_account(storage, owner, block_number(ctx)?).await?;
    ensure_domain(
        storage,
        &parent_id,
        ctx.block_timestamp,
        false,
        block_number(ctx)?,
    )
    .await?;

    let existing = storage.domains().find_by_id(&domain_id).await?;
    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        is_migrated,
        block_number(ctx)?,
    )
    .await?;

    if existing
        .as_ref()
        .and_then(|domain| domain.parent_id.as_ref())
        .is_none()
    {
        storage
            .domains()
            .increment_subdomain_count(&parent_id)
            .await?;
        mark_domain_changed(storage, &parent_id, block_number(ctx)?).await?;
    }

    let known_label = known_label(labelhash);
    if existing
        .as_ref()
        .and_then(|domain| domain.name.as_ref())
        .is_none()
        || known_label.is_some()
    {
        let label = label_from_hash(labelhash);
        let parent = storage.domains().find_by_id(&parent_id).await?;
        let name = if parent_id == ROOT_NODE {
            Some(label.clone())
        } else {
            parent
                .and_then(|parent| parent.name)
                .map(|parent_name| format!("{label}.{parent_name}"))
        };
        if let Some(known_label) = known_label {
            storage
                .domains()
                .set_name(&domain_id, Some(known_label), name.as_deref())
                .await?;
        } else {
            storage
                .domains()
                .set_name_if_unknown(&domain_id, None, name.as_deref())
                .await?;
        }
    }

    storage
        .domains()
        .set_parent_and_label(
            &domain_id,
            &parent_id,
            &LabelHash(labelhash).as_subgraph_id(),
            is_migrated,
        )
        .await?;
    storage.domains().set_owner(&domain_id, &owner_id).await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_new_owner(NewOwnerEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            parent_domain_id: parent_id,
            owner_id,
        })
        .await?;

    Ok(())
}

pub(crate) async fn registry_new_resolver(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    resolver: Address,
    source: RegistrySource,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    if should_skip_old_registry(storage, &domain_id, source).await? {
        return Ok(());
    }

    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        false,
        block_number(ctx)?,
    )
    .await?;

    let (resolver_id, resolved_address_id, event_resolver_id) = if resolver == Address::ZERO {
        (None, None, EMPTY_ADDRESS.to_owned())
    } else {
        let resolver_id = ResolverId {
            address: resolver,
            node,
        }
        .as_subgraph_id();
        if storage
            .resolvers()
            .create_if_missing(&resolver_id, &domain_id, &hex_address(resolver))
            .await?
        {
            mark_resolver_changed(storage, &resolver_id, block_number(ctx)?).await?;
        }
        let resolved_address_id = storage
            .resolvers()
            .find_by_id(&resolver_id)
            .await?
            .and_then(|resolver| resolver.addr_id);
        (Some(resolver_id.clone()), resolved_address_id, resolver_id)
    };

    storage
        .domains()
        .set_resolver(
            &domain_id,
            resolver_id.as_deref(),
            resolved_address_id.as_deref(),
        )
        .await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_new_resolver(NewResolverEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            resolver_id: event_resolver_id,
        })
        .await?;

    Ok(())
}

pub(crate) async fn registry_new_ttl(
    storage: &Storage,
    ctx: &types::LogContext,
    node: B256,
    ttl: U256,
    source: RegistrySource,
) -> ProjectionResult<()> {
    let domain_id = DomainId(node).as_subgraph_id();
    if should_skip_old_registry(storage, &domain_id, source).await? {
        return Ok(());
    }

    ensure_domain(
        storage,
        &domain_id,
        ctx.block_timestamp,
        false,
        block_number(ctx)?,
    )
    .await?;
    let ttl = decimal_from_u256(ttl)?;
    storage.domains().set_ttl(&domain_id, ttl.clone()).await?;
    mark_domain_changed(storage, &domain_id, block_number(ctx)?).await?;
    storage
        .events()
        .insert_new_ttl(NewTtlEventInsert {
            id: ctx.event_id(),
            domain_id,
            block_number: block_number(ctx)?,
            transaction_id: types::hex_b256(ctx.transaction_hash),
            ttl,
        })
        .await?;

    Ok(())
}

async fn should_skip_old_registry(
    storage: &Storage,
    domain_id: &str,
    source: RegistrySource,
) -> ProjectionResult<bool> {
    if !matches!(source, RegistrySource::Old) {
        return Ok(false);
    }

    Ok(storage
        .domains()
        .find_by_id(domain_id)
        .await?
        .is_some_and(|domain| domain.is_migrated))
}
