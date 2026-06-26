# 11 Manifest Event Catalog

This file is the source-family event checklist extracted from bigname manifests.

The adapter document explains how event families behave. This catalog is the explicit manifest-level inventory: which raw events bigname declares for each source family and which normalized event kinds those raw events are expected to produce.

## How To Read This Catalog

Bigname does not treat ABI events as product API shapes.

The path is:

```text
manifest ABI event
  -> raw log watch plan
  -> source-family adapter
  -> one or more normalized_events
  -> projection invalidations
  -> current read model
```

Some manifests list multiple versions of the same source family. A later manifest version can add events or update ABI fragments without changing the downstream normalized event vocabulary.

## ENS v1 Registry

Source family:

```text
ens_v1_registry_l1
```

Manifests:

```text
mainnet/ethereum/ens/ens_v1_registry_l1/v1.toml
mainnet/ethereum/ens/ens_v1_registry_l1/v2.toml
mainnet/ethereum/ens/ens_v1_registry_l1/v3.toml
```

Declared contract role:

```text
ENSRegistry
```

Active event declarations in the current manifest version:

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `NewOwner` | `NewOwner(bytes32 node, bytes32 indexed label, address owner)` | `AuthorityTransferred` | Creates or updates a child registry authority edge. |
| `Transfer` | `Transfer(bytes32 indexed node, address owner)` | `AuthorityTransferred` | Updates owner for an existing node. |
| `NewResolver` | `NewResolver(bytes32 indexed node, address resolver)` | `ResolverChanged` | Updates resolver pointer and can discover resolver contracts. |
| `NewTTL` | `NewTTL(bytes32 indexed node, uint64 ttl)` | `AuthorityEpochChanged` | Updates TTL/authority epoch information for exact-name state. |

Important behavior:

- The old registry and current registry are both admissible historical sources.
- Once current registry activity proves a node migrated, older non-root registry writes are retained but suppressed from current semantics.
- Root `NewResolver` from the old registry is the exception because it can affect root resolver discovery.
- Registry events often lack readable labels. They can create hashed child edges before `label_preimages` learns the display label.

## ENS v1 Registrar

Source family:

```text
ens_v1_registrar_l1
```

Declared contract role:

```text
ETHRegistrar
```

Watched contracts include BaseRegistrar and controller generations.

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `Transfer` | `Transfer(address indexed from, address indexed to, uint256 indexed tokenId)` | `TokenControlTransferred` | ERC-721 holder movement for registrar token lineage. |
| `NameRegistered` | `NameRegistered(string name, bytes32 indexed label, address indexed owner, uint256 cost, uint256 expires)` | `RegistrationGranted`, `PreimageObserved` | Legacy controller registration with readable label and expiry. |
| `NameRegistered` | `NameRegistered(string name, bytes32 indexed label, address indexed owner, uint256 baseCost, uint256 premium, uint256 expires)` | `RegistrationGranted` | Newer controller registration. |
| `NameRegistered` | `NameRegistered(string name, bytes32 indexed label, address indexed owner, uint256 baseCost, uint256 premium, uint256 expires, bytes32 referrer)` | `RegistrationGranted` | Referrer-bearing controller registration. |
| `NameRenewed` | `NameRenewed(string name, bytes32 indexed label, uint256 cost, uint256 expires)` | `RegistrationRenewed`, `PreimageObserved` | Renewal plus possible label preimage. |
| `NameRenewed` | `NameRenewed(string name, bytes32 indexed label, uint256 cost, uint256 expires, bytes32 referrer)` | `RegistrationRenewed` | Referrer-bearing renewal. |

Important behavior:

- Registrar registrations create or continue lease-backed resources.
- Renewals extend the current lease resource.
- Transfers move token control but do not by themselves create a new public surface.
- Full lapse plus later registration creates a new resource/token lineage for the same logical name.
- Same-transaction setup and historical repair code exists because registrar and registry facts can arrive in the same block/transaction with subtle before-state boundaries.

## ENS v1 NameWrapper

Source family:

