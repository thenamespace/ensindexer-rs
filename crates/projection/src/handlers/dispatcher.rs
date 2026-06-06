use contracts::{EnsEvent, IndexedEvent};
use storage::Storage;

use crate::{
    ProjectionResult,
    handlers::{registrar::*, registry::*, resolver::*, wrapper::*},
};

pub async fn apply_event(storage: &Storage, indexed: IndexedEvent) -> ProjectionResult<()> {
    match indexed.event {
        EnsEvent::RegistryTransfer {
            node,
            owner,
            source,
        } => registry_transfer(storage, &indexed.ctx, node, owner, source).await,
        EnsEvent::RegistryNewOwner {
            node,
            label,
            owner,
            source,
        } => registry_new_owner(storage, &indexed.ctx, node, label, owner, source).await,
        EnsEvent::RegistryNewResolver {
            node,
            resolver,
            source,
        } => registry_new_resolver(storage, &indexed.ctx, node, resolver, source).await,
        EnsEvent::RegistryNewTtl { node, ttl, source } => {
            registry_new_ttl(storage, &indexed.ctx, node, ttl, source).await
        }
        EnsEvent::BaseNameRegistered {
            labelhash,
            owner,
            expires,
        } => base_name_registered(storage, &indexed.ctx, labelhash, owner, expires).await,
        EnsEvent::BaseNameRenewed { labelhash, expires } => {
            base_name_renewed(storage, &indexed.ctx, labelhash, expires).await
        }
        EnsEvent::BaseTransfer { to, labelhash, .. } => {
            base_name_transferred(storage, &indexed.ctx, labelhash, to).await
        }
        EnsEvent::ControllerNameRegistered {
            label,
            labelhash,
            cost,
        }
        | EnsEvent::ControllerNameRenewed {
            label,
            labelhash,
            cost,
        } => controller_name_preimage(storage, &indexed.ctx, label, labelhash, cost).await,
        EnsEvent::NameWrapped {
            node,
            dns_name,
            owner,
            fuses,
            expiry,
        } => name_wrapped(storage, &indexed.ctx, node, dns_name, owner, fuses, expiry).await,
        EnsEvent::NameUnwrapped { node, owner } => {
            name_unwrapped(storage, &indexed.ctx, node, owner).await
        }
        EnsEvent::FusesSet { node, fuses } => fuses_set(storage, &indexed.ctx, node, fuses).await,
        EnsEvent::ExpiryExtended { node, expiry } => {
            expiry_extended(storage, &indexed.ctx, node, expiry).await
        }
        EnsEvent::WrappedTransferSingle { to, token_id } => {
            wrapped_transfer(storage, &indexed.ctx, token_id, to, 0).await
        }
        EnsEvent::WrappedTransferBatch { to, token_ids } => {
            for (index, token_id) in token_ids.into_iter().enumerate() {
                wrapped_transfer(storage, &indexed.ctx, token_id, to, index).await?;
            }
            Ok(())
        }
        EnsEvent::ResolverAddrChanged {
            resolver,
            node,
            addr,
        } => resolver_addr_changed(storage, &indexed.ctx, resolver, node, addr).await,
        EnsEvent::ResolverMulticoinAddrChanged {
            resolver,
            node,
            coin_type,
            addr,
        } => {
            resolver_multicoin_addr_changed(storage, &indexed.ctx, resolver, node, coin_type, addr)
                .await
        }
        EnsEvent::ResolverNameChanged {
            resolver,
            node,
            name,
        } => resolver_name_changed(storage, &indexed.ctx, resolver, node, name).await,
        EnsEvent::ResolverAbiChanged {
            resolver,
            node,
            content_type,
        } => resolver_abi_changed(storage, &indexed.ctx, resolver, node, content_type).await,
        EnsEvent::ResolverPubkeyChanged {
            resolver,
            node,
            x,
            y,
        } => resolver_pubkey_changed(storage, &indexed.ctx, resolver, node, x, y).await,
        EnsEvent::ResolverTextChanged {
            resolver,
            node,
            key,
            value,
        } => resolver_text_changed(storage, &indexed.ctx, resolver, node, key, value).await,
        EnsEvent::ResolverContenthashChanged {
            resolver,
            node,
            hash,
        } => resolver_contenthash_changed(storage, &indexed.ctx, resolver, node, hash).await,
        EnsEvent::ResolverInterfaceChanged {
            resolver,
            node,
            interface_id,
            implementer,
        } => {
            resolver_interface_changed(
                storage,
                &indexed.ctx,
                resolver,
                node,
                interface_id,
                implementer,
            )
            .await
        }
        EnsEvent::ResolverAuthorisationChanged {
            resolver,
            node,
            owner,
            target,
            is_authorized,
        } => {
            resolver_authorisation_changed(
                storage,
                &indexed.ctx,
                resolver,
                node,
                owner,
                target,
                is_authorized,
            )
            .await
        }
        EnsEvent::ResolverVersionChanged {
            resolver,
            node,
            version,
        } => resolver_version_changed(storage, &indexed.ctx, resolver, node, version).await,
    }
}
