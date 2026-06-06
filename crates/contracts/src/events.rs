use alloy_primitives::{Address, B256, LogData, U256};
use alloy_sol_types::SolEvent;

use crate::{
    abi::{
        base_registrar, legacy_eth_registrar_controller, name_wrapper, registry, resolver,
        unwrapped_eth_registrar_controller, wrapped_eth_registrar_controller,
    },
    model::{DecodeError, EnsEvent, FixedLogSource},
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

pub fn decode_fixed_source_log(
    source: FixedLogSource,
    log: &alloy::rpc::types::Log,
) -> Result<EnsEvent, DecodeError> {
    match source {
        FixedLogSource::CurrentRegistry | FixedLogSource::OldRegistry => {
            decode_registry_log(source, log)
        }
        FixedLogSource::BaseRegistrar => decode_base_registrar_log(log),
        FixedLogSource::LegacyEthRegistrarController
        | FixedLogSource::WrappedEthRegistrarController
        | FixedLogSource::UnwrappedEthRegistrarController => decode_controller_log(source, log),
        FixedLogSource::NameWrapper => decode_name_wrapper_log(log),
    }
}

fn decode_registry_log(
    source: FixedLogSource,
    log: &alloy::rpc::types::Log,
) -> Result<EnsEvent, DecodeError> {
    let registry_source = source
        .registry_source()
        .expect("registry source exists for registry logs");
    let Some(topic0) = log.topic0().copied() else {
        return Err(DecodeError::NoMatch(source));
    };

    if topic0 == registry::Transfer::SIGNATURE_HASH {
        let event = decode_sol_event::<registry::Transfer>(log, source)?;
        return Ok(EnsEvent::RegistryTransfer {
            node: event.node,
            owner: event.owner,
            source: registry_source,
        });
    }

    if topic0 == registry::NewOwner::SIGNATURE_HASH {
        let event = decode_sol_event::<registry::NewOwner>(log, source)?;
        return Ok(EnsEvent::RegistryNewOwner {
            node: event.node,
            label: event.label,
            owner: event.owner,
            source: registry_source,
        });
    }

    if topic0 == registry::NewResolver::SIGNATURE_HASH {
        let event = decode_sol_event::<registry::NewResolver>(log, source)?;
        return Ok(EnsEvent::RegistryNewResolver {
            node: event.node,
            resolver: event.resolver,
            source: registry_source,
        });
    }

    if topic0 == registry::NewTTL::SIGNATURE_HASH {
        let event = decode_sol_event::<registry::NewTTL>(log, source)?;
        return Ok(EnsEvent::RegistryNewTtl {
            node: event.node,
            ttl: U256::from(event.ttl),
            source: registry_source,
        });
    }

    Err(DecodeError::NoMatch(source))
}

fn decode_base_registrar_log(log: &alloy::rpc::types::Log) -> Result<EnsEvent, DecodeError> {
    let source = FixedLogSource::BaseRegistrar;
    let Some(topic0) = log.topic0().copied() else {
        return Err(DecodeError::NoMatch(source));
    };

    if topic0 == base_registrar::NameRegistered::SIGNATURE_HASH {
        let event = decode_sol_event::<base_registrar::NameRegistered>(log, source)?;
        return Ok(EnsEvent::BaseNameRegistered {
            labelhash: B256::from(event.id.to_be_bytes()),
            owner: event.owner,
            expires: event.expires,
        });
    }

    if topic0 == base_registrar::NameRenewed::SIGNATURE_HASH {
        let event = decode_sol_event::<base_registrar::NameRenewed>(log, source)?;
        return Ok(EnsEvent::BaseNameRenewed {
            labelhash: B256::from(event.id.to_be_bytes()),
            expires: event.expires,
        });
    }

    if topic0 == base_registrar::Transfer::SIGNATURE_HASH {
        let event = decode_sol_event::<base_registrar::Transfer>(log, source)?;
        return Ok(EnsEvent::BaseTransfer {
            from: event.from,
            to: event.to,
            labelhash: B256::from(event.tokenId.to_be_bytes()),
        });
    }

    Err(DecodeError::NoMatch(source))
}

fn decode_controller_log(
    source: FixedLogSource,
    log: &alloy::rpc::types::Log,
) -> Result<EnsEvent, DecodeError> {
    let Some(topic0) = log.topic0().copied() else {
        return Err(DecodeError::NoMatch(source));
    };

    match source {
        FixedLogSource::LegacyEthRegistrarController => {
            if topic0 == legacy_eth_registrar_controller::NameRegistered::SIGNATURE_HASH {
                let event = decode_sol_event::<legacy_eth_registrar_controller::NameRegistered>(
                    log, source,
                )?;
                return Ok(EnsEvent::ControllerNameRegistered {
                    label: event.name,
                    labelhash: event.label,
                    cost: event.cost,
                });
            }

            if topic0 == legacy_eth_registrar_controller::NameRenewed::SIGNATURE_HASH {
                let event =
                    decode_sol_event::<legacy_eth_registrar_controller::NameRenewed>(log, source)?;
                return Ok(EnsEvent::ControllerNameRenewed {
                    label: event.name,
                    labelhash: event.label,
                    cost: event.cost,
                });
            }
        }
        FixedLogSource::WrappedEthRegistrarController => {
            if topic0 == wrapped_eth_registrar_controller::NameRegistered::SIGNATURE_HASH {
                let event = decode_sol_event::<wrapped_eth_registrar_controller::NameRegistered>(
                    log, source,
                )?;
                return Ok(EnsEvent::ControllerNameRegistered {
                    label: event.name,
                    labelhash: event.label,
                    cost: event.baseCost + event.premium,
                });
            }

            if topic0 == wrapped_eth_registrar_controller::NameRenewed::SIGNATURE_HASH {
                let event =
                    decode_sol_event::<wrapped_eth_registrar_controller::NameRenewed>(log, source)?;
                return Ok(EnsEvent::ControllerNameRenewed {
                    label: event.name,
                    labelhash: event.label,
                    cost: event.cost,
                });
            }
        }
        FixedLogSource::UnwrappedEthRegistrarController => {
            if topic0 == unwrapped_eth_registrar_controller::NameRegistered::SIGNATURE_HASH {
                let event = decode_sol_event::<unwrapped_eth_registrar_controller::NameRegistered>(
                    log, source,
                )?;
                return Ok(EnsEvent::ControllerNameRegistered {
                    label: event.label,
                    labelhash: event.labelhash,
                    cost: event.baseCost + event.premium,
                });
            }

            if topic0 == unwrapped_eth_registrar_controller::NameRenewed::SIGNATURE_HASH {
                let event = decode_sol_event::<unwrapped_eth_registrar_controller::NameRenewed>(
                    log, source,
                )?;
                return Ok(EnsEvent::ControllerNameRenewed {
                    label: event.label,
                    labelhash: event.labelhash,
                    cost: event.cost,
                });
            }
        }
        FixedLogSource::CurrentRegistry
        | FixedLogSource::OldRegistry
        | FixedLogSource::BaseRegistrar
        | FixedLogSource::NameWrapper => {}
    }

    Err(DecodeError::NoMatch(source))
}

fn decode_name_wrapper_log(log: &alloy::rpc::types::Log) -> Result<EnsEvent, DecodeError> {
    let source = FixedLogSource::NameWrapper;
    let Some(topic0) = log.topic0().copied() else {
        return Err(DecodeError::NoMatch(source));
    };

    if topic0 == name_wrapper::NameWrapped::SIGNATURE_HASH {
        let event = decode_sol_event::<name_wrapper::NameWrapped>(log, source)?;
        return Ok(EnsEvent::NameWrapped {
            node: event.node,
            dns_name: event.name.to_vec(),
            owner: event.owner,
            fuses: event.fuses,
            expiry: U256::from(event.expiry),
        });
    }

    if topic0 == name_wrapper::NameUnwrapped::SIGNATURE_HASH {
        let event = decode_sol_event::<name_wrapper::NameUnwrapped>(log, source)?;
        return Ok(EnsEvent::NameUnwrapped {
            node: event.node,
            owner: event.owner,
        });
    }

    if topic0 == name_wrapper::FusesSet::SIGNATURE_HASH {
        let event = decode_sol_event::<name_wrapper::FusesSet>(log, source)?;
        return Ok(EnsEvent::FusesSet {
            node: event.node,
            fuses: event.fuses,
        });
    }

    if topic0 == name_wrapper::ExpiryExtended::SIGNATURE_HASH {
        let event = decode_sol_event::<name_wrapper::ExpiryExtended>(log, source)?;
        return Ok(EnsEvent::ExpiryExtended {
            node: event.node,
            expiry: U256::from(event.expiry),
        });
    }

    if topic0 == name_wrapper::TransferSingle::SIGNATURE_HASH {
        let event = decode_sol_event::<name_wrapper::TransferSingle>(log, source)?;
        return Ok(EnsEvent::WrappedTransferSingle {
            to: event.to,
            token_id: event.id,
        });
    }

    if topic0 == name_wrapper::TransferBatch::SIGNATURE_HASH {
        let event = decode_sol_event::<name_wrapper::TransferBatch>(log, source)?;
        return Ok(EnsEvent::WrappedTransferBatch {
            to: event.to,
            token_ids: event.ids,
        });
    }

    Err(DecodeError::NoMatch(source))
}

pub fn decode_resolver_log(log: &alloy::rpc::types::Log) -> Result<EnsEvent, DecodeError> {
    let Some(topic0) = log.topic0().copied() else {
        return Err(DecodeError::NoMatchResolver);
    };
    let resolver = *log.address();
    let resolver = Address::from(resolver);

    if topic0 == resolver::AddrChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::AddrChanged>(log)?;
        return Ok(EnsEvent::ResolverAddrChanged {
            resolver,
            node: event.node,
            addr: event.a,
        });
    }

    if topic0 == resolver::AddressChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::AddressChanged>(log)?;
        return Ok(EnsEvent::ResolverMulticoinAddrChanged {
            resolver,
            node: event.node,
            coin_type: event.coinType,
            addr: event.newAddress.to_vec(),
        });
    }

    if topic0 == resolver::NameChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::NameChanged>(log)?;
        return Ok(EnsEvent::ResolverNameChanged {
            resolver,
            node: event.node,
            name: event.name,
        });
    }

    if topic0 == resolver::ABIChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::ABIChanged>(log)?;
        return Ok(EnsEvent::ResolverAbiChanged {
            resolver,
            node: event.node,
            content_type: event.contentType,
        });
    }

    if topic0 == resolver::PubkeyChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::PubkeyChanged>(log)?;
        return Ok(EnsEvent::ResolverPubkeyChanged {
            resolver,
            node: event.node,
            x: event.x,
            y: event.y,
        });
    }

    if topic0 == resolver::TextChangedWithValue::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::TextChangedWithValue>(log)?;
        return Ok(EnsEvent::ResolverTextChanged {
            resolver,
            node: event.node,
            key: event.key,
            value: Some(event.value),
        });
    }

    if topic0 == resolver::TextChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::TextChanged>(log)?;
        return Ok(EnsEvent::ResolverTextChanged {
            resolver,
            node: event.node,
            key: event.key,
            value: None,
        });
    }

    if topic0 == resolver::ContenthashChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::ContenthashChanged>(log)?;
        return Ok(EnsEvent::ResolverContenthashChanged {
            resolver,
            node: event.node,
            hash: event.hash.to_vec(),
        });
    }

    if topic0 == resolver::InterfaceChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::InterfaceChanged>(log)?;
        return Ok(EnsEvent::ResolverInterfaceChanged {
            resolver,
            node: event.node,
            interface_id: event.interfaceID,
            implementer: event.implementer,
        });
    }

    if topic0 == resolver::AuthorisationChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::AuthorisationChanged>(log)?;
        return Ok(EnsEvent::ResolverAuthorisationChanged {
            resolver,
            node: event.node,
            owner: event.owner,
            target: event.target,
            is_authorized: event.isAuthorized,
        });
    }

    if topic0 == resolver::VersionChanged::SIGNATURE_HASH {
        let event = decode_resolver_event::<resolver::VersionChanged>(log)?;
        return Ok(EnsEvent::ResolverVersionChanged {
            resolver,
            node: event.node,
            version: U256::from(event.newVersion),
        });
    }

    Err(DecodeError::NoMatchResolver)
}

fn decode_sol_event<E: SolEvent>(
    log: &alloy::rpc::types::Log,
    source: FixedLogSource,
) -> Result<E, DecodeError> {
    let log_data: &LogData = log.as_ref();
    E::decode_raw_log(log_data.topics().iter().copied(), &log_data.data)
        .map_err(|_| DecodeError::NoMatch(source))
}

fn decode_resolver_event<E: SolEvent>(log: &alloy::rpc::types::Log) -> Result<E, DecodeError> {
    let log_data: &LogData = log.as_ref();
    E::decode_raw_log(log_data.topics().iter().copied(), &log_data.data)
        .map_err(|_| DecodeError::NoMatchResolver)
}
