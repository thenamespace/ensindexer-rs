# 09 Protocol-Specific Mappings

This file explains how major source families map into the global tree.

The rule is always the same:

```text
source event -> adapter -> normalized event -> global name/resource state
```

## ENS v1 Registry

Raw events:

- `NewOwner`;
- `Transfer`;
- `NewResolver`;
- `NewTTL`.

Outputs:

- `NameObserved`;
- `AuthorityChanged`;
- `ResolverChanged`;
- `TtlChanged`;
- `LabelPreimageObserved` when label text is available from context.

Notes:

- old and current registries need migration guards;
- registry-only subnames may exist without registrar resources;
- hash-only children are valid until labels are healed.

## ENS v1 Registrar

Raw events:

- `NameRegistered`;
- `NameRenewed`;
- ERC-721 `Transfer`.

Outputs:

- `RegistrationGranted`;
- `RegistrationRenewed`;
- `TokenControlChanged`;
- `LabelPreimageObserved`.

Notes:

- `.eth` 2LD registrations create lease resources;
- renewal extends resource expiry;
- transfer changes token holder, not name identity;
- lapse plus re-registration creates a new resource lineage.

## ENS NameWrapper

Raw events:

- `NameWrapped`;
- `NameUnwrapped`;
- `FusesSet`;
- `ExpiryExtended`;
- ERC-1155 transfers.

Outputs:

- `NameWrapped`;
- `NameUnwrapped`;
- `PermissionScopeChanged`;
- `ExpiryChanged`;
- `TokenControlChanged`;
- `LabelPreimageObserved`.

Notes:

- wrapper control can override visible effective authority;
- burned fuses must mask permissions;
- unwrap can return a name to registrar or registry authority.

## ENS v1 Resolver

Raw events:

- address records;
- text records;
- contenthash;
- ABI/interface/DNS records;
- name records;
- version changes.

Outputs:

- `RecordChanged`;
- `RecordVersionChanged`;
- `ResolverAliasChanged` where supported.

Notes:

- resolver support is profile-gated;
- event-silent resolvers require block-pinned execution/hydration;
- resolver version changes are record boundaries.

## ENS Reverse

Raw observations:

- reverse registrar claims;
- reverse resolver `NameChanged`;
- event-silent reverse reads.

Outputs:

- `ReverseClaimChanged`;
- `PrimaryCandidateChanged`;
- `RecordChanged`.

Notes:

- declared reverse claim is not the same as verified primary;
- verified primary requires forward-confirmation/execution logic.

## ENS v2 Registry

Raw events:

- `LabelRegistered`;
- `LabelReserved`;
- `LabelUnregistered`;
- `ExpiryUpdated`;
- `SubregistryUpdated`;
- `ResolverUpdated`;
- `TokenResource`;
- `TokenRegenerated`;
- `ParentUpdated`;
- `EACRolesChanged`.

Outputs:

- `RegistrationGranted`;
- `RegistrationReserved`;
- `RegistrationReleased`;
- `ExpiryChanged`;
- `SubregistryChanged`;
- `ResolverChanged`;
- `TokenResourceLinked`;
- `TokenRegenerated`;
- `ParentChanged`;
- `PermissionChanged`.

Notes:

- resource identity is first-class;
- token id can change while resource identity remains stable;
- role changes can affect root-level and resource-level permissions differently;
- enrichment reads must be block-hash pinned.

## ENS v2 Registrar

Raw events:

- `NameRegistered`;
- `NameRenewed`.

Outputs:

- `RegistrationGranted` or `RegistrarNameRegistered`;
- `RegistrationRenewed`;
- `LabelPreimageObserved`.

Notes:

- registrar events need registry resource context;
- commercial/payment fields can remain in payload/history, not current API core.

## ENS v2 Resolver

Raw events:

- `AddressChanged`;
- `TextChanged`;
- `ContenthashChanged`;
- `NameChanged`;
- `VersionChanged`;
- `AliasChanged`;
- named resource events;
- `EACRolesChanged`.

Outputs:

- `RecordChanged`;
- `RecordVersionChanged`;
- `ResolverAliasChanged`;
- `PermissionChanged`.

## Basenames / L2-Managed `.base.eth`

Raw sources:

- Base registry;
- Base registrar;
- Base resolver;
- Base reverse/primary contracts;
- L1 compatibility resolver/execution path.

Outputs:

- same normalized events as ENS v1/v2 where semantics match.

Important:

`alice.base.eth` remains a child of `base.eth`.

The authority source is Base. The public identity is still the global name.

## Future L2 Sources

For Linea/Celo/other chains, repeat the same pattern:

1. identify the root/suffix they are allowed to manage;
2. define source-family adapters;
3. emit shared normalized events;
4. project into the same current tables;
5. keep chain/source provenance for conflicts and audit.