```text
ens_v1_wrapper_l1
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `NameWrapped` | `NameWrapped(bytes32 indexed node, bytes name, address owner, uint32 fuses, uint64 expiry)` | `WrapperNameWrapped`, `PreimageObserved` | Wraps a name, admits readable DNS-encoded name, creates wrapper authority context. |
| `NameUnwrapped` | `NameUnwrapped(bytes32 indexed node, address owner)` | `WrapperNameUnwrapped` | Removes wrapper authority and may return authority to registrar/registry context. |
| `FusesSet` | `FusesSet(bytes32 indexed node, uint32 fuses)` | `PermissionScopeChanged` | Updates effective permission masks. |
| `ExpiryExtended` | `ExpiryExtended(bytes32 indexed node, uint64 expiry)` | `ExpiryChanged` | Extends wrapper expiry. |
| `TransferSingle` | `TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value)` | `TokenControlTransferred` | ERC-1155 single-token control movement. |
| `TransferBatch` | `TransferBatch(address indexed operator, address indexed from, address indexed to, uint256[] ids, uint256[] values)` | `TokenControlTransferred` | ERC-1155 batch control movement. |

Important behavior:

- Wrapper events are resource transitions, not alternate names.
- Fuses are projected into effective permissions. A capability can exist in historical ownership but be masked from current powers by burned fuses.
- Wrapped and unwrapped resources can bind to the same `logical_name_id` at different time ranges.

## ENS v1 Resolver

Source family:

```text
ens_v1_resolver_l1
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `ABIChanged` | `ABIChanged(bytes32 indexed node, uint256 indexed contentType)` | `RecordChanged` | ABI record inventory/value changed. |
| `AddrChanged` | `AddrChanged(bytes32 indexed node, address a)` | `RecordChanged` | Coin type 60 address changed. |
| `AddressChanged` | `AddressChanged(bytes32 indexed node, uint256 coinType, bytes newAddress)` | `RecordChanged` | Multicoin address changed. |
| `ContentChanged` | `ContentChanged(bytes32 indexed node, bytes32 hash)` | `RecordChanged` | Legacy content record changed. |
| `ContenthashChanged` | `ContenthashChanged(bytes32 indexed node, bytes hash)` | `RecordChanged` | Contenthash record changed. |
| `DNSRecordChanged` | `DNSRecordChanged(bytes32 indexed node, bytes name, uint16 resource, bytes record)` | `RecordChanged` | DNS record upsert/update. |
| `DNSRecordDeleted` | `DNSRecordDeleted(bytes32 indexed node, bytes name, uint16 resource)` | `RecordChanged` | DNS record deletion. |
| `DNSZonehashChanged` | `DNSZonehashChanged(bytes32 indexed node, bytes lastzonehash, bytes zonehash)` | `RecordChanged` | DNS zonehash boundary changed. |
| `DataChanged` | `DataChanged(bytes32 indexed node, string indexed indexedKey, string key, bytes indexed indexedData)` | `RecordChanged` | Arbitrary data record observation. |
| `InterfaceChanged` | `InterfaceChanged(bytes32 indexed node, bytes4 indexed interfaceID, address implementer)` | `RecordChanged` | Interface implementer record changed. |
| `NameChanged` | `NameChanged(bytes32 indexed node, string name)` | `RecordChanged` | Reverse/name record changed. |
| `TextChanged` | `TextChanged(bytes32 indexed node, string indexed indexedKey, string key)` | `RecordChanged` | Legacy text-key change without value. |
| `TextChanged` | `TextChanged(bytes32 indexed node, string indexed indexedKey, string key, string value)` | `RecordChanged` | Text record changed with value. |
| `VersionChanged` | `VersionChanged(bytes32 indexed node, uint64 newVersion)` | `RecordVersionChanged` | Resolver version boundary. |

Important behavior:

- A resolver address observed through registry `NewResolver` is not automatically fully supported.
- Manifest/profile admission decides whether the resolver implementation is compatible enough for complete record-family coverage.
- Resolver event rows are declared state. Verified resolution reads go through execution traces/outcomes, not resolver events alone.
- Version boundaries affect record inventory and cache validity.

## ENS v1 Reverse

Source family:

```text
ens_v1_reverse_l1
```

The reverse adapter is not only a simple manifest ABI mapping. It consumes reverse-registrar and resolver/name-record observations to produce declared reverse state.

Typical normalized output:

