# 05 Adapters And Event Handling

Adapters turn raw facts into bigname normalized events and identity rows.

The boundary:

```text
raw_logs / raw_transactions / raw_receipts / code / call snapshots
  -> adapter-specific decoding and state
  -> identity rows
  -> normalized_events
```

Adapters do not write projection tables.

## Normalized Event Taxonomy

Bigname's normalized events are shared semantic events.

Important kinds:

```text
PreimageObserved
NameClassified
SurfaceBound
SurfaceUnbound
ContractDiscovered
MetadataChanged
SourceManifestUpdated

RegistrationReserved
RegistrationGranted
RegistrarNameRegistered
RegistrationRenewed
RegistrationReleased
ExpiryChanged
AuthorityTransferred
AuthorityEpochChanged
MigrationApplied
PricingPolicyChanged

TokenResourceLinked
TokenRegenerated
TokenControlTransferred
ResolutionEpochChanged

ResolverChanged
SubregistryChanged
ParentChanged
AliasChanged
WildcardCoverageChanged
RecordChanged
RecordVersionChanged
RecordInventoryObserved

PermissionChanged
RootPermissionChanged
PermissionScopeChanged

ReverseChanged
PrimaryNameClaimed
PrimaryNameVerified
PrimaryNameInvalidated

VerifiedResolutionObserved
VerifiedResolutionInvalidated
CoverageChanged
```

Not every source emits every event. The point is that downstream projections consume shared semantics instead of source-specific logs.

## Adapter Replay Models

Bigname classifies adapters by replay safety.

### `stateless_raw_fact`

The raw log plus manifest/source metadata is enough.

Examples:

- `block_derived_normalized_events`
- `ens_v1_reverse_claim`

Restricted block/source replay is allowed.

### `contextual_dependency_required`

The raw log is not enough. The event needs stable output from another adapter or discovery graph.

Examples:

- `ens_v1_subregistry_discovery`
- `ens_v2_registrar`
- `ens_v2_resolver`
- `manifest_normalized_events`

Restricted replay is denied unless dependency closure is proven.

### `stateful_closure_required`

The adapter needs chronological history and internal state.

Examples:

- `ens_v1_unwrapped_authority`
- `ens_v2_registry_resource_surface`
- `ens_v2_permissions`

Replay must start from a valid closure boundary or durable adapter snapshot.

## Generic Adapter Output

Each normalized event carries:

- event identity;
- event kind;
- namespace;
- logical name id when known;
- resource id when known;
- token lineage id when known;
- source family;
- manifest id/version;
- raw fact reference;
- chain id;
- block number;
- block hash;
- tx hash;
- log index;
- before state;
- after state;
- canonicality state.

Event identity is deterministic. Replay inserts the same identity again as an upsert. If payload differs and no documented repair applies, storage treats it as a conflict.

## ENS v1 Registry Handling

Source family:

```text
ens_v1_registry_l1
```

Input contracts:

- ENSRegistryOld;
- current ENSRegistry.

Important raw events:

```text
NewOwner(bytes32 node, bytes32 label, address owner)
Transfer(bytes32 node, address owner)
NewResolver(bytes32 node, address resolver)
NewTTL(bytes32 node, uint64 ttl)
```

Typical normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| `NewOwner` | `AuthorityTransferred`, `SurfaceBound`, sometimes `PreimageObserved` if label known elsewhere | Creates/updates registry authority and child edge. |
| `Transfer` | `AuthorityTransferred`, `PermissionChanged` | Updates registry owner for node. |
| `NewResolver` | `ResolverChanged`, possibly `ContractDiscovered` | Nonzero resolver can update resolver binding and discovery graph. |
| `NewTTL` | TTL/control state in normalized payload | Feeds exact-name profile. |

Old-registry events pass migration guard. After current registry has migrated a node, old registry updates are retained but suppressed, except root resolver.

## ENS v1 Registrar Handling

Source family:

```text
ens_v1_registrar_l1
```

Input contracts:

- BaseRegistrar;
- legacy ETHRegistrarController;
- WrappedETHRegistrarController;
- current ETHRegistrarController.

Important raw events:

