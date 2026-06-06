use alloy_primitives::B256;
use alloy_sol_types::SolEvent;

use crate::{
    abi::{
        base_registrar, legacy_eth_registrar_controller, name_wrapper, registry, resolver,
        unwrapped_eth_registrar_controller, wrapped_eth_registrar_controller,
    },
    model::FixedLogSource,
};

pub fn fixed_source_topic0s(source: FixedLogSource) -> Vec<B256> {
    match source {
        FixedLogSource::CurrentRegistry | FixedLogSource::OldRegistry => vec![
            registry::Transfer::SIGNATURE_HASH,
            registry::NewOwner::SIGNATURE_HASH,
            registry::NewResolver::SIGNATURE_HASH,
            registry::NewTTL::SIGNATURE_HASH,
        ],
        FixedLogSource::BaseRegistrar => vec![
            base_registrar::NameRegistered::SIGNATURE_HASH,
            base_registrar::NameRenewed::SIGNATURE_HASH,
            base_registrar::Transfer::SIGNATURE_HASH,
        ],
        FixedLogSource::LegacyEthRegistrarController => vec![
            legacy_eth_registrar_controller::NameRegistered::SIGNATURE_HASH,
            legacy_eth_registrar_controller::NameRenewed::SIGNATURE_HASH,
        ],
        FixedLogSource::WrappedEthRegistrarController => vec![
            wrapped_eth_registrar_controller::NameRegistered::SIGNATURE_HASH,
            wrapped_eth_registrar_controller::NameRenewed::SIGNATURE_HASH,
        ],
        FixedLogSource::UnwrappedEthRegistrarController => vec![
            unwrapped_eth_registrar_controller::NameRegistered::SIGNATURE_HASH,
            unwrapped_eth_registrar_controller::NameRenewed::SIGNATURE_HASH,
        ],
        FixedLogSource::NameWrapper => vec![
            name_wrapper::NameWrapped::SIGNATURE_HASH,
            name_wrapper::NameUnwrapped::SIGNATURE_HASH,
            name_wrapper::FusesSet::SIGNATURE_HASH,
            name_wrapper::ExpiryExtended::SIGNATURE_HASH,
            name_wrapper::TransferSingle::SIGNATURE_HASH,
            name_wrapper::TransferBatch::SIGNATURE_HASH,
        ],
    }
}

pub fn resolver_topic0s() -> Vec<B256> {
    vec![
        resolver::AddrChanged::SIGNATURE_HASH,
        resolver::AddressChanged::SIGNATURE_HASH,
        resolver::NameChanged::SIGNATURE_HASH,
        resolver::ABIChanged::SIGNATURE_HASH,
        resolver::PubkeyChanged::SIGNATURE_HASH,
        resolver::TextChanged::SIGNATURE_HASH,
        resolver::TextChangedWithValue::SIGNATURE_HASH,
        resolver::ContenthashChanged::SIGNATURE_HASH,
        resolver::InterfaceChanged::SIGNATURE_HASH,
        resolver::AuthorisationChanged::SIGNATURE_HASH,
        resolver::VersionChanged::SIGNATURE_HASH,
    ]
}
