use alloy_primitives::{Address, B256, U256, keccak256};
use serde::{Deserialize, Serialize};

pub mod constants {
    pub const MAINNET_CHAIN_ID: u64 = 1;
    pub const ETH_REGISTRATION_GRACE_PERIOD_SECONDS: u64 = 7_776_000;
    pub const ROOT_NODE: &str =
        "0x0000000000000000000000000000000000000000000000000000000000000000";
    pub const ETH_NODE: &str = "0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae";
    pub const EMPTY_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

    pub const ENS_REGISTRY_ADDRESS: &str = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e";
    pub const OLD_ENS_REGISTRY_ADDRESS: &str = "0x314159265dD8dbb310642f98f50C066173C1259b";
    pub const BASE_REGISTRAR_ADDRESS: &str = "0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85";
    pub const LEGACY_ETH_REGISTRAR_CONTROLLER_ADDRESS: &str =
        "0x283Af0B28c62C092C9727F1Ee09c02CA627EB7F5";
    pub const WRAPPED_ETH_REGISTRAR_CONTROLLER_ADDRESS: &str =
        "0x253553366Da8546fC250F225fe3d25d0C782303b";
    pub const UNWRAPPED_ETH_REGISTRAR_CONTROLLER_ADDRESS: &str =
        "0x59E16fcCd424Cc24e280Be16E11Bcd56fb0CE547";
    pub const NAME_WRAPPER_ADDRESS: &str = "0xD4416b13d2b3a9aBae7AcD5D6C2BbDBE25686401";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DomainId(pub B256);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LabelHash(pub B256);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub Address);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResolverId {
    pub address: Address,
    pub node: B256,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrySource {
    Old,
    Current,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogContext {
    pub block_number: i64,
    pub block_timestamp: i64,
    pub block_hash: B256,
    pub transaction_hash: B256,
    pub transaction_index: i64,
    pub log_index: i64,
    pub contract_address: Address,
}

impl LogContext {
    pub fn event_id(&self) -> String {
        format!("{}-{}", self.block_number, self.log_index)
    }

    pub fn batch_event_id(&self, item_index: usize) -> String {
        format!("{}-{}-{item_index}", self.block_number, self.log_index)
    }
}

impl DomainId {
    pub fn as_subgraph_id(&self) -> String {
        hex_b256(self.0)
    }
}

impl LabelHash {
    pub fn as_subgraph_id(&self) -> String {
        hex_b256(self.0)
    }

    pub fn from_token_id(token_id: U256) -> Self {
        Self(B256::from(token_id.to_be_bytes()))
    }

    pub fn as_token_id(&self) -> U256 {
        U256::from_be_bytes(self.0.0)
    }
}

impl AccountId {
    pub fn as_subgraph_id(&self) -> String {
        hex_address(self.0)
    }
}

impl ResolverId {
    pub fn as_subgraph_id(&self) -> String {
        format!("{}-{}", hex_address(self.address), hex_b256(self.node))
    }
}

pub fn hex_address(address: Address) -> String {
    format!("{address:#x}")
}

pub fn hex_b256(value: B256) -> String {
    format!("{value:#x}")
}

pub fn hex_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 + bytes.len() * 2);
    out.push_str("0x");
    for byte in bytes {
        use std::fmt::Write;
        let _ = write!(out, "{byte:02x}");
    }
    out
}

pub fn make_subnode(parent: B256, labelhash: B256) -> B256 {
    let mut input = [0_u8; 64];
    input[..32].copy_from_slice(parent.as_slice());
    input[32..].copy_from_slice(labelhash.as_slice());
    keccak256(input)
}

pub fn u256_to_decimal_string(value: U256) -> String {
    value.to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum EnsNameError {
    #[error("invalid DNS-encoded ENS name")]
    InvalidDnsName,
    #[error("invalid ENS label")]
    InvalidLabel,
}

pub fn validate_label(label: &str) -> Result<(), EnsNameError> {
    if label.is_empty() || label.chars().any(|ch| matches!(ch, '\0' | '.' | '[' | ']')) {
        return Err(EnsNameError::InvalidLabel);
    }

    Ok(())
}

pub fn decode_dns_name(bytes: &[u8]) -> Result<String, EnsNameError> {
    let mut labels = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
        let len = bytes[offset] as usize;
        offset += 1;

        if len == 0 {
            break;
        }

        let end = offset
            .checked_add(len)
            .ok_or(EnsNameError::InvalidDnsName)?;
        let label = bytes.get(offset..end).ok_or(EnsNameError::InvalidDnsName)?;
        labels.push(std::str::from_utf8(label).map_err(|_| EnsNameError::InvalidDnsName)?);
        offset = end;
    }

    Ok(labels.join("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_batch_event_id() {
        let ctx = LogContext {
            block_number: 10,
            block_timestamp: 20,
            block_hash: B256::ZERO,
            transaction_hash: B256::ZERO,
            transaction_index: 1,
            log_index: 7,
            contract_address: Address::ZERO,
        };

        assert_eq!(ctx.event_id(), "10-7");
        assert_eq!(ctx.batch_event_id(2), "10-7-2");
    }

    #[test]
    fn decodes_dns_name() {
        assert_eq!(decode_dns_name(&[3, b'e', b't', b'h', 0]).unwrap(), "eth");
    }

    #[test]
    fn rejects_graph_invalid_labels() {
        for label in ["", "foo.bar", "foo[bar", "foo]bar", "foo\0bar"] {
            assert!(validate_label(label).is_err(), "{label:?}");
        }

        assert!(validate_label("foo").is_ok());
    }

    #[test]
    fn computes_eth_subnode() {
        let root = B256::ZERO;
        let eth_labelhash = keccak256("eth".as_bytes());

        assert_eq!(
            hex_b256(make_subnode(root, eth_labelhash)),
            constants::ETH_NODE
        );
    }
}
