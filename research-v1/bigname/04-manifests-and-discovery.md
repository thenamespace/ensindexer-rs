# 04 Manifests And Discovery

Manifests are source truth, not only deployment config.

They define:

- source families;
- namespace ownership;
- chains;
- watched roots/contracts;
- rollout status;
- capabilities;
- ABI fragments;
- discovery rules;
- start blocks.

## Manifest Location

```text
manifests/<profile>/<chain_combo>/<namespace>/<source_family>/<version>.toml
```

Examples:

```text
manifests/mainnet/ethereum/ens/ens_v1_registry_l1/v1.toml
manifests/mainnet/base/basenames/basenames_base_registry/v2.toml
manifests/sepolia/ethereum/ens/ens_v2_registry_l1/v1.toml
```

One runtime selects one profile root.

## Required Fields

Each manifest contains:

```text
manifest_version
namespace
source_family
chain
deployment_epoch
rollout_status
normalizer_version
capability_flags
roots
contracts
discovery_rules
abi
```

`start_block` is optional. If omitted, the historical start is unknown. Bigname does not infer zero.

## Rollout Status

```text
draft
shadow
active
deprecated
```

Rollout affects admission and public capability, but active source presence alone does not automatically widen route support.

## Capability Flags

Capability flags use:

```text
unsupported
shadow
supported
```

Capability ownership attaches to the declaring source family.

Example:

```text
ens_execution owns verified ENS resolution
basenames_execution owns Basenames verified resolution
basenames_base_primary owns declared Base primary-name claim intake
```

## ABI Entries

ABI entries are human-readable Solidity fragments.

The loader derives:

- event topic0;
- function selector;
- canonical signature;
- indexed fields;
- inputs/outputs.

Manifest entries map ABI events to normalized event kinds:

```toml
[[abi.events]]
name = "ResolverUpdated"
fragment = "event ResolverUpdated(uint256 indexed node, address resolver, address sender)"
emitter_roles = ["registry"]
normalized_events = ["ResolverChanged"]
status = "supported"
```

## Contract Instance Admission

Manifests admit contract instances, not raw address strings.

Rules:

- same admitted address on same chain reuses `contract_instance_id`;
- active range is time-ranged;
- re-admission after inactive gap reuses same instance;
- address replacement creates new instance;
- continuity between different instances is a migration edge;
- proxy and implementation are separate contract instances.

## Discovery Graph

Discovery edges are time-ranged graph relationships.

Edge kinds:

```text
resolver
subregistry
parent
alias
metadata
proxy_implementation
migration
transport
```

Discovery expands the watch plan from manifest roots through reachable contract instances.

## Watch Plan

The watch plan is derived from:

- manifest roots;
- manifest contracts;
- active contract addresses;
- discovery edges;
- event ABI entries;
- active source families;
- rollout status.

Raw intake queries the watch plan. A log from an unknown contract can be retained for audit, but it cannot mutate public state unless admitted.

## ENS Mainnet Source Families

### `ens_v1_registry_l1`

Owns:

- current ENS registry;
- old ENS registry as migration-aware input;
- owner/resolver/ttl/subnode facts;
- registry resolver discovery.

Current registry starts at block `9380380`.

Old registry starts at block `3327417`.

### `ens_v1_registrar_l1`

Owns `.eth` BaseRegistrar and registrar-controller label-bearing intake.

Contracts include:

- BaseRegistrar;
- legacy ETHRegistrarController;
- WrappedETHRegistrarController;
- current ETHRegistrarController.

Events produce registration, renewal, preimage, and token control facts.

### `ens_v1_wrapper_l1`

Owns NameWrapper authority:

- wrapped owner;
- fuses;
- wrapper expiry;
- wrapper-revealed names;
- wrapper-driven resolver/TTL changes;
- wrapper token transfers.

### `ens_v1_resolver_l1`

Owns ENS Labs PublicResolver-generation profile admission.

Registry `NewResolver(node,resolver)` can discover resolver instances, but `NewResolver` alone does not make a resolver fully supported. Profile admission controls complete record-family support.

### `ens_v1_reverse_l1`

Owns declared reverse claim intake from Mainnet ReverseRegistrar.

### `ens_execution`

Owns verified ENS resolution through the Universal Resolver proxy.

## Basenames Mainnet Source Families

### `basenames_base_registry`

Base registry at:

```text
0xb94704422c2a1e396835a571837aa5ae53285a95
```

Owns Base-side owner/resolver/TTL state and resolver discovery.

### `basenames_base_registrar`

Base registrar plus registrar controllers.

Owns Basenames registration, renewal, and token control observations.

### `basenames_base_resolver`

Base L2Resolver profile.

Owns declared resolver-local record/version facts on Base.

### `basenames_base_primary`

Uses ENSv1 Base `L2ReverseRegistrar`:

```text
0x0000000000D8e504002cC26E3Ec46D81971C1664
```

Owns declared primary-name value intake for Base coin type `2147492101`.

It does not own exact-name truth.

### `basenames_l1_compat`

Ethereum L1 Resolver transport compatibility for `base.eth`.

### `basenames_execution`

Same L1 Resolver address, but different source family purpose: verified execution entrypoint.

Bigname separates transport attribution from execution ownership even when the address is the same.

## ENSv2 Sepolia Source Families

### `ens_v2_root_l1`

RootRegistry seed.

### `ens_v2_registry_l1`

ETHRegistry plus discovered UserRegistry instances.

Owns resource/token/subregistry/parent/resolver registry facts.

### `ens_v2_registrar_l1`

ETHRegistrar events.

Owns registrar-local registration intent, renewal, and lifecycle facts.

### `ens_v2_resolver_l1`

PermissionedResolver events.

Owns resolver records, aliases, version boundaries, and resolver-scoped permissions.

## Manifest Changes As Events

Bigname treats manifest changes as first-class semantic changes.

Normalized event taxonomy includes:

```text
SourceManifestUpdated
ProxyImplementationChanged
CapabilityChanged
```

This matters because source admission can affect projection coverage and execution cache validity.

