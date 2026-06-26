# 09 Operational Replay And Repair

Bigname treats replay and repair as explicit production workflows.

## Raw-Fact Normalized Replay

Replay reads canonical raw facts and asks adapters to upsert normalized events and identity state.

It advances only replay-owned cursors:

```text
normalized_replay_cursors
normalized_replay_adapter_checkpoints
normalized_replay_adapter_checkpoint_items
```

Replay does not mutate:

- chain checkpoints;
- raw facts;
- backfill jobs;
- projections directly;
- execution outcomes directly;
- manifests;
- discovery state outside adapter-owned bounded discovery;
- API state.

## Whole-Range Replay

Whole-range replay is the default for full closure.

Why:

- ENSv1 authority combines registry, registrar, wrapper, resolver, and reverse signals;
- per-source cursors can tear histories;
- before_state and resource continuity require ordered history.

## Adapter-Private Checkpoints

Stateful adapters can persist private checkpoint payloads.

Example for `ens_v1_unwrapped_authority`:

- dirty name histories;
- reverse claim histories;
- learned name metadata;
- pending namehash observations;
- migrated-registry markers;
- flushed normalized event counters;
- block-boundary watermark.

These checkpoints are:

- replay resumability state;
- not raw facts;
- not projection readiness;
- not API state;
- version-bound to adapter implementation.

If process exits after normalized events are flushed but before checkpoint save, restart may replay and upsert the same event identities again. Conflicting payload remains a storage mismatch.

## Replay Page Boundaries

Bigname can page replay by candidate-log count while preserving whole-block boundaries.

Rules:

- page size is physical throughput detail;
- a single dense block is not split because it exceeds cap;
- chunk size does not create semantic replay boundary;
- global cursor advances only after adapter finalizes writes through target block.

## Restricted Replay

Allowed only for stateless adapters.

Denied for:

- stateful closure adapters;
- contextual dependency adapters;
- ranges that do not start from closure boundary;
- ranges without stable dependency closure.

Source-scoped/per-target replay is operational repair, not the default semantic model.

## Projection Replay

Bigname has full current projection replay tooling.

Projection replay:

- rebuilds current projection families;
- records completion in `current_projection_replay_status`;
- can skip already completed families when replay version and target match;
- seeds projection apply cursor after full replay with captured change watermark.

Replay markers are not live-readiness signals. Continuous catch-up is owned by:

```text
projection_apply_cursors
projection_invalidations
```

## Repair Framework

Adapter repair is narrower than replay.

It exists for deterministic adapter bugs where existing rows are the same event but some field was encoded incorrectly.

Repair rules:

- must match retained source identity;
- must be constrained to same adapter/chain/logical name/canonical state;
- must document fields it can change;
- must enqueue stale and new projection keys where needed;
- must not silently rewrite unrelated rows.

Examples of repair classes seen in bigname migrations/code:

- ENSv1 registrar renewal resource repair;
- ENSv1 registry boundary repair;
- ENSv1 resolver before-state repair;
- ENSv1 wrapper token before-state repair;
- Basenames primary-claim source repair;
- name-surface normalization repair;
- label preimage backfill and child invalidation.

## Orphaning Repair

Reorg repair and some adapter repairs mark rows orphaned rather than deleting them.

Families with reorg/orphaning indexes include:

- `normalized_events`;
- `token_lineages`;
- `resources`;
- `name_surfaces`;
- `surface_bindings`;
- `chain_lineage`.

This supports fast lookup of rows tied to orphaned block hashes.

## Label Preimage Backfill

Bigname can import ENS Rainbow-style data.

Expected source table shape:

```text
ens_names(hash, name)
```

Import validates:

1. name is one ENS label;
2. normalized label hashes to supplied hash;
3. confidence/provenance is stored.

Then it invalidates affected `children_current` rows.

## Event-Silent Resolver Hydration

Some legacy reverse resolvers can change state without useful events.

Bigname handles this by:

- retaining selected successful direct-call transactions/receipts to configured event-silent resolver addresses;
- copying them to `event_silent_resolver_call_observations`;
- using those observations as projection invalidation triggers;
- rechecking affected current reverse tuples through pinned calls.

It does not decode arbitrary calldata into adapter-owned normalized events.

## Dead Letter Behavior

Projection apply failures are retried.

After repeated deterministic failure, an invalidation moves to:

```text
projection_invalidation_dead_letters
```

Dead letters:

- are durable operator visibility;
- are not claimable;
- do not count as live projection lag;
- can be superseded by fresh invalidation generation.

## Operational Principle

Bigname prefers fail-closed behavior.

Examples:

- provider cannot serve exact historical payload;
- digest mismatches;
- restricted replay lacks closure;
- execution cache identity drifts;
- normalizer remaps a retained name;
- manifest source identity conflicts.

In each case, bigname records explicit failure/unsupported/repair state rather than inventing current state from projections or provider latest.

