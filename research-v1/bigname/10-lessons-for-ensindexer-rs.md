# 10 Lessons For ensindexer-rs

This file translates bigname behavior into design lessons for our indexer.

## Copy These Ideas

### 1. Raw Facts, Normalized Events, Projections

Bigname's strongest design is the three-layer pipeline:

```text
raw facts
  -> normalized events
  -> projections
```

This is exactly what our Rust indexer should use.

Raw archives and raw tables are replay input. Normalized events are semantic truth. Projections are read models.

### 2. Block Hash First

Do not key correctness by block number.

Use:

```text
chain_id
block_hash
parent_hash
block_number
canonicality_state
```

This is required for reorg safety.

### 3. Explicit Canonicality

Rows should carry:

```text
observed
canonical
safe
finalized
orphaned
```

Never let "latest row wins" decide truth.

### 4. Identity Split

Copy bigname's internal identity split:

```text
public name identity
resource identity
token lineage identity
contract instance identity
name/resource binding
```

For us:

```text
bigname public id = product bucket + normalized name
ours name_id = namehash(normalized global name)
```

Everything else remains useful.

### 5. Contract Graph

Use stable contract instances and time-ranged addresses.

Dynamic resolvers, subregistries, proxies, and transport contracts should be graph nodes, not raw address strings sprinkled everywhere.

### 6. Manifest-Driven Admission

Every watched contract/event/capability should come from a manifest or discovery rule.

Do not let arbitrary logs mutate product state.

### 7. Adapter Replay Classes

Classify each adapter:

```text
stateless_raw_fact
contextual_dependency_required
stateful_closure_required
```

This prevents unsafe partial replay.

### 8. Projection Invalidations

Use key-scoped invalidation:

```text
projection
projection_key
generation
state
```

Coalesce repeated changes and rebuild only affected keys.

### 9. Dead Letters

Projection failures should become operator-visible dead letters after retry limit.

Do not spin forever or silently drop invalidations.

### 10. Execution Traces

Verified resolution should persist traces/steps.

Cache outcomes are disposable/reusable. Traces are audit.

## Do Not Copy These Ideas Directly

### Product-Split Public Identity

Bigname:

```text
ens:base.eth
basenames:alice.base.eth
```

Our indexer should use:

```text
base.eth
alice.base.eth
```

Then attach:

```text
source_family = basenames_base
authority_scope = suffix:base.eth
```

### Parent As Required Identity Dependency

Bigname avoids this because namespaces split `base.eth` and `alice.base.eth`.

Our global tree should avoid strict parent FKs in `names`.

Use:

```text
names = identity
name_hierarchy_current = derived tree projection
children_current = route read model
```

This avoids fake parent authority rows during live cross-chain indexing.

### Route Shape

Bigname routes by its own product model.

Our API routes and GraphQL resolvers should be global-name-first:

```text
/v1/names/alice.base.eth
/v1/names/base.eth/children
/v1/search?q=alice
```

Source family should not be part of public identity.

## Storage Ideas To Adopt

Minimum production families:

```text
chain_blocks
chain_checkpoints
raw_logs or raw archive metadata
raw_call_snapshots
source_families
source_manifest_snapshots
contract_instances
contract_addresses
watch_targets
raw_archive_ranges
adapter_cursors
names
name_hierarchy_current
resources
name_resource_bindings
token_lineages
label_preimages
role_assignments
normalized_events
normalized_event_changes
name_current
address_names_current
resolver_records_current
resolver_current
primary_names_current
permissions_current
search_names_current
projection_cursors
projection_invalidations
projection_dead_letters
projection_rebuild_runs
execution_traces
execution_steps
execution_outcomes
cache_entries
cache_invalidations
backfill_jobs
repair_jobs
indexer_findings
```

## Backfill Lessons

Backfill should be:

- bounded;
- resumable;
- idempotent;
- source-shape stable;
- separate from chain checkpoints;
- same downstream pipeline as live intake.

Raw archive replay should follow the same adapter path as RPC/HyperSync backfill.

## Reorg Lessons

Reorg repair should:

1. walk lineage by parent hash;
2. mark losing branch facts orphaned;
3. admit winning branch facts;
4. enqueue affected projection keys;
5. invalidate route/execution cache by dependency keys;
6. never patch projections directly.

## Projection Lessons

Every hot API route should have a projection.

Do not do API-time joins across raw logs or broad normalized-event scans.

For our global tree, key projections by:

```text
name_id
parent_name_id
root_name_id
address
resource_id
resolver tuple
record selector
```

not by source/product bucket.

## Event Lessons

Adopt bigname-style normalized event vocabulary, but rename where helpful:

```text
NameRegistered
NameRenewed
NameReleased
AuthorityChanged
ResolverChanged
RecordChanged
RecordVersionChanged
PermissionChanged
TokenResourceLinked
TokenRegenerated
SubregistryChanged
ParentChanged
AliasChanged
PrimaryNameClaimed
VerifiedResolutionObserved
CoverageChanged
```

The vocabulary should be source-agnostic.

## Final Recommendation

Bigname is strong because it makes production indexing responsibilities explicit:

- manifest admission;
- raw replay;
- normalized events;
- resource identity;
- projection invalidation;
- reorg orphaning;
- execution audit;
- repair workflows.

Our indexer should copy that production discipline.

But our public identity model should stay global:

```text
root / 2LD / subname
```

not source/product-prefixed public identity.
