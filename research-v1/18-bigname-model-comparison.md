# 18 Bigname Model Comparison

Bigname is useful because it makes production indexing concerns explicit:

- raw facts;
- normalized events;
- resources;
- token lineages;
- projections;
- execution traces;
- reorg-aware canonicality;
- repair tooling.

We should copy many internal ideas, but not its public product split.

## What Bigname Optimizes For

Bigname exposes names through separate public product buckets.

That gives bigname clean product boundaries:

- ENS names in one bucket;
- Basenames-issued names in another bucket;
- source-specific routing at the API layer.

This can be good for a product that wants Basenames to be a separate public surface.

It is not the model we want for `ensindexer-rs`.

## Our Model

Our model is the global tree:

```text
base.eth
  -> alice.base.eth
```

The source may be Base, but the name is still a child under `base.eth`.

Stable identity:

```text
name_id = namehash(normalized_name)
```

Source provenance:

```text
source_family_id
chain_id
contract_instance_id
manifest_snapshot_id
authority_scope
```

Those provenance fields explain where the fact came from without splitting the product model.

## What To Copy From Bigname

### Raw -> Normalized -> Projection

This separation is correct and should be central.

```text
raw facts
  -> normalized events
  -> identity
  -> projections
```

### Block Hash Canonicality

Block number alone is not enough.

Copy the idea that chain-derived rows carry block hash and canonicality state.

### Resource Identity

Copy the resource split.

Public name text is not the same as the authority object behind it.

### Token Lineage

Copy token lineage.

It is needed for wrapper tokens, registrar tokens, and ENS v2 token regeneration.

### Manifest Provenance

Copy immutable manifest snapshots in DB.

Do not copy DB-as-runtime-config.

### Projection Invalidations

Copy key-scoped invalidation queues and dead letters.

Projection failures must become visible.

### Execution Traces

Copy durable traces/outcomes for verified resolution and primary-name verification.

Cache outcomes are reusable. Traces are audit.

## What Not To Copy

Do not copy:

- separate public product buckets;
- source-specific current tables;
- API routes that make source family part of public identity;
- every operational side table before it has a clear purpose.

## Why Our Model Is Better For This Indexer

It matches ENS/DNS semantics:

```text
alice.base.eth is under base.eth
```

It is better for:

- subname search;
- tree traversal;
- future L2-managed suffixes;
- GraphQL/subgraph-style compatibility;
- users comparing records across sources;
- avoiding schema churn when new chains are added.

## What Becomes Harder

The global tree requires stricter authority rules.

Hard cases:

- child observed before parent;
- multiple sources writing the same name;
- L2 authority under an L1 parent;
- resolver/primary state crossing chains;
- future source upgrades changing supported behavior.

The answer is not product splitting. The answer is:

- explicit source-family provenance;
- authority scopes;
- canonicality state;
- support status;
- conflict findings;
- projection rules that can be tested.

## Final Decision

Use bigname's internal architecture discipline.

Do not use bigname's public product split.

The indexer should feel like one ENS graph, regardless of which chain emitted the latest fact.
