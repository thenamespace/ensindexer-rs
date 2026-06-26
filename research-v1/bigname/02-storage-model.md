# 02 Storage Model

Bigname storage is split by write ownership and replay boundary.

The important invariant:

```text
raw facts and normalized events are truth;
projection tables are disposable current read models.
```

## Storage Layers

```text
chain_lineage
raw_facts
manifests_and_discovery
identity_and_events
projections
execution
operator_audit
```

## Table Families

| Family | Tables | Write Owner | Function |
| --- | --- | --- | --- |
| Chain | `chain_lineage`, `chain_checkpoints`, `chain_header_audit` | intake | block ancestry, finality, reorg repair |
| Raw facts | `raw_logs`, `raw_transactions`, `raw_receipts`, `raw_call_snapshots`, `raw_code_hashes`, `raw_payload_cache_metadata` | intake | durable or staging replay input |
| Backfill | `backfill_jobs`, `backfill_ranges` | backfill worker | bounded resumable backfill lifecycle |
| Manifests | `manifest_versions`, `manifest_capability_flags`, `manifest_contract_instances`, `manifest_discovery_rules`, `manifest_alert_observations` | manifest/discovery workers | source admission and drift audit |
| Contract graph | `contract_instances`, `contract_instance_addresses`, `discovery_edges` | manifest/discovery/adapters | stable contract identity and topology |
| Identity | `name_surfaces`, `surface_bindings`, `resources`, `token_lineages`, `label_preimages`, `name_surface_normalization_repair_findings` | adapters/storage repair | name/resource/token identities |
| Events | `normalized_events`, `projection_normalized_event_changes` | adapters/storage trigger | semantic event stream and projection feed |
| Replay state | `normalized_replay_cursors`, `normalized_replay_adapter_checkpoints`, `normalized_replay_adapter_checkpoint_items` | replay orchestration | replay progress and adapter-private checkpoints |
| Projections | `name_current`, `children_current`, `address_names_current`, `permissions_current`, `resolver_current`, `record_inventory_current`, `primary_names_current` | projection workers | current API read models |
| Projection work | `projection_apply_cursors`, `projection_invalidations`, `projection_invalidation_dead_letters`, `current_projection_replay_status` | projection workers/storage triggers | projection watermarks, queues, dead letters, bootstrap markers |
| Execution | `execution_traces`, `execution_steps`, `execution_cache_outcomes`, `event_silent_resolver_call_observations` | execution/intake/reorg | verified execution audit and reusable outcomes |
| Identity facade sidecars | `address_names_current_identity_counts`, `address_names_current_identity_feed` | storage triggers | fast partner-facing identity count/feed reads |

## Baseline Tables

The baseline migration creates the core system:

```text
address_names_current
backfill_jobs
backfill_ranges
chain_checkpoints
chain_header_audit
chain_lineage
children_current
contract_instance_addresses
contract_instances
discovery_edges
execution_cache_outcomes
execution_steps
execution_traces
manifest_alert_observations
manifest_capability_flags
manifest_contract_instances
manifest_discovery_rules
manifest_versions
name_current
name_surfaces
normalized_events
normalized_replay_cursors
permissions_current
primary_names_current
raw_call_snapshots
raw_code_hashes
raw_logs
raw_payload_cache_metadata
raw_receipts
raw_transactions
record_inventory_current
resolver_current
resources
surface_bindings
token_lineages
```

Later migrations add:

```text
current_projection_replay_status
projection_normalized_event_changes
projection_apply_cursors
projection_invalidations
normalized_replay_adapter_checkpoints
normalized_replay_adapter_checkpoint_items
address_names_current_identity_counts
address_names_current_identity_feed
name_surface_normalization_repair_findings
event_silent_resolver_call_observations
label_preimages
label_preimage_backfill_runs
projection_invalidation_dead_letters
```

## Enum Types

Bigname has enums for:

```text
backfill_lifecycle_status
canonicality_state
capability_support_status
manifest_rollout_status
projection_invalidation_state
```

`canonicality_state` is central:

```text
observed
canonical
safe
finalized
orphaned
```

Bigname never infers truth from "latest row wins." Rows carry canonicality explicitly.

## Chain Tables

### `chain_lineage`

Stores block identity and ancestry:

- chain id;
- block hash;
- block number;
- parent hash;
- timestamp;
- canonicality state;
- observed time.

Why:

- block hash is identity;
- block number is position;
- reorg repair walks parent hashes;
- safe/finalized promotion is explicit;
- every raw and normalized chain fact can be tied to a block identity.

### `chain_checkpoints`

Stores canonical/safe/finalized heads per chain.

Why:

- API consistency modes map to these checkpoints;
- live intake resumes from them;
- backfill does not mutate them;
- finalized catch-up can plan bounded jobs against them.

### `chain_header_audit`

Optional header audit fields:

- logs bloom;
- transactions root;
- receipts root;
- state root.

Why:

- not needed for normal hot reads;
- useful for audit/provider drift;
- absent fields do not prevent reorg repair.

## Raw Fact Tables

### `raw_logs`

Stores selected/admitted EVM logs:

- chain/block/tx/log position;
- block hash;
- emitter address;
- topics;
- data;
- source/manifest/contract context;
- canonicality state.

