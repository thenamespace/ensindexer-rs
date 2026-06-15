use std::collections::BTreeSet;

use alloy_primitives::Address;
use contracts::{EnsEvent, decode_fixed_source_log};

use crate::sources::LogSource;

pub(crate) fn add_resolver_from_log(
    addresses: &mut BTreeSet<Address>,
    source: LogSource,
    log: &alloy::rpc::types::Log,
) -> anyhow::Result<()> {
    if !matches!(source, LogSource::Fixed(_)) {
        return Ok(());
    }

    let Ok(EnsEvent::RegistryNewResolver { resolver, .. }) =
        decode_fixed_source_log(source.fixed_source()?, log)
    else {
        return Ok(());
    };

    if resolver != Address::ZERO {
        addresses.insert(resolver);
    }
    Ok(())
}
