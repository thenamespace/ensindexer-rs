# 14 Contract Event Inventory

This inventory is intentionally semantic. It lists raw event families and the normalized events they should become.

## Registry Events

| Raw event | Sources | Normalized output |
| --- | --- | --- |
| `NewOwner` | ENS v1 registry, Base registry | `NameObserved`, `AuthorityChanged` |
| `Transfer` | ENS v1 registry, Base registry | `AuthorityChanged` |
| `NewResolver` | ENS v1 registry, Base registry | `ResolverChanged`, maybe `ContractDiscovered` |
| `NewTTL` | ENS v1 registry, Base registry | `TtlChanged` |
| `SubregistryUpdated` | ENS v2 registry | `SubregistryChanged`, maybe `ContractDiscovered` |
| `ResolverUpdated` | ENS v2 registry | `ResolverChanged`, maybe `ContractDiscovered` |
| `ParentUpdated` | ENS v2 registry/root | `ParentChanged` |

## Registration Events

| Raw event | Sources | Normalized output |
| --- | --- | --- |
| `NameRegistered` | ENS v1 registrar/controller, ENS v2 registrar, Base registrar | `RegistrationGranted`, `LabelPreimageObserved` |
| `NameRenewed` | ENS v1 registrar/controller, ENS v2 registrar, Base registrar | `RegistrationRenewed`, maybe `LabelPreimageObserved` |
| `LabelRegistered` | ENS v2 registry/root | `RegistrationGranted`, `LabelPreimageObserved` |
| `LabelReserved` | ENS v2 registry/root | `RegistrationReserved`, `LabelPreimageObserved` |
| `LabelUnregistered` | ENS v2 registry/root | `RegistrationReleased` |
| `ExpiryUpdated` | ENS v2 registry/root | `ExpiryChanged` |

## Token / Resource Events

| Raw event | Sources | Normalized output |
| --- | --- | --- |
| ERC-721 `Transfer` | registrar tokens | `TokenControlChanged` |
| ERC-1155 `TransferSingle` | NameWrapper | `TokenControlChanged` |
| ERC-1155 `TransferBatch` | NameWrapper | `TokenControlChanged` |
| `TokenResource` | ENS v2 registry/root | `TokenResourceLinked` |
| `TokenRegenerated` | ENS v2 registry/root | `TokenRegenerated` |

## Wrapper Events

| Raw event | Normalized output |
| --- | --- |
| `NameWrapped` | `NameWrapped`, `LabelPreimageObserved`, `PermissionScopeChanged`, `TokenControlChanged` |
| `NameUnwrapped` | `NameUnwrapped` |
| `FusesSet` | `PermissionScopeChanged` |
| `ExpiryExtended` | `ExpiryChanged` |

## Resolver Record Events

| Raw event | Normalized output |
| --- | --- |
| `AddrChanged` | `RecordChanged` |
| `AddressChanged` | `RecordChanged` |
| `TextChanged` | `RecordChanged` |
| `ContenthashChanged` | `RecordChanged` |
| `ContentChanged` | `RecordChanged` |
| `ABIChanged` | `RecordChanged` |
| `InterfaceChanged` | `RecordChanged` |
| `DNSRecordChanged` | `RecordChanged` |
| `DNSRecordDeleted` | `RecordChanged` |
| `DNSZonehashChanged` | `RecordChanged` |
| `NameChanged` | `RecordChanged`, maybe `PrimaryCandidateChanged` |
| `VersionChanged` | `RecordVersionChanged` |
| `AliasChanged` | `ResolverAliasChanged` |

## Permission Events

| Raw event | Sources | Normalized output |
| --- | --- | --- |
| `EACRolesChanged` | ENS v2 registry/root/resolver | `PermissionChanged` |
| wrapper fuse updates | ENS NameWrapper | `PermissionScopeChanged` |
| ownership transfer | registry/registrar/wrapper | `PermissionChanged` derived by projection |

## Reverse / Primary Events

| Raw event / observation | Source | Normalized output |
| --- | --- | --- |
| reverse registrar claim | ENS reverse registrar | `ReverseClaimChanged` |
| `NameChanged` on reverse node | resolver | `PrimaryCandidateChanged`, `RecordChanged` |
| `NameForAddrChanged` | Base primary contract | `ReverseClaimChanged`, `PrimaryCandidateChanged`, `RecordChanged` |
| block-pinned primary verification | execution | `VerifiedPrimaryChanged` |

## Manifest / Discovery Events

| Observation | Normalized output |
| --- | --- |
| source definition activated | `SourceChanged` |
| contract instance discovered | `ContractDiscovered` |
| support status changed | `CoverageChanged` |
| code/proxy drift detected | finding, not current name truth |

## Important Rule

One raw event can produce multiple normalized events.

Example:

```text
NameWrapped
  -> NameWrapped
  -> LabelPreimageObserved
  -> PermissionScopeChanged
  -> TokenControlChanged
```

This is fine. The normalized event id must include a semantic sub-index so replay is deterministic.