```text
NameRegistered
NameRenewed
Transfer
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| Registrar `NameRegistered` | `RegistrationGranted`, `PreimageObserved` when label is present | Establishes registrar lease/resource and label preimage. |
| Controller `NameRegistered` | `RegistrationGranted` | Label-bearing registration observation. |
| `NameRenewed` | `RegistrationRenewed`, sometimes `PreimageObserved` | Extends expiry on same registrar resource. |
| ERC721 `Transfer` | `TokenControlTransferred` | Updates token holder lineage. |

Registrar resources are stable across renewal and transfer. Full lapse and re-registration mints a new resource/token lineage.

## ENS v1 Wrapper Handling

Source family:

```text
ens_v1_wrapper_l1
```

Important raw events:

```text
NameWrapped
NameUnwrapped
FusesSet
ExpiryExtended
TransferSingle
TransferBatch
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| `NameWrapped` | `WrapperNameWrapped`, `PreimageObserved`, `SurfaceBound`, `PermissionScopeChanged`, `TokenControlTransferred` | Moves authority to wrapper resource. |
| `NameUnwrapped` | `WrapperNameUnwrapped`, `SurfaceUnbound`, authority transition | May reactivate prior registrar resource if same lease. |
| `FusesSet` | `PermissionScopeChanged` | Updates effective powers. |
| `ExpiryExtended` | `ExpiryChanged` | Updates wrapper expiry. |
| ERC1155 transfer | `TokenControlTransferred` | Updates wrapper token holder. |

Wrapper fuse projection is important. Bigname masks effective powers when fuses prohibit operations.

## ENS v1 Resolver Handling

Source family:

```text
ens_v1_resolver_l1
```

Resolver discovery starts from registry `NewResolver`.

Raw resolver-local events include:

```text
ABIChanged
AddrChanged
AddressChanged
ContenthashChanged
DNSRecordChanged
DNSZonehashChanged
InterfaceChanged
NameChanged
TextChanged
VersionChanged
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| `AddrChanged` | `RecordChanged` | ETH address selector. |
| `AddressChanged` | `RecordChanged` | Multicoin address selector. |
| `TextChanged` | `RecordChanged` | Text key/value selector. |
| `ContenthashChanged` | `RecordChanged` | Contenthash selector. |
| DNS/interface/ABI/data events | `RecordChanged` | Selector-specific records. |
| `NameChanged` | `RecordChanged`, preimage observation in reverse/primary flows | Does not synthesize authority. |
| `VersionChanged` | `RecordVersionChanged` | Boundary invalidates prior record inventory. |

Important gate:

```text
NewResolver observation != full resolver support
```

Resolver profile admission decides whether complete record-family coverage is supported. Unknown resolvers stay pending/unsupported.

## ENS v1 Reverse Claim Handling

Source family:

```text
ens_v1_reverse_l1
```

Important raw event:

```text
ReverseClaimed
```

Normalized output:

```text
ReverseChanged
```

For legacy event-silent reverse resolvers, bigname has projection-owned hydration triggered by retained successful direct-call observations and resolver-edge changes. That hydration updates `primary_names_current`; it does not create new adapter-owned normalized events from calldata.

## ENS v2 Registry Handling

Source families:

```text
ens_v2_root_l1
ens_v2_registry_l1
```

Important raw events:

```text
LabelRegistered
LabelReserved
LabelUnregistered
ExpiryUpdated
SubregistryUpdated
ResolverUpdated
TokenResource
TokenRegenerated
ParentUpdated
EACRolesChanged
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| `LabelRegistered` | `RegistrationGranted`, `PreimageObserved` | Creates or updates surface/resource state. |
| `LabelReserved` | `RegistrationReserved` | Reservation state. |
| `LabelUnregistered` | `RegistrationReleased` | Releases resource/surface. |
| `ExpiryUpdated` | `ExpiryChanged` | Updates expiry. |
| `SubregistryUpdated` | `SubregistryChanged`, discovery edge updates | Endpoint resolves to contract instance. |
| `ResolverUpdated` | `ResolverChanged`, discovery edge updates | Resolver endpoint admission. |
| `TokenResource` | `TokenResourceLinked` | Links token id to stable EAC resource. |
| `TokenRegenerated` | `TokenRegenerated` | Preserves `resource_id` and `token_lineage_id`; updates token id. |
| `ParentUpdated` | `ParentChanged` | Updates registry parent graph. |
| `EACRolesChanged` | `PermissionChanged`, `RootPermissionChanged` | Role bitmap decoded by registry role vocabulary. |

