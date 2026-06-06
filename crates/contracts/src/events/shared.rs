use alloy_primitives::LogData;
use alloy_sol_types::SolEvent;

use crate::model::{DecodeError, FixedLogSource};

pub fn decode_sol_event<E: SolEvent>(
    log: &alloy::rpc::types::Log,
    source: FixedLogSource,
) -> Result<E, DecodeError> {
    let log_data: &LogData = log.as_ref();
    E::decode_raw_log(log_data.topics().iter().copied(), &log_data.data)
        .map_err(|_| DecodeError::NoMatch(source))
}

pub fn decode_resolver_event<E: SolEvent>(log: &alloy::rpc::types::Log) -> Result<E, DecodeError> {
    let log_data: &LogData = log.as_ref();
    E::decode_raw_log(log_data.topics().iter().copied(), &log_data.data)
        .map_err(|_| DecodeError::NoMatchResolver)
}
