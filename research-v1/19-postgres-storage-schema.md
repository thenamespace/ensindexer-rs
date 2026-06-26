# 19 Postgres Storage Schema

This is a practical schema outline, not final SQL.

The goal is to show the important columns, keys, and indexes without turning the research guide into migration code.

## Common Types

Use explicit enums or constrained text for:

```text
canonicality_state: observed | canonical | safe | finalized | orphaned
job_state: pending | running | complete | failed | cancelled
support_status: supported | unsupported | unknown | deprecated
projection_state: pending | running | complete | failed
```

Use binary types for hashes/addresses where possible:

```text
bytea or fixed domain types for hashes
numeric/text for large ids where sqlx/alloy compatibility is simpler
```

## Chain

### `chain_blocks`

Core columns:

- `chain_id`;
- `block_number`;
- `block_hash`;
- `parent_hash`;
- `timestamp`;
- `canonicality_state`;
- `first_seen_at`;
- `updated_at`.

Keys/indexes:

```sql
primary key (chain_id, block_hash)
unique (chain_id, block_number, block_hash)
index (chain_id, block_number)
index (chain_id, parent_hash)
index (chain_id, canonicality_state, block_number)
```

### `chain_checkpoints`

Core columns:

- `chain_id`;
- `checkpoint_kind`;
- `block_number`;
- `block_hash`;
- `updated_at`.

Key:

```sql
primary key (chain_id, checkpoint_kind)
```

## Source

### `source_families`

Core columns:

- `source_family_id`;
- `chain_id`;
- `name`;
- `adapter_kind`;
- `support_status`;
- `created_at`;
- `updated_at`.

### `source_manifest_snapshots`

Core columns:

- `manifest_snapshot_id`;
- `source_family_id`;
- `version`;
- `content_hash`;
- `active_from_block`;
- `active_to_block`;
- `snapshot_json`;
- `created_at`.

Important indexes:

```sql
unique (source_family_id, content_hash)
index (source_family_id, active_from_block, active_to_block)
```

### `contract_instances`

Core columns:

- `contract_instance_id`;
- `source_family_id`;
- `chain_id`;
- `role`;
- `support_status`;
- `first_seen_block`;
- `created_at`.

### `contract_addresses`

Core columns:

- `contract_instance_id`;
- `chain_id`;
- `address`;
- `from_block`;
- `to_block`;
- `code_hash`;
- `created_at`.

Important indexes:

```sql
index (chain_id, address, from_block, to_block)
unique (contract_instance_id, chain_id, address, from_block)
```

### `watch_targets`

Core columns:

- `watch_target_id`;
- `source_family_id`;
- `manifest_snapshot_id`;
- `contract_instance_id`;
- `chain_id`;
- `address`;
- `event_signature`;
- `from_block`;
- `to_block`;
- `support_status`.

## Raw / Archive

### `raw_logs`

Core columns:

- `raw_log_id`;
- `chain_id`;
- `block_number`;
- `block_hash`;
- `transaction_hash`;
- `transaction_index`;
- `log_index`;
- `address`;
- `topic0`;
- `topics`;
- `data`;
- `source_family_id`;
- `watch_target_id`;
- `canonicality_state`;
- `archive_range_id`;
- `created_at`.

Keys/indexes:

```sql
unique (chain_id, block_hash, transaction_hash, log_index)
index (source_family_id, block_number)
index (chain_id, canonicality_state, block_number)
index (archive_range_id)
```

Partition by chain and block range once volume is large.

### `raw_call_snapshots`

Core columns:

- `call_snapshot_id`;
- `chain_id`;
- `block_number`;
- `block_hash`;
- `to_address`;
- `call_data_hash`;
- `return_data`;
- `success`;
- `source_family_id`;
- `purpose`;
- `canonicality_state`;
- `created_at`.

### `raw_archive_ranges`

Core columns:

- `archive_range_id`;
- `chain_id`;
- `from_block`;
- `to_block`;
- `format_version`;
- `file_path`;
- `byte_size`;
- `log_count`;
- `checksum`;
- `created_at`;
- `verified_at`.

Indexes:

```sql
unique (chain_id, from_block, to_block, file_path)
index (chain_id, from_block)
```

## Events

### `normalized_events`

Core columns:

- `normalized_event_id`;
- `event_identity`;
- `event_kind`;
- `source_family_id`;
- `manifest_snapshot_id`;
- `chain_id`;
- `block_number`;
- `block_hash`;
- `transaction_hash`;
- `log_index`;
- `raw_log_id`;
- `call_snapshot_id`;
- `canonicality_state`;
- `name_id`;
- `resource_id`;
- `token_lineage_id`;
- `address`;
- `resolver_address`;
- `record_key`;
- `before_state`;
- `after_state`;
- `adapter_version`;
- `created_at`;
- `updated_at`.

Keys/indexes:

```sql
unique (event_identity)
index (event_kind, block_number)
index (name_id, event_kind, block_number)
index (resource_id, event_kind, block_number)
index (address, event_kind, block_number)
index (chain_id, canonicality_state, block_number)
```

### `normalized_event_changes`

Core columns:

- `change_id`;
- `normalized_event_id`;
- `change_kind`;
- `created_at`.

Index:

```sql
index (change_id)
```

### `adapter_cursors`

Core columns:

- `source_family_id`;
- `cursor_kind`;
- `chain_id`;
- `from_block`;
- `to_block`;
- `last_processed_block`;
- `state`;
- `updated_at`.

## Identity

### `names`

Core columns:

- `name_id`;
- `node`;
- `normalized_name`;
- `display_name`;
- `labelhash`;
- `label`;
- `root_label`;
- `depth`;
- `normalization_status`;
- `first_seen_event_id`;
- `created_at`;
- `updated_at`.

