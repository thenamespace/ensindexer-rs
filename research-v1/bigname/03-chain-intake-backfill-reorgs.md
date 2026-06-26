# 03 Chain Intake, Backfill, Reorgs

Bigname's chain intake principle:

```text
subscriptions, filters, and polling are hints;
block-hash-reconciled raw facts are truth.
```

## Profile Boundary

A runtime selects one manifest profile root.

Examples:

```text
manifests/mainnet
manifests/sepolia
```

Mainnet and Sepolia do not share:

- canonical corpus;
- checkpoints;
- projection state;
- watch plan;
- discovery graph.

A runtime must not ingest across profiles.

## Provider Requirements

For each selected chain, bigname wants providers that can support:

- block fetch by hash;
- block fetch by number/tag;
- exact block-scoped log fetch;
- block-scoped receipt fetch or bounded fallback;
- code/call reads at pinned chain positions;
- safe and finalized head visibility.

Production correctness depends on safe/finalized support. A source that cannot provide enough checkpoint evidence is bootstrap/shadow only or fails closed for the affected path.

## Live Intake Flow

```text
head notification or poll
  -> fetch block/header by hash if possible
  -> reconcile parent_hash against recent window
  -> fetch exact block-scoped selected logs/receipts/context
  -> persist block admission unit
  -> update chain_lineage
  -> route raw facts to adapters
  -> write normalized events
  -> enqueue projections
  -> advance chain checkpoints after reconciliation
```

Subscription payloads are never the system of record.

If a provider sends a head notification and the connection drops, correctness comes from re-fetching exact block data and reconciling parent hashes.

## Block Identity

```text
block_hash = identity
block_number = position
```

One block number can have multiple block hashes. Therefore bigname stores one lineage row per observed block hash.

Every block-scoped fact carries:

```text
chain_id
block_number
block_hash
canonicality_state
```

## Recent Window

Intake keeps a recent reconciled window:

```text
(chain_id, block_hash)
parent_hash
block_number
timestamp
optional header audit fields
canonicality_state
```

The recent window:

- detects parent mismatch;
- walks back to common ancestor;
- backfills short parent gaps;
- resolves canonicality disputes;
- supports safe/finalized promotion.

## Raw Fact Admission

For each admitted block, bigname may store:

- block lineage;
- selected logs;
- selected transactions;
- selected receipts;
- same-transaction sibling logs;
- code hash observations;
- call snapshots;
- payload cache metadata.

Selected target logs are the main raw input. Same-transaction sibling logs are retained only because replay needs the same context as live intake.

## Backfill Flow

Backfill is bounded, persisted, resumable work.

```text
create backfill job
  -> resolve selected target set
  -> partition finite range into backfill_ranges
  -> reserve range with lease
  -> fetch selected facts
  -> admit raw facts through same intake path
  -> advance range checkpoint
  -> complete range
  -> complete job when all ranges complete
```

Backfill and live ingestion share downstream normalization/projection after raw fetch.

## Backfill Job Shape

Job identity includes:

- selected deployment profile;
- chain;
- source selector;
- scan mode;
- finite range start;
- finite range end;
- resolved selected target set;
- idempotency key.

If an idempotency key is reused with the same immutable shape, bigname resumes/reuses the job.

If the same key is used with a different shape, it fails with conflict.

It does not silently widen, reset, or reinterpret old work.

## Selector Modes

Bigname source-scoped backfill supports:

### `whole_active_watched_chain`

Default. Includes every active watched target for the selected profile/chain whose active range intersects the job range.

### `source_family`

Targets one source family.

Unknown family or no active targets fails before job creation.

### `watched_target_set`

Explicit set of `contract_instance_id` targets.

Raw addresses alone are not accepted.

## Backfill Source Identity

The persisted source identity is not CLI spelling.

It is the resolved target set sorted by:

```text
source_family
contract_instance_id
normalized address
effective range start
effective range end
```

Large sets can be stored as a digest:

```text
selected_targets_digest_v1
```

This makes the job idempotent and auditable even if the manifest/watch plan changes later.

## Backfill Does Not Promote Chain Heads

Backfill checkpoints are operational fetch state.

They do not mutate:

```text
canonical_head
safe_head
finalized_head
```

Raw facts admitted by backfill still get canonicality states when evidence exists:

```text
finalized
safe
canonical
observed
```

But job completion is not finality.

## Canonicality At Historical Admission

If the range is proven below the finalized checkpoint, facts can be stored as `finalized`.

If below safe checkpoint, facts can be stored as `safe`.

If only current canonical chain is proven, facts can be `canonical`.

If evidence is weak, facts remain explicit weaker state or the job fails closed.

## Reorg Detection

Reorg detection is parent-hash reconciliation.

```text
incoming parent_hash != stored canonical head hash
  -> walk backward through stored lineage
  -> find common ancestor
  -> mark losing branch orphaned
  -> admit winning branch
  -> enqueue affected projection invalidations
```

## Reorg Repair

Bigname does not delete losing-branch facts.

It marks affected rows:

```text
canonicality_state = orphaned
```

Affected families:

- chain lineage;
- raw facts if retained;
- normalized events;
- identity rows;
- token lineages;
- resources;
- surface bindings;
- execution cache outcomes;
- projection invalidations.

Projection rows are then rebuilt from canonical/safe/finalized rows.

## Minimal Raw Retention And Reorgs

If minimal mode has compacted raw staging rows, reorg repair can still work because:

- `chain_lineage` remains;
- `normalized_events` carry block identity;
- identity rows carry block identity/canonicality;
- projections can rebuild from retained normalized/identity state.

If adapter-level byte replay is required for a compacted range and there is no retained digest-checked payload or provider/cache fill, repair fails closed.

## Execution Cache During Reorg

Execution traces and steps stay durable.

Reusable execution outcomes become ineligible if their dependency set includes an orphaned block hash.

Reorg invalidation affects cache reuse, not audit traces.

## Old ENS Registry Migration Guard

ENSRegistryOld is admitted as historical input.

Old registry:

```text
0x314159265dd8dbb310642f98f50c066173c1259b
start_block = 3327417
```

Current registry:

```text
0x00000000000C2E074eC69A0dFb2997BA6C7d2E1E
start_block = 9380380
```

Rule:

- current registry `NewOwner` marks a subnode migrated;
- later old-registry `NewOwner`, `Transfer`, `NewTTL`, and non-root `NewResolver` for that node are retained but suppressed;
- old-registry `NewResolver(ROOT_NODE,resolver)` is the exception and can still update root resolver discovery.

This mirrors ENS subgraph migration guard behavior but makes it explicit.

