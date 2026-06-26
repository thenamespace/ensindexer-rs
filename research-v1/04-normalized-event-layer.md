# 04 Normalized Event Layer

Normalized events are the shared language between source adapters and projections.

They keep source-specific complexity out of query code.

## Why This Layer Exists

Without normalized events, every projection would need to understand every raw event:

```text
ENSRegistry.NewOwner
BaseRegistry.NewOwner
ENSv2.SubregistryUpdated
NameWrapper.NameWrapped
...
```

That does not scale.

Instead, adapters emit shared meanings:

```text
AuthorityChanged
ResolverChanged
RegistrationGranted
RecordChanged
```

## Event Shape

Every normalized event should carry:

- deterministic event id;
- event kind;
- source family id;
- manifest snapshot id;
- chain id;
- block number;
- block hash;
- transaction hash;
- log index or call snapshot id;
- canonicality state;
- affected name id when known;
- affected resource id when known;
- affected token lineage id when known;
- affected address/resolver/record key when relevant;
- before state when available;
- after state;
- adapter version.

This looks verbose, but it makes replay and audit possible.

## Core Event Kinds

Recommended v1 vocabulary:

```text
NameObserved
LabelPreimageObserved

AuthorityChanged
TtlChanged
ResolverChanged
SubregistryChanged
ParentChanged

RegistrationReserved
RegistrationGranted
RegistrationRenewed
RegistrationReleased
ExpiryChanged

NameWrapped
NameUnwrapped
PermissionChanged
PermissionScopeChanged

TokenResourceLinked
TokenRegenerated
TokenControlChanged

RecordChanged
RecordVersionChanged
ResolverAliasChanged
RecordInventoryChanged

ReverseClaimChanged
PrimaryCandidateChanged
VerifiedPrimaryChanged
VerifiedResolutionChanged

SourceChanged
ContractDiscovered
CoverageChanged
```

Keep this vocabulary small. Add new kinds only when projections need different semantics.

## Deterministic IDs

A normalized event id should be deterministic from:

```text
source family
source event identity
semantic event kind
semantic sub-index
```

For a log:

```text
chain_id:block_hash:tx_hash:log_index:event_kind:sub_index
```

Why:

- replay is idempotent;
- duplicate ingestion is harmless;
- conflicting payloads are obvious;
- repair code can target one semantic event.

## Canonicality

Normalized events should not be deleted on reorg.

They should move between states:

```text
observed
canonical
safe
finalized
orphaned
```

Current projections normally consume canonical/safe/finalized rows and ignore orphaned rows.

## Before And After State

Before/after state is useful when:

- a projection must invalidate both old and new keys;
- repair must explain what changed;
- the API exposes history;
- cache invalidation needs exact dependency keys.

Do not block all ingestion if before state is unavailable. Store `null` and mark confidence/support status.

## What Should Not Be A Normalized Event

Do not create normalized events for:

- API cache writes;
- projection rows;
- operator logs;
- unverified guesses;
- provider latest reads without block hash;
- data that cannot be traced to source evidence.

If it cannot be replayed or explained, it should not mutate semantic state.
