# Official Subgraph Overview

This document describes what the official ENS subgraph indexes and how that maps to a custom Rust implementation.

## Networks

The official repository includes `mainnet`, `sepolia`, and `holesky` network config. This project is targeting mainnet first.

Mainnet indexed sources:

| Data source | Address | Start block | Mapping |
| --- | --- | ---: | --- |
| `ENSRegistry` | `0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e` | `9380380` | `src/ensRegistry.ts` |
| `ENSRegistryOld` | `0x314159265dd8dbb310642f98f50c066173c1259b` | `3327417` | `src/ensRegistry.ts` |
| `Resolver` | wildcard contract address | `3327417` conceptually | `src/resolver.ts` |
| `BaseRegistrar` | `0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85` | `9380410` | `src/ethRegistrar.ts` |
| `LegacyEthRegistrarController` | `0x283Af0B28c62C092C9727F1Ee09c02CA627EB7F5` | `9380471` | `src/ethRegistrar.ts` |
| `WrappedEthRegistrarController` | `0x253553366Da8546fC250F225fe3d25d0C782303b` | `16925618` | `src/ethRegistrar.ts` |
| `NameWrapper` | `0xD4416b13d2b3a9aBae7AcD5D6C2BbDBE25686401` | `16925608` | `src/nameWrapper.ts` |
| `UnwrappedEthRegistrarController` | `0x59E16fcCd424Cc24e280Be16E11Bcd56fb0CE547` | `22764821` | `src/ethRegistrar.ts` |

The `Resolver` source is a wildcard source in The Graph: it has an ABI but no fixed address. Events are decoded from any resolver contract that emits the public resolver event signatures. A from-scratch indexer must emulate this by filtering logs by topic signatures and decoding by ABI, without an address filter.

## Indexed Event Signatures

### ENS Registry and Old Registry

Both registry contracts use the same ABI:

| Event | Handler | Meaning |
| --- | --- | --- |
| `Transfer(indexed bytes32,address)` | `handleTransfer` / `handleTransferOldRegistry` | registry owner for an existing node changed |
| `NewOwner(indexed bytes32,indexed bytes32,address)` | `handleNewOwner` / `handleNewOwnerOldRegistry` | subnode owner changed or subnode created |
| `NewResolver(indexed bytes32,address)` | `handleNewResolver` / `handleNewResolverOldRegistry` | resolver address changed |
| `NewTTL(indexed bytes32,uint64)` | `handleNewTTL` / `handleNewTTLOldRegistry` | TTL changed |

Old registry handlers are migration-aware. They only update old-registry domains while the domain is not migrated.

### Base Registrar

The base registrar is the `.eth` ERC-721 registrar.

| Event | Handler | Meaning |
| --- | --- | --- |
| `NameRegistered(indexed uint256,indexed address,uint256)` | `handleNameRegistered` | creates/updates a `.eth` `Registration` |
| `NameRenewed(indexed uint256,uint256)` | `handleNameRenewed` | updates registration expiry |
| `Transfer(indexed address,indexed address,indexed uint256)` | `handleNameTransferred` | updates ERC-721 registrant |

The labelhash is emitted as `uint256`. The subgraph converts it to a 32-byte big-endian byte array and uses that byte hex as the `Registration.id`.

### Registrar Controllers

Controller contracts are not the source of lifecycle registration events in the entity graph. They enrich `Registration` and `Domain` records with known label preimages and costs.

