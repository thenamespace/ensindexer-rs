# 02 Source Families And Contract Patterns

A source family is a set of contracts and rules that can emit facts about the global name tree.

It is not a product partition. It is an ingestion boundary.

## Source Family Shape

Each family defines:

- chain id;
- start block;
- contract addresses or discovery roots;
- typed Alloy events;
- adapter that converts logs into normalized events;
- authority scope;
- capabilities;
- manifest version/hash for audit.

Example:

```text
source_family: basenames_base_registrar
chain_id: Base
authority_scope: descendants of base.eth
events: NameRegistered, NameRenewed, Transfer
outputs: RegistrationGranted, RegistrationRenewed, TokenControlTransferred
```

The emitted facts affect names like `alice.base.eth` in the same global tree as `vitalik.eth`.

## Contract Patterns To Support

### Registry

Registry contracts map nodes to owners, resolvers, TTL, subregistries, or parent relationships.

Common raw events:

- `NewOwner`;
- `Transfer`;
- `NewResolver`;
- `NewTTL`;
- `SubregistryUpdated`;
- `ResolverUpdated`;
- `ParentUpdated`.

Normalized outputs:

- `AuthorityChanged`;
- `ResolverChanged`;
- `TtlChanged`;
- `SubregistryChanged`;
- `ParentChanged`.

### Registrar

Registrar contracts create or renew lease-like name rights.

Common raw events:

- `NameRegistered`;
- `NameRenewed`;
- ERC-721 `Transfer`;
- ENS v2 `LabelRegistered`;
- ENS v2 `LabelReserved`;
- ENS v2 `LabelUnregistered`.

Normalized outputs:

- `RegistrationGranted`;
- `RegistrationReserved`;
- `RegistrationRenewed`;
- `RegistrationReleased`;
- `TokenControlChanged`;
- `LabelPreimageObserved`.

### Wrapper

Wrappers add tokenized control and permission fuses on top of names.

Common raw events:

- `NameWrapped`;
- `NameUnwrapped`;
- `FusesSet`;
- `ExpiryExtended`;
- ERC-1155 transfers.

Normalized outputs:

- `NameWrapped`;
- `NameUnwrapped`;
- `PermissionScopeChanged`;
- `ExpiryChanged`;
- `TokenControlChanged`.

### Resolver

Resolvers store records or route resolution.

Common raw events:

- `AddrChanged`;
- `AddressChanged`;
- `TextChanged`;
- `ContenthashChanged`;
- `NameChanged`;
- `VersionChanged`;
- `ABIChanged`;
- `InterfaceChanged`;
- DNS record events;
- ENS v2 named-resource events.

Normalized outputs:

- `RecordChanged`;
- `RecordVersionChanged`;
- `ResolverAliasChanged`;
- `PermissionChanged`.

### Reverse / Primary

Reverse and primary name systems map addresses back to names.

Common raw events:

- reverse registrar claims;
- resolver `NameChanged`;
- `NameForAddrChanged`;
- block-pinned resolver calls for event-silent legacy paths.

Normalized outputs:

- `ReverseClaimChanged`;
- `PrimaryCandidateChanged`;
- `RecordChanged`.

Verified primary names are not just declared claims. Verification requires execution/readback.

### Execution Sources

Some state is only known by calling contracts:

- Universal Resolver;
- CCIP-read paths;
- L1 compatibility resolvers;
- event-silent reverse resolvers.

These calls must be block pinned and auditable.

Outputs:

- execution traces;
- execution steps;
- reusable execution outcomes;
- cache invalidations.

## Adding A New Chain

Adding a new chain should not add new product tables.

It should add:

1. a source-family definition;
2. contract instances or discovery roots;
3. an adapter;
4. normalized event mappings;
5. authority rules for the affected part of the tree;
6. projection tests.

If adding a chain requires changing the public name identity model, the design is wrong.