Indexes:

```sql
primary key (name_id)
unique (normalized_name) where normalized_name is not null
index (node)
index (labelhash)
index (root_label)
```

No parent foreign key here.

### `label_preimages`

Core columns:

- `labelhash`;
- `label`;
- `normalized_label`;
- `source`;
- `first_seen_event_id`;
- `confidence`;
- `created_at`.

Key:

```sql
primary key (labelhash, normalized_label)
```

### `resources`

Core columns:

- `resource_id`;
- `resource_kind`;
- `source_family_id`;
- `chain_id`;
- `upstream_key`;
- `created_event_id`;
- `created_at`;
- `updated_at`.

### `name_resource_bindings`

Core columns:

- `binding_id`;
- `name_id`;
- `resource_id`;
- `binding_kind`;
- `from_event_id`;
- `to_event_id`;
- `from_block`;
- `to_block`;
- `canonicality_state`;
- `created_at`;
- `updated_at`.

Indexes:

```sql
index (name_id, from_block, to_block)
index (resource_id, from_block, to_block)
```

### `token_lineages`

Core columns:

- `token_lineage_id`;
- `resource_id`;
- `token_standard`;
- `chain_id`;
- `contract_instance_id`;
- `current_token_id`;
- `created_event_id`;
- `updated_event_id`.

### `role_assignments`

Core columns:

- `role_assignment_id`;
- `resource_id`;
- `name_id`;
- `address`;
- `role`;
- `from_event_id`;
- `to_event_id`;
- `canonicality_state`.

Indexes:

```sql
index (address, role)
index (name_id, role)
index (resource_id, role)
```

## Current Projections

All projection rows should include:

- projection version;
- source/canonicality provenance;
- last event/change id;
- recomputed timestamp.

### `name_current`

Core columns:

- `name_id`;
- `normalized_name`;
- `display_name`;
- `node`;
- `current_resource_id`;
- `owner_address`;
- `registrant_address`;
- `wrapped_owner_address`;
- `resolver_address`;
- `expiry`;
- `ttl`;
- `is_wrapped`;
- `is_expired`;
- `support_status`;
- `last_change_id`;
- `recomputed_at`.

Indexes:

```sql
primary key (name_id)
index (normalized_name)
index (owner_address)
index (registrant_address)
index (resolver_address)
```

### `name_hierarchy_current`

Core columns:

- `parent_name_id`;
- `child_name_id`;
- `root_name_id`;
- `depth`;
- `label`;
- `labelhash`;
- `child_display_name`;
- `edge_status`;
- `last_change_id`.

Indexes:

```sql
primary key (parent_name_id, child_name_id)
index (child_name_id)
index (root_name_id, depth)
```

### `address_names_current`

Core columns:

- `address`;
- `name_id`;
- `relation`;
- `resource_id`;
- `is_verified`;
- `rank`;
- `last_change_id`.

Indexes:

```sql
primary key (address, name_id, relation)
index (name_id, relation)
index (address, relation, rank)
```

### `resolver_records_current`

Core columns:

- `name_id`;
- `resolver_address`;
- `record_kind`;
- `record_key`;
- `record_value`;
- `record_version`;
- `last_change_id`.

Indexes:

```sql
primary key (name_id, record_kind, record_key)
index (resolver_address)
```

### Other Projection Tables

Use the same pattern for:

- `resolver_current`;
- `permissions_current`;
- `primary_names_current`;
- `search_names_current`.

## Projection Work

### `projection_cursors`

```sql
primary key (projection_name)
```

Fields:

- `projection_name`;
- `last_change_id`;
- `updated_at`.

### `projection_invalidations`

Fields:

- `projection_name`;
- `projection_key`;
- `state`;
- `first_change_id`;
- `last_change_id`;
- `attempts`;
- `updated_at`.

Key:

```sql
primary key (projection_name, projection_key)
```

### `projection_dead_letters`

Fields:

- `projection_name`;
- `projection_key`;
- `error`;
- `failed_at`;
- `attempts`;
- `payload`.

## Execution / Cache

### `execution_traces`

Fields:

- `execution_trace_id`;
- `request_key`;
- `name_id`;
- `address`;
- `chain_id`;
- `block_number`;
- `block_hash`;
- `entrypoint`;
- `status`;
- `created_at`.

### `execution_steps`

Fields:

- `execution_trace_id`;
- `step_index`;
- `step_kind`;
- `to_address`;
- `call_data_hash`;
- `result_hash`;
- `status`;
- `details`.

### `execution_outcomes`

Fields:

- `outcome_id`;
- `request_key`;
- `execution_trace_id`;
- `result`;
- `dependency_keys`;
- `valid_from_block`;
- `invalidated_at`.

### `cache_entries`

Fields:

- `cache_key`;
- `value`;
- `dependency_keys`;
- `expires_at`;
- `created_at`;
- `invalidated_at`.

## Operations

Keep operational tables boring and small:

- `backfill_jobs`;
- `repair_jobs`;
- `indexer_findings`;
- `schema_migration_audits`.

Each should have:

- id;
- kind;
- state;
- input parameters;
- started/completed timestamps;
- error/finding payload;
- operator/build metadata.

## Insert Order

Recommended migration order:

```text
chain
source
raw/archive
events
identity
projections
projection work
execution/cache
operations
```

## Partitioning

Partition only the large append-only tables:

- `raw_logs`;
- `normalized_events`;
- optionally `normalized_event_changes`;
- high-volume execution/cache tables if needed.

Do not prematurely partition small identity/projection tables.

## Final Schema Rule

For every table, ask:

```text
Can this row be explained from chain evidence, source definition, or projection logic?
```

If no, it should not be in the production schema.
