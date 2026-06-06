use alloy_primitives::{Address, B256, FixedBytes, U256};
use serde::{Deserialize, Serialize};
use types::{LogContext, RegistrySource};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedEvent {
    pub ctx: LogContext,
    pub event: EnsEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnsEvent {
    RegistryTransfer {
        node: B256,
        owner: Address,
        source: RegistrySource,
    },
    RegistryNewOwner {
        node: B256,
        label: B256,
        owner: Address,
        source: RegistrySource,
    },
    RegistryNewResolver {
        node: B256,
        resolver: Address,
        source: RegistrySource,
    },
    RegistryNewTtl {
        node: B256,
        ttl: U256,
        source: RegistrySource,
    },
    BaseNameRegistered {
        labelhash: B256,
        owner: Address,
        expires: U256,
    },
    BaseNameRenewed {
        labelhash: B256,
        expires: U256,
    },
    BaseTransfer {
        from: Address,
        to: Address,
        labelhash: B256,
    },
    ControllerNameRegistered {
        label: String,
        labelhash: B256,
        cost: U256,
    },
    ControllerNameRenewed {
        label: String,
        labelhash: B256,
        cost: U256,
    },
    NameWrapped {
        node: B256,
        dns_name: Vec<u8>,
        owner: Address,
        fuses: u32,
        expiry: U256,
    },
    NameUnwrapped {
        node: B256,
        owner: Address,
    },
    FusesSet {
        node: B256,
        fuses: u32,
    },
    ExpiryExtended {
        node: B256,
        expiry: U256,
    },
    WrappedTransferSingle {
        to: Address,
        token_id: U256,
    },
    WrappedTransferBatch {
        to: Address,
        token_ids: Vec<U256>,
    },
    ResolverAddrChanged {
        resolver: Address,
        node: B256,
        addr: Address,
    },
    ResolverMulticoinAddrChanged {
        resolver: Address,
        node: B256,
        coin_type: U256,
        addr: Vec<u8>,
    },
    ResolverNameChanged {
        resolver: Address,
        node: B256,
        name: String,
    },
    ResolverAbiChanged {
        resolver: Address,
        node: B256,
        content_type: U256,
    },
    ResolverPubkeyChanged {
        resolver: Address,
        node: B256,
        x: B256,
        y: B256,
    },
    ResolverTextChanged {
        resolver: Address,
        node: B256,
        key: String,
        value: Option<String>,
    },
    ResolverContenthashChanged {
        resolver: Address,
        node: B256,
        hash: Vec<u8>,
    },
    ResolverInterfaceChanged {
        resolver: Address,
        node: B256,
        interface_id: FixedBytes<4>,
        implementer: Address,
    },
    ResolverAuthorisationChanged {
        resolver: Address,
        node: B256,
        owner: Address,
        target: Address,
        is_authorized: bool,
    },
    ResolverVersionChanged {
        resolver: Address,
        node: B256,
        version: U256,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixedLogSource {
    CurrentRegistry,
    OldRegistry,
    BaseRegistrar,
    LegacyEthRegistrarController,
    WrappedEthRegistrarController,
    UnwrappedEthRegistrarController,
    NameWrapper,
}

impl FixedLogSource {
    pub fn registry_source(self) -> Option<RegistrySource> {
        match self {
            Self::CurrentRegistry => Some(RegistrySource::Current),
            Self::OldRegistry => Some(RegistrySource::Old),
            Self::BaseRegistrar
            | Self::LegacyEthRegistrarController
            | Self::WrappedEthRegistrarController
            | Self::UnwrappedEthRegistrarController
            | Self::NameWrapper => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("log does not match source {0:?}")]
    NoMatch(FixedLogSource),
    #[error("log does not match a resolver event")]
    NoMatchResolver,
}