| Contract | Event | Handler | Projection |
| --- | --- | --- | --- |
| `LegacyEthRegistrarController` | `NameRegistered(string,indexed bytes32,indexed address,uint256,uint256)` | `handleNameRegisteredByLegacyController` | set label preimage and cost |
| `LegacyEthRegistrarController` | `NameRenewed(string,indexed bytes32,uint256,uint256)` | `handleNameRenewedByLegacyController` | set label preimage and cost |
| `WrappedEthRegistrarController` | `NameRegistered(string,indexed bytes32,indexed address,uint256,uint256,uint256)` | `handleNameRegisteredByWrappedController` | set label preimage and `baseCost + premium` |
| `WrappedEthRegistrarController` | `NameRenewed(string,indexed bytes32,uint256,uint256)` | `handleNameRenewedByLegacyController` | set label preimage and cost |
| `UnwrappedEthRegistrarController` | `NameRegistered(string,indexed bytes32,indexed address,uint256,uint256,uint256,bytes32)` | `handleNameRegisteredByUnwrappedController` | set label preimage and `baseCost + premium` |
| `UnwrappedEthRegistrarController` | `NameRenewed(string,indexed bytes32,uint256,uint256,bytes32)` | `handleNameRenewedByUnwrappedController` | set label preimage and cost |

### Name Wrapper

| Event | Handler | Meaning |
| --- | --- | --- |
| `NameWrapped(indexed bytes32,bytes,address,uint32,uint64)` | `handleNameWrapped` | domain wrapped, fuses/expiry/owner/name set |
| `NameUnwrapped(indexed bytes32,address)` | `handleNameUnwrapped` | wrapped owner cleared and `WrappedDomain` removed |
| `FusesSet(indexed bytes32,uint32)` | `handleFusesSet` | fuses updated |
| `ExpiryExtended(indexed bytes32,uint64)` | `handleExpiryExtended` | wrapped expiry updated |
| `TransferSingle(indexed address,indexed address,indexed address,uint256,uint256)` | `handleTransferSingle` | ERC-1155 single wrapped token owner changed |
| `TransferBatch(indexed address,indexed address,indexed address,uint256[],uint256[])` | `handleTransferBatch` | ERC-1155 batch owner changed |

The ERC-1155 transfer events can arrive before `NameWrapped` for new registrations. The official subgraph creates placeholder `WrappedDomain` rows on transfer and later fills them from `NameWrapped`.

### Resolver

Resolver events are wildcard-decoded from resolver contracts.

| Event | Handler | Entity |
| --- | --- | --- |
| `ABIChanged(indexed bytes32,indexed uint256)` | `handleABIChanged` | `AbiChanged` |
| `AddrChanged(indexed bytes32,address)` | `handleAddrChanged` | `AddrChanged` |
| `AddressChanged(indexed bytes32,uint256,bytes)` | `handleMulticoinAddrChanged` | `MulticoinAddrChanged` |
| `AuthorisationChanged(indexed bytes32,indexed address,indexed address,bool)` | `handleAuthorisationChanged` | `AuthorisationChanged` |
| `ContenthashChanged(indexed bytes32,bytes)` | `handleContentHashChanged` | `ContenthashChanged` |
| `InterfaceChanged(indexed bytes32,indexed bytes4,address)` | `handleInterfaceChanged` | `InterfaceChanged` |
| `NameChanged(indexed bytes32,string)` | `handleNameChanged` | `NameChanged` |
| `PubkeyChanged(indexed bytes32,bytes32,bytes32)` | `handlePubkeyChanged` | `PubkeyChanged` |
| `TextChanged(indexed bytes32,indexed string,string)` | `handleTextChanged` | `TextChanged` |
| `TextChanged(indexed bytes32,indexed string,string,string)` | `handleTextChangedWithValue` | `TextChanged` |
| `VersionChanged(indexed bytes32,uint64)` | `handleVersionChanged` | `VersionChanged` |

Resolver IDs are `{resolver_address}-{node}`.

## Constants

| Name | Value |
| --- | --- |
| `ROOT_NODE` | `0x0000000000000000000000000000000000000000000000000000000000000000` |
| `ETH_NODE` | `0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae` |
| `EMPTY_ADDRESS` | `0x0000000000000000000000000000000000000000` |
| `.eth` grace period | `7776000` seconds, 90 days |
| `PARENT_CANNOT_CONTROL` fuse | `65536` |

## Event IDs

Most immutable event entities use:

```text
{block_number}-{log_index}
```

Name wrapper ERC-1155 transfer batch events append the item index:

```text
{block_number}-{log_index}-{batch_index}
```

The Rust indexer should preserve these exact IDs for subgraph-compatible query results.

