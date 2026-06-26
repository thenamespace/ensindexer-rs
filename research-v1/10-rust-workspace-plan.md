# 10 Rust Workspace Plan

The workspace should keep protocol logic, storage, projections, and API separated.

## Crates

Recommended crates:

```text
crates/types
crates/config
crates/chain
crates/source
crates/archive
crates/adapters
crates/projection
crates/storage
crates/execution
crates/api
crates/server
crates/cli
```

## Responsibilities

### `types`

Shared domain types:

- `NameId`;
- `ResourceId`;
- `SourceFamilyId`;
- normalized events;
- canonicality state;
- block references.

Keep this crate small and dependency-light.

### `config`

Loads environment/config files and validates strict startup behavior.

No hidden defaults for production-dangerous behavior.

### `chain`

RPC/HyperSync/live polling:

- block headers;
- logs;
- block-pinned calls;
- chain checkpoints;
- reorg detection helpers.

### `source`

Source-family definitions:

- typed Alloy events;
- contract addresses;
- manifest snapshots;
- authority rules;
- discovery definitions.

### `archive`

Binary raw archive support:

- writer;
- reader;
- manifest/checksum;
- conversion tooling if needed;
- range iteration.

### `adapters`

Source-specific normalization:

- decode raw facts;
- update identity state;
- emit normalized events;
- emit projection keys.

Adapter files should stay focused. Split by source family and protocol role.

### `projection`

Projection rebuild logic:

- exact name;
- hierarchy;
- address names;
- resolver records;
- permissions;
- primary names;
- search.

### `storage`

SQLx repositories and migrations.

Rules:

- no protocol logic;
- no API response shaping;
- explicit batch APIs for backfill;
- transaction boundaries are visible.

### `execution`

Verified reads:

- Universal Resolver;
- CCIP-read support where needed;
- primary-name verification;
- execution traces/outcomes;
- cache dependencies.

### `api`

GraphQL/HTTP schema and handlers.

Handlers read projections/execution/cache. They should not scan raw logs or mutate source-of-truth state.

### `server`

Unified runtime:

- HTTP server;
- optional backfill;
- optional live indexing;
- projection workers;
- health/readiness.

### `cli`

Operational commands:

- serve;
- migrate;
- archive tools;
- repair;
- benchmark;
- projection rebuild.

Keep CLI flags minimal. Environment/config should be source of truth for normal server behavior.

## Implementation Order

1. Domain types and config.
2. Migrations for chain/source/raw/normalized/identity/projection minimum.
3. Raw archive reader/writer.
4. ENS v1 registry/registrar/resolver adapters.
5. Projection rebuilds.
6. GraphQL compatibility API.
7. Reorg/canonicality handling.
8. ENS Rainbow/local label healing.
9. ENS v2/Basenames source adapters.
10. Execution/verified resolution.
11. Cache.
12. Repair and benchmark tooling.

This order keeps every step testable.
