use alloy_primitives::{Address, B256};
use contracts::{EnsEvent, IndexedEvent};
use storage::EntityPreloadIds;
use types::{
    AccountId, DomainId, LabelHash, ResolverId,
    constants::{EMPTY_ADDRESS, ETH_NODE, ROOT_NODE},
    make_subnode,
};

pub(super) fn collect_touched_entities(events: &[IndexedEvent]) -> EntityPreloadIds {
    let mut ids = EntityPreloadIds::default();
    for indexed in events {
        collect_event_entities(indexed, &mut ids);
    }
    ids
}

fn collect_event_entities(indexed: &IndexedEvent, ids: &mut EntityPreloadIds) {
    match &indexed.event {
        EnsEvent::RegistryTransfer { node, owner, .. } => {
            touch_domain(ids, *node);
            touch_account(ids, *owner);
            touch_empty_account(ids);
        }
        EnsEvent::RegistryNewOwner {
            node, label, owner, ..
        } => {
            touch_domain(ids, *node);
            touch_domain(ids, make_subnode(*node, *label));
            touch_account(ids, *owner);
            touch_empty_account(ids);
        }
        EnsEvent::RegistryNewResolver { node, resolver, .. } => {
            touch_domain(ids, *node);
            touch_empty_account(ids);
            if *resolver != Address::ZERO {
                touch_resolver(ids, *resolver, *node);
            }
        }
        EnsEvent::RegistryNewTtl { node, .. } => {
            touch_domain(ids, *node);
            touch_empty_account(ids);
        }
        EnsEvent::BaseNameRegistered {
            labelhash, owner, ..
        } => {
            touch_eth_parent(ids);
            touch_eth_2ld(ids, *labelhash);
            touch_registration(ids, *labelhash);
            touch_account(ids, *owner);
            touch_empty_account(ids);
        }
        EnsEvent::BaseNameRenewed { labelhash, .. } => {
            touch_eth_2ld(ids, *labelhash);
            touch_registration(ids, *labelhash);
        }
        EnsEvent::BaseTransfer { to, labelhash, .. } => {
            touch_eth_2ld(ids, *labelhash);
            touch_registration(ids, *labelhash);
            touch_account(ids, *to);
        }
        EnsEvent::ControllerNameRegistered { labelhash, .. }
        | EnsEvent::ControllerNameRenewed { labelhash, .. } => {
            touch_eth_parent(ids);
            touch_eth_2ld(ids, *labelhash);
            touch_registration(ids, *labelhash);
            touch_empty_account(ids);
        }
        EnsEvent::NameWrapped { node, owner, .. } => {
            touch_domain(ids, *node);
            touch_wrapped_domain(ids, *node);
            touch_account(ids, *owner);
            touch_empty_account(ids);
        }
        EnsEvent::NameUnwrapped { node, owner } => {
            touch_domain(ids, *node);
            touch_wrapped_domain(ids, *node);
            touch_account(ids, *owner);
            touch_empty_account(ids);
        }
        EnsEvent::FusesSet { node, .. } | EnsEvent::ExpiryExtended { node, .. } => {
            touch_domain(ids, *node);
            touch_wrapped_domain(ids, *node);
            touch_empty_account(ids);
        }
        EnsEvent::WrappedTransferSingle { to, token_id } => {
            let node = B256::from(token_id.to_be_bytes());
            touch_domain(ids, node);
            touch_wrapped_domain(ids, node);
            touch_account(ids, *to);
            touch_empty_account(ids);
        }
        EnsEvent::WrappedTransferBatch { to, token_ids } => {
            touch_account(ids, *to);
            touch_empty_account(ids);
            for token_id in token_ids {
                let node = B256::from(token_id.to_be_bytes());
                touch_domain(ids, node);
                touch_wrapped_domain(ids, node);
            }
        }
        EnsEvent::ResolverAddrChanged {
            resolver,
            node,
            addr,
        } => {
            touch_resolver(ids, *resolver, *node);
            touch_account(ids, *addr);
            touch_empty_account(ids);
        }
        EnsEvent::ResolverMulticoinAddrChanged { resolver, node, .. }
        | EnsEvent::ResolverNameChanged { resolver, node, .. }
        | EnsEvent::ResolverAbiChanged { resolver, node, .. }
        | EnsEvent::ResolverPubkeyChanged { resolver, node, .. }
        | EnsEvent::ResolverTextChanged { resolver, node, .. }
        | EnsEvent::ResolverContenthashChanged { resolver, node, .. }
        | EnsEvent::ResolverInterfaceChanged { resolver, node, .. }
        | EnsEvent::ResolverAuthorisationChanged { resolver, node, .. }
        | EnsEvent::ResolverVersionChanged { resolver, node, .. } => {
            touch_resolver(ids, *resolver, *node);
            touch_empty_account(ids);
        }
    }
}

fn touch_account(ids: &mut EntityPreloadIds, address: Address) {
    ids.accounts.insert(AccountId(address).as_subgraph_id());
}

fn touch_empty_account(ids: &mut EntityPreloadIds) {
    ids.accounts.insert(EMPTY_ADDRESS.to_owned());
}

fn touch_domain(ids: &mut EntityPreloadIds, node: B256) {
    ids.domains.insert(DomainId(node).as_subgraph_id());
}

fn touch_registration(ids: &mut EntityPreloadIds, labelhash: B256) {
    ids.registrations
        .insert(LabelHash(labelhash).as_subgraph_id());
}

fn touch_resolver(ids: &mut EntityPreloadIds, resolver: Address, node: B256) {
    ids.resolvers.insert(
        ResolverId {
            address: resolver,
            node,
        }
        .as_subgraph_id(),
    );
}

fn touch_wrapped_domain(ids: &mut EntityPreloadIds, node: B256) {
    ids.wrapped_domains.insert(DomainId(node).as_subgraph_id());
}

fn touch_eth_parent(ids: &mut EntityPreloadIds) {
    ids.domains.insert(ROOT_NODE.to_owned());
    ids.domains.insert(ETH_NODE.to_owned());
}

fn touch_eth_2ld(ids: &mut EntityPreloadIds, labelhash: B256) {
    let eth_node = ETH_NODE
        .parse()
        .expect("ETH_NODE constant is a valid B256 value");
    touch_domain(ids, make_subnode(eth_node, labelhash));
}
