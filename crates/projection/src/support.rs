use std::str::FromStr;

use alloy_primitives::{Address, B256, U256};
use bigdecimal::BigDecimal;
use storage::{DomainUpsert, Storage, decimal_from_str};
use types::{
    AccountId, DomainId, LabelHash, ResolverId,
    constants::{EMPTY_ADDRESS, ETH_NODE, ETH_REGISTRATION_GRACE_PERIOD_SECONDS, ROOT_NODE},
    decode_dns_name, hex_address, make_subnode, u256_to_decimal_string,
};

use crate::{ProjectionError, ProjectionResult};

pub(crate) const PARENT_CANNOT_CONTROL: i32 = 65_536;

pub(crate) async fn ensure_account(
    storage: &Storage,
    address: Address,
) -> ProjectionResult<String> {
    let account_id = AccountId(address).as_subgraph_id();
    storage.accounts().create_if_missing(&account_id).await?;
    Ok(account_id)
}

pub(crate) async fn ensure_empty_account(storage: &Storage) -> ProjectionResult<String> {
    storage.accounts().create_if_missing(EMPTY_ADDRESS).await?;
    Ok(EMPTY_ADDRESS.to_owned())
}

pub(crate) async fn ensure_domain(
    storage: &Storage,
    domain_id: &str,
    timestamp: i64,
    is_migrated: bool,
) -> ProjectionResult<()> {
    let empty_account = ensure_empty_account(storage).await?;
    let is_root = domain_id == ROOT_NODE;
    storage
        .domains()
        .create_if_missing(DomainUpsert {
            id: domain_id.to_owned(),
            created_at: decimal_from_i64(timestamp)?,
            owner_id: empty_account,
            is_migrated: is_root || is_migrated,
        })
        .await?;
    Ok(())
}

pub(crate) async fn ensure_resolver(
    storage: &Storage,
    resolver: Address,
    node: B256,
    timestamp: i64,
) -> ProjectionResult<String> {
    let domain_id = DomainId(node).as_subgraph_id();
    ensure_domain(storage, &domain_id, timestamp, true).await?;
    let resolver_id = ResolverId {
        address: resolver,
        node,
    }
    .as_subgraph_id();
    storage
        .resolvers()
        .create_if_missing(&resolver_id, &domain_id, &hex_address(resolver))
        .await?;
    Ok(resolver_id)
}

pub(crate) async fn ensure_eth_parent(storage: &Storage, timestamp: i64) -> ProjectionResult<()> {
    ensure_domain(storage, ROOT_NODE, timestamp, true).await?;
    ensure_domain(storage, ETH_NODE, timestamp, true).await?;
    storage
        .domains()
        .set_name_if_unknown(ETH_NODE, Some("eth"), Some("eth"))
        .await?;
    storage
        .domains()
        .set_parent_and_label(ETH_NODE, ROOT_NODE, &eth_labelhash(), true)
        .await?;
    Ok(())
}

pub(crate) fn eth_2ld_domain_id(labelhash: B256) -> ProjectionResult<String> {
    let eth_node = B256::from_str(ETH_NODE)
        .map_err(|_| ProjectionError::InvalidConstant("ETH_NODE".to_owned()))?;
    Ok(DomainId(make_subnode(eth_node, labelhash)).as_subgraph_id())
}

pub(crate) fn eth_labelhash() -> String {
    LabelHash(alloy_primitives::keccak256("eth".as_bytes())).as_subgraph_id()
}

pub(crate) fn bracketed_labelhash(labelhash: B256) -> String {
    format!(
        "[{}]",
        LabelHash(labelhash)
            .as_subgraph_id()
            .trim_start_matches("0x")
    )
}

pub(crate) fn decode_wrapped_name(bytes: &[u8]) -> Option<(String, String)> {
    if bytes.first().copied() == Some(0) {
        return Some((String::new(), ".".to_owned()));
    }

    let name = decode_dns_name(bytes).ok()?;
    let label = name.split('.').next().unwrap_or_default().to_owned();
    Some((label, name))
}

pub(crate) fn check_pcc_burned(fuses: i32) -> bool {
    (fuses & PARENT_CANNOT_CONTROL) == PARENT_CANNOT_CONTROL
}

pub(crate) fn fuses_to_i32(fuses: u32) -> i32 {
    i32::from_ne_bytes(fuses.to_ne_bytes())
}

pub(crate) fn decimal_from_u256(value: U256) -> ProjectionResult<BigDecimal> {
    Ok(decimal_from_str(u256_to_decimal_string(value))?)
}

pub(crate) fn decimal_from_i64(value: i64) -> ProjectionResult<BigDecimal> {
    Ok(decimal_from_str(value.to_string())?)
}

pub(crate) fn expiry_with_grace(expires: U256) -> ProjectionResult<BigDecimal> {
    decimal_from_u256(expires + U256::from(ETH_REGISTRATION_GRACE_PERIOD_SECONDS))
}

pub(crate) fn block_number(ctx: &types::LogContext) -> ProjectionResult<i32> {
    ctx.block_number
        .try_into()
        .map_err(|_| ProjectionError::BlockNumberOutOfRange(ctx.block_number))
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{B256, U256, keccak256};
    use types::{LabelHash, constants::ETH_REGISTRATION_GRACE_PERIOD_SECONDS};

    use super::*;

    #[test]
    fn computes_eth_second_level_domain_id() {
        let labelhash = keccak256("alice".as_bytes());
        let id = eth_2ld_domain_id(labelhash).unwrap();

        assert_eq!(
            id,
            types::DomainId(types::make_subnode(
                B256::from_str(types::constants::ETH_NODE).unwrap(),
                labelhash,
            ))
            .as_subgraph_id()
        );
    }

    #[test]
    fn formats_unknown_labelhash_as_bracketed_label() {
        let labelhash = keccak256("unknown".as_bytes());
        assert_eq!(
            bracketed_labelhash(labelhash),
            format!(
                "[{}]",
                LabelHash(labelhash)
                    .as_subgraph_id()
                    .trim_start_matches("0x")
            )
        );
    }

    #[test]
    fn adds_eth_registration_grace_period() {
        let expires = U256::from(1_000_u64);
        assert_eq!(
            expiry_with_grace(expires).unwrap().to_string(),
            (1_000_u64 + ETH_REGISTRATION_GRACE_PERIOD_SECONDS).to_string()
        );
    }

    #[test]
    fn decodes_wrapped_dns_name() {
        assert_eq!(
            decode_wrapped_name(&[5, b'a', b'l', b'i', b'c', b'e', 3, b'e', b't', b'h', 0]),
            Some(("alice".to_owned(), "alice.eth".to_owned()))
        );
        assert_eq!(
            decode_wrapped_name(&[0]),
            Some((String::new(), ".".to_owned()))
        );
    }

    #[test]
    fn detects_parent_cannot_control_fuse() {
        assert!(check_pcc_burned(PARENT_CANNOT_CONTROL));
        assert!(check_pcc_burned(PARENT_CANNOT_CONTROL | 1));
        assert!(!check_pcc_burned(1));
    }
}
