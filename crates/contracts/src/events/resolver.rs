use alloy_primitives::{Address, U256};
use alloy_sol_types::SolEvent;

use super::shared::decode_resolver_event;
use crate::{
    abi::resolver,
    model::{DecodeError, EnsEvent},
};

pub fn decode_resolver_log(log: &alloy::rpc::types::Log) -> Result<EnsEvent, DecodeError> {
    let Some(topic0) = log.topic0().copied() else {
        return Err(DecodeError::NoMatchResolver);
    };
    let resolver = Address::from(*log.address());

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
