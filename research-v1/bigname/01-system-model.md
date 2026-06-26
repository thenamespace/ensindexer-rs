# 01 System Model

Bigname is a versioned indexing and read platform for ENS v1, ENS v2, and Basenames.

It exposes native REST APIs. It does not preserve ENS subgraph GraphQL shapes. The system is designed around point-in-time reads, provenance, coverage, finality, consistency, and replay.

## Public Product Model

Bigname uses public namespaces.

The reviewed architecture defines exactly:

```text
ens
basenames
```

`ens` includes ENS v1 and ENS v2 as internal authority epochs.

`basenames` includes Basenames-issued `*.base.eth` names on Base.

The special split:

```text
base.eth              -> ens:base.eth
alice.base.eth        -> basenames:alice.base.eth
```

The namespace is assigned before `logical_name_id` is minted. The rule order is:

```text
exact_name
suffix
authority_root
```

Initial policy:

```text
exact base.eth      -> ens
suffix *.base.eth   -> basenames
other ENS surfaces  -> ens
```

This is the biggest public-model difference from our indexer. Bigname chooses product namespaces. We choose one global tree.

## Read Contract

Every public answer carries or can explain:

```text
declared_state
verified_state
provenance
coverage
chain_positions
consistency
last_updated
```

Routes accept:

```text
namespace
name
address
coin_type
at
chain_positions
consistency=head|safe|finalized
mode=declared|verified|both
include
pagination
```

`at` and `chain_positions` are mutually exclusive. `at` selects by timestamp. `chain_positions` pins exact per-chain block positions.

## Identity Anchors

Bigname's whole architecture depends on separating four identities.

### `logical_name_id`

Public surface identity:

```text
<namespace>:<normalized_name>
```

Examples:

```text
ens:vitalik.eth
ens:base.eth
basenames:alice.base.eth
```

It survives resource rotation, token regeneration, lapse, re-registration, wrapper movement, and resolver alias behavior.

### `resource_id`

Opaque UUID for the backing authority object.

Examples:

- ENS v1 registry-only authority for a node.
- ENS v1 registrar lease.
- ENS v1 wrapper-backed authority.
- ENS v2 EAC resource.
- Basenames Base-side authority object.

Permissions and control are resource-first. Exact name lookup is surface-first.

### `token_lineage_id`

Opaque UUID for tokenized ownership history.

It survives token ID changes when the backing authority stays the same.

Examples:

- ENSv2 `TokenRegenerated(oldTokenId,newTokenId)` preserves token lineage.
- ENS v1 registrar transfer preserves token lineage.
- Wrap from registrar to wrapper rotates token lineage.

### `contract_instance_id`

Opaque UUID for registry, registrar, resolver, wrapper, proxy, implementation, and transport contracts.

One admitted address on one chain maps to one contract instance across manifest/discovery epochs. A proxy keeps identity across implementation changes. A watched contract address replacement creates a new instance and may link through a migration edge.

## Name Surface And Resource Binding

Bigname separates public surface from authority object.

```text
NameSurface(logical_name_id)
SurfaceBinding(logical_name_id -> resource_id over time)
Resource(resource_id)
TokenLineage(resource_id -> token history)
```

`NameSurface` stores:

- namespace;
- logical name id;
- input name;
- canonical display name;
- normalized name;
- DNS-encoded name;
- namehash;
- labelhash path;
- normalizer version;
- normalization warnings/errors;
- chain/block/provenance.

`SurfaceBinding` stores:

- surface binding id;
- logical name id;
- resource id;
- binding kind;
- active from;
- active to;
- provenance;
- canonicality state.

Binding kinds:

```text
declared_registry_path
linked_subregistry_path
resolver_alias_path
observed_wildcard_path
migration_rebind
observed_only
```

## High-Level Pipeline

```text
manifest profile selected
  -> manifests loaded and persisted
  -> contract instances admitted
  -> discovery edges and watch plan built
  -> chain blocks reconciled by hash
  -> selected raw facts stored
  -> adapters decode raw facts
  -> identity rows written
  -> normalized_events inserted
  -> projection change feed populated
  -> projection invalidations queued
  -> current projections rebuilt
  -> API reads projections and execution cache
```

## Immutable Versus Rebuildable

Immutable or append-only:

- block lineage;
- selected raw facts;
- manifests;
- discovery observations;
- normalized events;
- normalization/preimage observations;
- execution traces and steps;
- selected call snapshots;
- payload cache metadata.

Rebuildable:

- current name profiles;
- address-to-name lists;
- child lists;
- permissions current;
- resolver current;
- record inventory;
- primary name current;
- execution cache outcomes;
- coverage snapshots.

## Bigname's Central Production Choice

Bigname never lets API reads reconstruct state directly from raw logs.

The API reads projections and execution outputs. Raw facts and normalized events are replay/audit inputs. This is why bigname has more tables than a subgraph: it makes Graph Node's hidden infrastructure explicit in Postgres and workers.