Raw logs are replay input. In minimal mode, they can be compacted after normalized replay is durable. In log-audit mode, they remain durable audit facts.

### `raw_transactions`

Stores selected transaction context:

- transaction hash and index;
- block identity;
- from/to;
- selected input metadata;
- canonicality.

Why:

- same-transaction sibling context can be required by adapters;
- event-silent resolver hydration needs successful direct-call tx evidence;
- diagnostics can explain which transaction produced facts.

### `raw_receipts`

Stores selected receipt context:

- tx identity;
- status;
- gas metadata;
- created contract address;
- canonicality.

Why:

- confirms transaction success;
- supports deployment/proxy discovery;
- supports event-silent resolver observations.

### `raw_call_snapshots`

Stores block-pinned call observations.

Why:

- verified resolution and enrichment must be reproducible;
- call result must be tied to a block hash;
- execution can hand off selected snapshots, but intake owns the raw fact.

### `raw_code_hashes`

Stores code hash observations for watched contracts.

Why:

- manifest drift detection;
- proxy implementation checks;
- resolver profile admission;
- audit of dynamic discovery.

### `raw_payload_cache_metadata`

Stores metadata/digests for large payloads, not necessarily full bytes.

Why:

- Postgres remains hot indexed store;
- full block/receipt/trace payloads are cache unless explicitly retained;
- digest-checked provider/cache re-fetch can fail closed.

## Raw Retention Modes

Bigname supports two operational modes:

```text
minimal
log-audit
```

Minimal mode:

- raw logs/tx/receipts are staging;
- they may be compacted after normalized replay cursors are caught up;
- lineage, normalized events, identity rows, projection inputs remain durable.

Log-audit mode:

- raw facts remain durable audit material;
- heavier indexes can be retained;
- adapter byte replay is available without provider re-fetch.

Compaction is manual and guarded. It refuses to compact unless normalized replay is caught up and failure-free.

## Manifest And Discovery Tables

### `manifest_versions`

Stores source manifest snapshots:

- namespace;
- source family;
- chain;
- deployment epoch;
- rollout status;
- normalizer version;
- content hash;
- manifest JSON.

### `manifest_capability_flags`

Stores capability status:

```text
unsupported
shadow
supported
```

Capabilities attach to the declaring source family. Presence of another family does not imply support.

### `manifest_contract_instances`

Stores manifest-declared contract addresses and roles.

### `manifest_discovery_rules`

Stores source-specific discovery policy.

### `manifest_alert_observations`

Stores operational manifest/proxy/code drift findings. These are audit observations, not manifest truth.

## Contract Graph Tables

### `contract_instances`

Stable identity for contracts.

### `contract_instance_addresses`

Time-ranged address attributes for contract instances.

### `discovery_edges`

Time-ranged topology:

```text
resolver
subregistry
parent
alias
metadata
proxy_implementation
migration
transport
```

Why:

- watch plans expand through graph reachability;
- dynamic resolvers/subregistries are explainable;
- proxy implementation changes do not rewrite contract identity.

## Identity Tables

### `name_surfaces`

Canonical public surfaces.

Keyed by `logical_name_id`.

Stores:

- namespace;
- normalized name;
- display name;
- DNS-encoded name;
- namehash;
- labelhash path;
- normalizer version;
- canonicality/provenance.

### `surface_bindings`

Time-ranged surface-to-resource relation.

This captures authority movement:

- registry-only control;
- registrar lease;
- wrapper authority;
- migration;
- alias;
- wildcard observation.

### `resources`

Backing authority objects.

Examples:

- ENS v1 registry-only resource;
- ENS v1 registrar resource;
- ENS v1 wrapper resource;
- ENS v2 EAC resource;
- Basenames authority resource.

### `token_lineages`

Tokenized ownership history.

Important for ENSv2 token regeneration and ENS v1 registrar/wrapper transitions.

### `label_preimages`

Verified labelhash-to-label facts.

They improve child readability. They do not create ownership, resolver, exact-name authority, record truth, or primary-name truth.

## Normalized Event Tables

### `normalized_events`

Adapter-owned semantic event stream.

Stores:

- event identity;
- namespace;
- logical name id;
- resource id;
- token lineage id;
- source family;
- manifest id/version;
- chain id;
- block number/hash;
- tx/log identity;
- event kind;
- before state;
- after state;
- payload/provenance;
- canonicality state.

The upsert path is idempotent. Same event identity with conflicting payload is a hard mismatch unless a documented repair allows it.

### `projection_normalized_event_changes`

Append-only feed populated by storage trigger for:

- normalized event insert;
- normalized event update;
- canonicality update.

Projection workers consume this feed to enqueue key-scoped invalidations.

## Projection Tables

Projection tables are route read models:

```text
name_current
address_names_current
children_current
permissions_current
resolver_current
record_inventory_current
primary_names_current
```

They are rebuilt from canonical normalized events and identity rows.

## Execution Tables

### `execution_traces`

Durable trace identity and metadata.

### `execution_steps`

Ordered steps inside an execution trace.

### `execution_cache_outcomes`

Reusable verified result. Reusable only while block-hash dependencies, manifest versions, topology version, and record boundaries still match.

Reorg invalidates cache outcomes but does not delete traces/steps.

