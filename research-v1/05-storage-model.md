# 05 Storage Model

The database is split by responsibility.

Each table family answers a different operational question.

## Storage Layers

```text
chain evidence
  -> source definitions
  -> raw facts / raw archive metadata
  -> normalized events
  -> identity
  -> projections
  -> execution/cache
  -> operations/audit
```

## Essential Table Families

| Family | Purpose | Required in v1 |
| --- | --- | --- |
| Chain | block identity, checkpoints, reorg repair | yes |
| Source | source-family and manifest snapshot provenance | yes |
| Raw | replay input and archive metadata | yes |
| Normalized events | semantic truth stream | yes |
| Identity | names, resources, token lineages, bindings | yes |
| Projections | fast API read models | yes |
| Projection work | invalidation queue, cursors, dead letters | yes |
| Execution | verified resolution traces/outcomes | yes for verified routes |
| Cache | optional API/query acceleration | recommended |
| Operations | jobs, repairs, findings | yes, but small |

## Minimal Production Table Set

This is the table set that is worth carrying into v1.

### Chain

- `chain_blocks`
- `chain_checkpoints`

Optional but useful:

- `chain_header_audits`

### Source

- `source_families`
- `source_manifest_snapshots`
- `contract_instances`
- `contract_addresses`
- `watch_targets`

### Raw / Archive

- `raw_logs`
- `raw_call_snapshots`
- `raw_archive_ranges`

Optional by retention profile:

- `raw_transactions`
- `raw_receipts`
- `raw_code_observations`

### Semantic Events

- `normalized_events`
- `normalized_event_changes`
- `adapter_cursors`

### Identity

- `names`
- `label_preimages`
- `resources`
- `name_resource_bindings`
- `token_lineages`
- `role_assignments`

### Projections

- `name_current`
- `name_hierarchy_current`
- `address_names_current`
- `resolver_records_current`
- `resolver_current`
- `permissions_current`
- `primary_names_current`
- `search_names_current`

### Projection Work

- `projection_cursors`
- `projection_invalidations`
- `projection_dead_letters`
- `projection_rebuild_runs`

### Execution / Cache

- `execution_traces`
- `execution_steps`
- `execution_outcomes`
- `cache_entries`
- `cache_invalidations`

### Operations

- `backfill_jobs`
- `repair_jobs`
- `indexer_findings`
- `schema_migration_audits`

## Tables We Should Not Add Yet

Do not add separate tables for:

- Basenames names;
- Linea names;
- Celo names;
- ENS v2 names;
- each resolver event type;
- every possible execution support class;
- every GraphQL field.

Those concerns belong in source-family metadata, normalized event payloads, or projections.

## Retention Profiles

Use explicit retention modes.

### Fast Local Experiment

Keep:

- manifest snapshots;
- chain blocks/checkpoints;
- normalized events;
- identity;
- projections;
- raw archive range metadata.

Store raw logs primarily in binary archives on disk/object storage.

### Auditable Production

Also keep:

- selected `raw_logs` in Postgres;
- selected transaction/receipt context;
- code observations;
- execution traces;
- repair findings.

### Full Forensic Mode

Also keep:

- complete raw payloads;
- richer header audits;
- all receipt/log siblings in watched blocks.

This will grow storage quickly and should be a deliberate deployment choice.

## Why Not Store Everything In Current Tables?

Because current tables are the answer, not the proof.

If `name_current.owner` is wrong, we need to trace:

```text
name_current row
  -> projection invalidation
  -> normalized event
  -> raw log/call
  -> source manifest snapshot
  -> chain block hash
```

That chain is what makes the indexer auditable.
