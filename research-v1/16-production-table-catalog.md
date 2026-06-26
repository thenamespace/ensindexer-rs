# 16 Production Table Catalog

This is the recommended production table set.

It is smaller than a full forensic system, but enough for an auditable public indexer.

## Table Families

```text
chain
source
raw
events
identity
projection
execution/cache
operations
```

Each table below exists because it answers a real operational question.

## Chain Tables

### `chain_blocks`

Stores one observed block hash.

Why needed:

- reorg detection;
- canonical/safe/finalized promotion;
- block-hash provenance for raw facts and normalized events.

Core fields:

- `chain_id`;
- `block_number`;
- `block_hash`;
- `parent_hash`;
- `timestamp`;
- `canonicality_state`;
- `first_seen_at`;
- `updated_at`.

### `chain_checkpoints`

Stores per-chain worker progress.

Why needed:

- resume live indexing;
- know latest ingested/canonical/safe/finalized position;
- expose lag.

Core fields:

- `chain_id`;
- `checkpoint_kind`;
- `block_number`;
- `block_hash`;
- `updated_at`.

## Source Tables

### `source_families`

Stores source-family identity.

Examples:

- `ens_v1_registry_mainnet`;
- `ens_v1_registrar_mainnet`;
- `ens_v2_registry_sepolia`;
- `basenames_base_registrar`;
- `linea_resolver_future`.

Why needed:

- every fact must say which source interpreted it;
- source-specific metrics and replay need stable ids.

### `source_manifest_snapshots`

Immutable snapshot of source definition active during ingest.

Why needed:

- reproducible replay;
- explain which event definitions/contracts produced rows;
- safe upgrades.

This table stores audit snapshots, not editable runtime config.

### `contract_instances`

Stable contract identity.

Why needed:

- dynamic resolver discovery;
- proxy/code-hash drift;
- source role tracking;
- avoiding raw address strings everywhere.

### `contract_addresses`

Time-ranged address assignment for contract instances.

Why needed:

- proxies/migrations;
- address reuse;
- historical watch plan reconstruction.

### `watch_targets`

The concrete logs/calls currently watched.

Why needed:

- inspect backfill scope;
- diff source upgrades;
- explain why a raw log was selected or ignored.

## Raw Tables

### `raw_logs`

Selected chain logs used for indexing.

Why needed:

- replay adapters without provider calls;
- audit normalized events;
- debug missed/incorrect projections.

For deployments that keep raw logs in binary archives, this table can store only metadata or recent/live rows.

### `raw_call_snapshots`

Block-pinned contract call outputs.

Why needed:

- ENS v2 enrichment;
- verified resolution;
- event-silent resolver behavior;
- execution audit.

### `raw_archive_ranges`

Metadata for binary archive files.

Why needed:

- resume archive-only fetch;
- replay local archives;
- validate checksums and format versions.

### Optional Raw Context

Use by retention profile:

- `raw_transactions`;
- `raw_receipts`;
- `raw_code_observations`.

Keep them if they materially improve audit/debug for your deployment.

## Event Tables

### `normalized_events`

Semantic event stream.

Why needed:

- all projections rebuild from it;
- source differences become one shared vocabulary;
- reorg/canonicality changes are visible.

### `normalized_event_changes`

Append-only change feed for projection workers.

Why needed:

- projections should not poll the whole event table;
- canonicality updates must trigger rebuilds;
- repair jobs need exact change watermarks.

### `adapter_cursors`

Progress of raw-to-normalized replay by source family and range.

Why needed:

- raw archive can be complete while adapters are behind;
- partial replay is resumable;
- stateful adapters need explicit progress.

## Identity Tables

### `names`

Stable public name identity in the tree.

Why needed:

- exact lookup;
- hash-only names;
- label healing;
- search;
- parent/child projection.

Do not enforce parent row existence here.

### `label_preimages`

Validated label text for labelhashes.

Why needed:

- readable children;
- search;
- healing hash-only names.

### `resources`

Authority/control objects behind names.

Why needed:

- wrapping;
- registrar leases;
- ENS v2 resources;
- L2-managed authority;
- permissions.

### `name_resource_bindings`

Time-ranged binding from name to resource.

Why needed:

- explain authority transitions;
- rebuild current owner/controller;
- preserve history across wrap/unwrap/lapse.

### `token_lineages`

Token history attached to resources.

Why needed:

- registrar token transfers;
- wrapper token transfers;
- ENS v2 token regeneration.

### `role_assignments`

Normalized permission/control facts.

Why needed:

- permissions projection;
- address-name relation projection;
- role history and audit.

## Projection Tables

### `name_current`

Fast exact-name current state.

### `name_hierarchy_current`

Fast parent/children/subtree state.

### `address_names_current`

Fast address-to-name relations.

### `resolver_records_current`

Fast resolver record state.

### `resolver_current`

Resolver instance support/current summary.

### `permissions_current`

Effective permissions and roles.

### `primary_names_current`

Declared and verified primary names.

### `search_names_current`

Text/ranking/search projection.

Why projections are needed:

- API latency;
- stable pagination;
- no raw-event scans;
- easy cache invalidation.

## Projection Work Tables

### `projection_cursors`

Per-projection consumed change watermark.

### `projection_invalidations`

Key-scoped rebuild queue.

### `projection_dead_letters`

Failed projection keys requiring operator attention.

### `projection_rebuild_runs`

Audit of full/partial projection rebuilds.

## Execution And Cache Tables

### `execution_traces`

One verified resolution/primary-name request.

### `execution_steps`

Ordered call/CCIP/proof steps for a trace.

### `execution_outcomes`

Reusable verified answer with dependency keys.

### `cache_entries`

Optional API/query cache.

### `cache_invalidations`

Audit of cache invalidation by event/reorg/projection change.

## Operations Tables

### `backfill_jobs`

Historical backfill/replay jobs.

### `repair_jobs`

Explicit repair/healing jobs.

### `indexer_findings`

Operator-visible issues:

- source conflict;
- unsupported resolver;
- projection failure;
- invalid preimage;
- provider mismatch;
- invariant violation.

### `schema_migration_audits`

Operational context for large migrations.

SQLx migration metadata says what ran. This table explains why and with which safety mode.

## Tables Deliberately Excluded

Do not create:

- separate name tables per chain;
- separate name tables per branded suffix;
- one table per resolver record event;
- source-specific current tables unless a real query proves it is necessary.

Use source fields and normalized payloads instead.