| Observation | Normalized events | Meaning |
| --- | --- | --- |
| Reverse registrar claim | `ReverseChanged` | Address-to-name claim anchor changed. |
| Reverse resolver `NameChanged` | `ReverseChanged`, `RecordChanged` | Name record for reverse node changed. |
| Event-silent reverse state discovered by retained call observation | projection hydration, not adapter-owned normalized event | Supports legacy reverse resolvers that do not emit complete events. |

Important behavior:

- Declared reverse claim and verified primary resolution are separate.
- Verified primary-name answers are execution outcomes.
- Event-silent hydration updates projection state through a repair/hydration path, not by inventing raw logs.

## ENS v2 Root And Registry

Source families:

```text
ens_v2_root_l1
ens_v2_registry_l1
```

Declared contract roles:

```text
RootRegistry
ETHRegistry
```

Both root and registry manifests declare the same event vocabulary.

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `LabelRegistered` | `LabelRegistered(uint256 indexed tokenId, bytes32 indexed labelHash, string label, address owner, uint64 expiry, address indexed sender)` | `RegistrationGranted` | Registers a label and creates/updates surface/resource identity. |
| `LabelReserved` | `LabelReserved(uint256 indexed tokenId, bytes32 indexed labelHash, string label, uint64 expiry, address indexed sender)` | `RegistrationReserved` | Reserves a label without normal ownership semantics. |
| `LabelUnregistered` | `LabelUnregistered(uint256 indexed tokenId, address indexed sender)` | `RegistrationReleased` | Releases label/resource binding. |
| `ExpiryUpdated` | `ExpiryUpdated(uint256 indexed tokenId, uint64 indexed newExpiry, address indexed sender)` | `ExpiryChanged` | Updates expiry. |
| `SubregistryUpdated` | `SubregistryUpdated(uint256 indexed tokenId, address indexed subregistry, address indexed sender)` | `SubregistryChanged` | Changes delegated registry endpoint; can update discovery graph. |
| `ResolverUpdated` | `ResolverUpdated(uint256 indexed tokenId, address indexed resolver, address indexed sender)` | `ResolverChanged` | Changes resolver endpoint; can update discovery graph. |
| `TokenResource` | `TokenResource(uint256 indexed tokenId, uint256 indexed resource)` | `TokenResourceLinked` | Links token id to stable EAC resource id. |
| `EACRolesChanged` | `EACRolesChanged(uint256 indexed resource, address indexed account, uint256 oldRoleBitmap, uint256 newRoleBitmap)` | `PermissionChanged`, `RootPermissionChanged` | Role bitmap changed. Root registry can create root-level fallback permission. |
| `TokenRegenerated` | `TokenRegenerated(uint256 indexed oldTokenId, uint256 indexed newTokenId)` | `TokenRegenerated` | Token id changed while resource continuity remains stable. |
| `ParentUpdated` | `ParentUpdated(address indexed parent, string label, address indexed sender)` | `ParentChanged` | Parent graph changed. |

Important behavior:

- ENSv2 uses resource identity more explicitly than ENS v1.
- `TokenResource` and block-pinned enrichment calls let bigname link token ids to stable resources.
- `TokenRegenerated` does not mean a new name or new resource.
- Root permission and resource-specific permission are distinct because root fallback can authorize downstream resource actions.
- Number-only state reads are unsafe for historical correction unless block-hash pinned.

## ENS v2 Registrar

Source family:

```text
ens_v2_registrar_l1
```

Declared contract role:

```text
ETHRegistrar
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `NameRegistered` | `NameRegistered(uint256 indexed tokenId, string label, address owner, address subregistry, address resolver, uint64 duration, address paymentToken, bytes32 referrer, uint256 base, uint256 premium)` | `RegistrarNameRegistered` | Registrar-local registration intent and commercial details. |
| `NameRenewed` | `NameRenewed(uint256 indexed tokenId, string label, uint64 duration, uint64 newExpiry, address paymentToken, bytes32 referrer, uint256 base)` | `RegistrationRenewed` | Renewal fact for existing resource. |

Important behavior:

- Registrar events need registry/resource context before they can be fully projected.
- The adapter is contextual, not purely stateless.
- Registry output is the stronger source for public surface/resource identity.

## ENS v2 Resolver

Source family:

```text
ens_v2_resolver_l1
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `AddressChanged` | `AddressChanged(bytes32 indexed node, uint256 coinType, bytes newAddress)` | `RecordChanged` | Multicoin address record changed. |
| `TextChanged` | `TextChanged(bytes32 indexed node, string indexed indexedKey, string key, string value)` | `RecordChanged` | Text record changed. |
| `ContenthashChanged` | `ContenthashChanged(bytes32 indexed node, bytes hash)` | `RecordChanged` | Contenthash changed. |
| `NameChanged` | `NameChanged(bytes32 indexed node, string name)` | `RecordChanged` | Name/reverse-style record changed. |
| `VersionChanged` | `VersionChanged(bytes32 indexed node, uint64 newVersion)` | `RecordVersionChanged` | Resolver version boundary. |
| `AliasChanged` | `AliasChanged(bytes indexed indexedFromName, bytes indexed indexedToName, bytes fromName, bytes toName)` | `AliasChanged` | Resolver alias path changed. |
| `NamedResource` | `NamedResource(uint256 indexed resource, bytes name)` | `RecordChanged`, `PermissionChanged` | Named resource association changed. |
| `NamedTextResource` | `NamedTextResource(uint256 indexed resource, bytes name, bytes32 indexed keyHash, string key)` | `RecordChanged`, `PermissionChanged` | Named text-resource association changed. |
| `NamedAddrResource` | `NamedAddrResource(uint256 indexed resource, bytes name, uint256 indexed coinType)` | `RecordChanged`, `PermissionChanged` | Named address-resource association changed. |
| `EACRolesChanged` | `EACRolesChanged(uint256 indexed resource, address indexed account, uint256 oldRoleBitmap, uint256 newRoleBitmap)` | `PermissionChanged` | Resolver-scoped permissions changed. |

Important behavior:

- Resolver rows need stable registry/resource links.
- Named-resource events affect both records and effective permission surfaces.
- Alias changes are separate from ordinary record changes because they can redirect resolver path semantics.

## Basenames Registry

Source family:

```text
basenames_base_registry
```

Declared contract role:

```text
BasenamesRegistry
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `NewOwner` | `NewOwner(bytes32 indexed node, bytes32 indexed label, address owner)` | `AuthorityTransferred` | Base-side registry authority changed. |
| `Transfer` | `Transfer(bytes32 indexed node, address owner)` | `AuthorityTransferred` | Owner changed for existing Base registry node. |
| `NewResolver` | `NewResolver(bytes32 indexed node, address resolver)` | `ResolverChanged` | Base-side resolver pointer changed and can discover resolver. |
| `NewTTL` | `NewTTL(bytes32 indexed node, uint64 ttl)` | `AuthorityEpochChanged` | TTL/authority epoch changed. |

Important behavior:

- Bigname assigns these facts to the `basenames` namespace.
- In our global-tree model, these should be interpreted as facts about `*.base.eth` subnames.
- The event mechanics still map cleanly onto normalized authority/resolver/TTL events.

## Basenames Registrar

Source family:

```text
basenames_base_registrar
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `NameRegistered` | `NameRegistered(string name, bytes32 indexed label, address indexed owner, uint256 expires)` | `RegistrationGranted` | Basename registration with readable name and expiry. |
| `NameRenewed` | `NameRenewed(string name, bytes32 indexed label, uint256 expires)` | `RegistrationRenewed` | Basename renewal. |
| `Transfer` | `Transfer(address indexed from, address indexed to, uint256 indexed tokenId)` | `TokenControlTransferred` | ERC-721 holder movement. |

Important behavior:

- Registration grants exact-name and lease-style state.
- Token transfer changes controller/holder relation, not the name surface itself.

## Basenames Resolver

Source family:

```text
basenames_base_resolver
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `AddrChanged` | `AddrChanged(bytes32 indexed node, address a)` | `RecordChanged` | Coin type 60 address changed. |
| `AddressChanged` | `AddressChanged(bytes32 indexed node, uint256 coinType, bytes newAddress)` | `RecordChanged` | Multicoin address changed. |
| `NameChanged` | `NameChanged(bytes32 indexed node, string name)` | `RecordChanged` | Name/reverse-style record changed. |
| `TextChanged` | `TextChanged(bytes32 indexed node, string indexed indexedKey, string key, string value)` | `RecordChanged` | Text record changed. |
| `VersionChanged` | `VersionChanged(bytes32 indexed node, uint64 newVersion)` | `RecordVersionChanged` | Resolver version boundary. |

Important behavior:

- Resolver support is profile-gated.
- These are Base-side declared record facts.
- Verified Basenames resolution is handled by the execution source family, not by treating resolver events as proof of resolution.

## Basenames Primary

Source family:

```text
basenames_base_primary
```

| Raw event | Fragment shape | Normalized events | Meaning |
| --- | --- | --- | --- |
| `NameForAddrChanged` | `NameForAddrChanged(address indexed addr, string name)` | `ReverseChanged`, `RecordChanged` | Declared address-to-name claim changed. |

Important behavior:

- This is claim-side intake.
- It does not own exact-name authority.
- It does not replace verified primary-name execution.
- It can invalidate primary-name projections and execution outcomes.

## Basenames L1 Compatibility And Execution

Source families:

```text
basenames_l1_compat
basenames_execution
```

These families are not ordinary event catalogs in the same way as registry/registrar/resolver families.

They define:

- admitted transport endpoints;
- verified execution entrypoints;
- cache dependency scope;
- support class for Basenames resolution through the L1 Resolver.

The same L1 Resolver address can be relevant to both families, but they have different ownership:

| Source family | Owns | Does not own |
| --- | --- | --- |
| `basenames_l1_compat` | transport attribution and compatibility | verified result truth |
| `basenames_execution` | execution traces and reusable verified outcomes | Base-side exact-name authority |

## Manifest-Generated Semantic Events

Source family:

```text
manifest_normalized_events
```

This adapter emits semantic events from manifest/discovery state rather than from protocol logs.

Typical normalized events:

| Condition | Normalized event | Meaning |
| --- | --- | --- |
| manifest profile/version becomes active | `SourceManifestUpdated` | Source admission rules changed. |
| contract/proxy/code drift is detected | `MetadataChanged` or manifest alert event kind | Source metadata changed or needs operator review. |
| new contract instance is admitted | `ContractDiscovered` | Watch/discovery graph changed. |
| capability support changes | `CoverageChanged` | API support and projection coverage can change. |

Important behavior:

- Manifest changes can invalidate projections and execution cache.
- Source admission is treated as data, not hidden config.
- Drift observations are audit facts. They do not silently change protocol truth.

## Block-Derived Label Preimage Events

Source family:

```text
block_derived_normalized_events
```

This adapter scans retained logs for name-bearing payloads and emits label preimage observations when the candidate label hashes back to a retained labelhash.

Typical sources:

- ENS v1 registrar `NameRegistered` and `NameRenewed`;
- ENS v1 wrapper `NameWrapped`;
- ENS v2 registry `LabelRegistered` and `LabelReserved`;
- ENS v2 registrar `NameRegistered` and `NameRenewed`;
- Basenames registrar `NameRegistered` and `NameRenewed`;
- resolver/reverse events that contain readable names.

Output:

```text
PreimageObserved
```

Important behavior:

- Preimages improve readability.
- Preimages can invalidate `children_current` rows and exact-name display rows.
- Preimages do not create authority or ownership.
- Invalid, multi-label, or hash-mismatched candidates are ignored.

## Normalized Event Vocabulary Seen In Manifests

The manifest-declared vocabulary includes:

```text
AliasChanged
AuthorityEpochChanged
AuthorityTransferred
ContractDiscovered
CoverageChanged
ExpiryChanged
MetadataChanged
ParentChanged
PermissionChanged
PermissionScopeChanged
PreimageObserved
RecordChanged
RecordVersionChanged
RegistrationGranted
RegistrationReleased
RegistrationRenewed
RegistrationReserved
RegistrarNameRegistered
ResolverChanged
ReverseChanged
RootPermissionChanged
SourceManifestUpdated
SubregistryChanged
TokenControlTransferred
TokenRegenerated
TokenResourceLinked
WrapperNameUnwrapped
WrapperNameWrapped
```

The runtime/domain vocabulary is slightly broader because execution and projection repair can also use events such as verified resolution, primary-name verification, invalidation, inventory observations, and migration/coverage transitions.
