use std::str::FromStr;

use alloy_primitives::Address;
use contracts::FixedLogSource;
use serde::{Deserialize, Serialize};
use types::constants::{
    BASE_REGISTRAR_ADDRESS, ENS_REGISTRY_ADDRESS, LEGACY_ETH_REGISTRAR_CONTROLLER_ADDRESS,
    NAME_WRAPPER_ADDRESS, OLD_ENS_REGISTRY_ADDRESS, UNWRAPPED_ETH_REGISTRAR_CONTROLLER_ADDRESS,
    WRAPPED_ETH_REGISTRAR_CONTROLLER_ADDRESS,
};

#[derive(Debug, Clone, Copy)]
pub struct FixedSource {
    pub source: FixedLogSource,
    pub address: Address,
    pub start_block: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum LogSource {
    Fixed(FixedLogSource),
    Resolver,
}

impl LogSource {
    pub(crate) fn fixed_source(self) -> anyhow::Result<FixedLogSource> {
        match self {
            Self::Fixed(source) => Ok(source),
            Self::Resolver => anyhow::bail!("resolver log source is not fixed"),
        }
    }
}

impl FixedSource {
    pub fn checkpoint_name(&self) -> &'static str {
        match self.source {
            FixedLogSource::CurrentRegistry => "registry_current",
            FixedLogSource::OldRegistry => "registry_old",
            FixedLogSource::BaseRegistrar => "base_registrar",
            FixedLogSource::LegacyEthRegistrarController => "legacy_eth_registrar_controller",
            FixedLogSource::WrappedEthRegistrarController => "wrapped_eth_registrar_controller",
            FixedLogSource::UnwrappedEthRegistrarController => "unwrapped_eth_registrar_controller",
            FixedLogSource::NameWrapper => "name_wrapper",
        }
    }
}

pub fn fixed_sources() -> anyhow::Result<Vec<FixedSource>> {
    Ok(vec![
        FixedSource {
            source: FixedLogSource::OldRegistry,
            address: Address::from_str(OLD_ENS_REGISTRY_ADDRESS)?,
            start_block: 3_327_417,
        },
        FixedSource {
            source: FixedLogSource::CurrentRegistry,
            address: Address::from_str(ENS_REGISTRY_ADDRESS)?,
            start_block: 9_380_380,
        },
        FixedSource {
            source: FixedLogSource::BaseRegistrar,
            address: Address::from_str(BASE_REGISTRAR_ADDRESS)?,
            start_block: 9_380_410,
        },
        FixedSource {
            source: FixedLogSource::LegacyEthRegistrarController,
            address: Address::from_str(LEGACY_ETH_REGISTRAR_CONTROLLER_ADDRESS)?,
            start_block: 9_380_471,
        },
        FixedSource {
            source: FixedLogSource::NameWrapper,
            address: Address::from_str(NAME_WRAPPER_ADDRESS)?,
            start_block: 16_925_608,
        },
        FixedSource {
            source: FixedLogSource::WrappedEthRegistrarController,
            address: Address::from_str(WRAPPED_ETH_REGISTRAR_CONTROLLER_ADDRESS)?,
            start_block: 16_925_618,
        },
        FixedSource {
            source: FixedLogSource::UnwrappedEthRegistrarController,
            address: Address::from_str(UNWRAPPED_ETH_REGISTRAR_CONTROLLER_ADDRESS)?,
            start_block: 22_764_821,
        },
    ])
}

pub fn first_source_start_block() -> anyhow::Result<u64> {
    fixed_sources()?
        .into_iter()
        .map(|source| source.start_block)
        .min()
        .ok_or_else(|| anyhow::anyhow!("no ingest sources configured"))
}

#[cfg(test)]
mod tests {
    use contracts::{FixedLogSource, fixed_source_topic0s};

    use super::*;

    #[test]
    fn fixed_sources_are_ordered_by_start_block() {
        let sources = fixed_sources().unwrap();
        assert_eq!(sources[0].source, FixedLogSource::OldRegistry);
        assert_eq!(sources[1].source, FixedLogSource::CurrentRegistry);
        assert_eq!(sources[2].source, FixedLogSource::BaseRegistrar);
        assert_eq!(
            sources[3].source,
            FixedLogSource::LegacyEthRegistrarController
        );
        assert_eq!(sources[4].source, FixedLogSource::NameWrapper);
        assert_eq!(
            sources[5].source,
            FixedLogSource::WrappedEthRegistrarController
        );
        assert_eq!(
            sources[6].source,
            FixedLogSource::UnwrappedEthRegistrarController
        );
    }

    #[test]
    fn source_topic_sets_are_non_empty() {
        for source in fixed_sources().unwrap() {
            assert!(!fixed_source_topic0s(source.source).is_empty());
        }
    }
}