ENSv2 enrichment calls must be block-hash pinned:

```text
getResource(anyId)
getTokenId(anyId)
getState(anyId)
```

Number-only ambiguous state reads are not allowed to rewrite log-derived facts as reorg-proof.

## ENS v2 Registrar Handling

Source family:

```text
ens_v2_registrar_l1
```

Important raw events:

```text
NameRegistered
NameRenewed
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| `NameRegistered` | `RegistrarNameRegistered` | Registrar-local registration intent; links to registry resource when available. |
| `NameRenewed` | `RegistrationRenewed` | Renewal fact for existing resource. |

This adapter is contextual because it needs stable registry/resource identity from ENSv2 registry replay.

## ENS v2 Resolver Handling

Source family:

```text
ens_v2_resolver_l1
```

Important raw events:

```text
AddressChanged
TextChanged
ContenthashChanged
NameChanged
VersionChanged
AliasChanged
NamedResource
NamedTextResource
NamedAddrResource
EACRolesChanged
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| address/text/contenthash/name events | `RecordChanged` | Resolver records. |
| `VersionChanged` | `RecordVersionChanged` | Record boundary. |
| `AliasChanged` | `AliasChanged` | Alias path source/destination. |
| `NamedResource` / named record events | `RecordChanged`, `PermissionChanged` | Named resource and permission effects. |
| `EACRolesChanged` | `PermissionChanged` | Resolver-scoped roles. |

This adapter is contextual because resolver rows need stable name/resource links from registry output.

## Basenames Registry Handling

Source family:

```text
basenames_base_registry
```

Important raw events are ENS-registry-like:

```text
NewOwner
Transfer
NewResolver
NewTTL
```

Normalized output:

| Raw Event | Normalized Output | Notes |
| --- | --- | --- |
| `NewOwner` | `AuthorityTransferred` | Base-side authority for Basenames namespace. |
| `Transfer` | `AuthorityTransferred` | Owner transfer. |
| `NewResolver` | `ResolverChanged`, resolver discovery | Discovers Base resolver instances. |
| `NewTTL` | `AuthorityEpochChanged` / TTL state | Feeds current exact-name state. |

Bigname treats these as `basenames:*` surfaces, not children under `ens:base.eth`.

## Basenames Registrar Handling

Source family:

```text
basenames_base_registrar
```

Important raw events:

```text
NameRegistered
NameRenewed
Transfer
```

Normalized output:

| Raw Event | Normalized Output |
| --- | --- |
| `NameRegistered` | `RegistrationGranted` |
| `NameRenewed` | `RegistrationRenewed` |
| token transfer | `TokenControlTransferred` |

## Basenames Resolver Handling

Source family:

```text
basenames_base_resolver
```

Important raw events:

```text
AddrChanged
AddressChanged
TextChanged
ContenthashChanged
VersionChanged
```

Normalized output:

```text
RecordChanged
RecordVersionChanged
```

Resolver-local fact consumption requires L2Resolver-compatible profile admission.

## Basenames Primary Handling

Source family:

```text
basenames_base_primary
```

Input event:

```text
NameForAddrChanged(address,string)
```

Output:

```text
ReverseChanged
RecordChanged(name claim observation)
```

It is declared claim intake only. It does not own exact-name, address-name, or children truth.

## Basenames L1 Compatibility And Execution

`basenames_l1_compat` and `basenames_execution` may reference the same L1 Resolver address, but they have different meanings:

- `basenames_l1_compat`: compatibility transport attribution;
- `basenames_execution`: verified-resolution entrypoint and persisted execution.

They do not create Base-side exact-name truth.

## Label Preimage Handling

Preimages can come from:

- registrar events;
- wrapper events;
- resolver/reverse name-bearing events;
- retained name surfaces;
- operational ENS Rainbow import.

Every candidate label must:

1. normalize as a single ENS label;
2. hash back to the retained labelhash.

Preimages improve display and children readability. They do not create authority.

