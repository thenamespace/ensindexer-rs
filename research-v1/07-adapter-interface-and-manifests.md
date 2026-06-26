# 07 Adapter Interface And Manifests

Adapters are the only place where source-specific protocol logic should live.

Everything downstream should understand normalized events, not raw ABI quirks.

## Adapter Contract

Each adapter should implement this conceptual interface:

```text
source family metadata
raw facts for a block/range
block-pinned helper reads when required
  -> identity updates
  -> normalized events
  -> projection invalidation keys
```

Adapter output must be deterministic for the same input and manifest snapshot.

## Source Definitions

Keep source definitions in Rust/code:

- Alloy event types;
- contract addresses;
- deployment blocks;
- event signatures;
- adapter kind;
- authority scope;
- capability flags;
- discovery rules.

Why code is the source of truth:

- event decoding is type-safe;
- compile-time checks catch ABI mistakes;
- reviews happen in git;
- runtime does not need to parse arbitrary DB-defined behavior.

## Manifest Snapshots In Postgres

Store immutable snapshots of active source definitions in Postgres.

The DB snapshot is not editable config. It is provenance.

Recommended fields:

- `manifest_snapshot_id`;
- source family id;
- chain id;
- version;
- content hash;
- active block range;
- contract list;
- watched event signatures;
- capability flags;
- authority scope;
- serialized snapshot JSON;
- code/git build metadata.

Why this is needed:

- a normalized event can point to the exact rules that produced it;
- old backfills remain explainable after code changes;
- a replay can choose "same snapshot" or "new snapshot" deliberately;
- manifest drift becomes auditable.

## Contract Instances

Do not scatter raw addresses through every table.

Use stable contract instance rows:

```text
contract_instance_id
chain_id
address
source_family
role
first_seen_block
last_seen_block
code_hash
proxy_target
support_status
```

Reasons:

- resolvers are dynamically discovered;
- proxies can change implementation;
- the same address can have different meaning in different source families;
- replays need stable references.

## Discovery

Discovery expands the watch set.

Examples:

- registry `NewResolver` discovers resolver instances;
- ENS v2 `SubregistryUpdated` discovers user registries;
- proxy/code-hash checks change support status;
- manifest changes add or retire contracts.

Discovery should create contract/watch metadata. It should not directly mutate name current state.

## Adapter Replay Classes

Classify every adapter by replay needs.

### Stateless

The raw log is enough.

Examples:

- simple registrar registration;
- resolver record events with all fields present;
- raw label preimage observation.

### Contextual

The raw log needs already-built identity or contract context.

Examples:

- ENS v2 registrar events that need registry resource identity;
- resolver events that need active resolver binding;
- reverse claims needing address/name context.

### Stateful

The adapter needs chronological history.

Examples:

- registry authority transitions;
- wrapper unwrap returning to a prior resource;
- ENS v2 token regeneration/resource continuity.

This classification matters because unsafe partial replay can create wrong semantics.

## Failure Rules

If an adapter cannot prove meaning:

- store a finding or unsupported status;
- keep the raw fact;
- do not guess current state;
- do not emit a misleading normalized event.

Failing closed is better than a fast wrong answer.
