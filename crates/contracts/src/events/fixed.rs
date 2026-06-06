use alloy_primitives::{B256, U256};
use alloy_sol_types::SolEvent;

use super::shared::decode_sol_event;
use crate::{
    abi::{
        base_registrar, legacy_eth_registrar_controller, name_wrapper, registry,
        unwrapped_eth_registrar_controller, wrapped_eth_registrar_controller,
    },
    model::{DecodeError, EnsEvent, FixedLogSource},
};

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
