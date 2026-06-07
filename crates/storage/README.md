# storage

Postgres persistence and query crate.

## Responsibility

`storage` owns database connectivity, migrations, row models, insert DTOs, repository methods, query filters, and maintenance operations.

## Modules

- `store`: SQLx pool wrapper and repository factories.
- `models`: SQL row DTOs.
- `inserts`: typed projection insert/update DTOs.
- `filters`: storage-level filter and order types.
- `query`: shared SQL query-builder helpers and order mappings.
- `repositories`: entity-specific repositories for accounts, domains, registrations, wrapped domains, resolvers, events, blocks, and checkpoints.
- `maintenance`: destructive operational maintenance helpers such as indexed-state reset.
- `error`: storage error type.

## Architecture Notes

Projection code should call repository methods instead of hand-writing SQL. Query construction uses typed enum-to-SQL mappings and parameterized `sqlx::QueryBuilder` fragments. Event history is stored in concrete event tables, while mutable entity tables hold current ENS state. Migrations are embedded with SQLx and applied by the CLI/server startup path.

## Boundary Rules

- This crate owns every table shape, migration, query DTO, and persistence-side filter type.
- This crate should not depend on GraphQL crates, Axum, Ethereum RPC clients, or contract ABI bindings.
- Repositories expose intent-level methods such as upserting domains, inserting events, reading relation pages, and updating checkpoints.
- SQL should remain parameterized through SQLx. Dynamic query builders must map enums to static column names instead of accepting raw client strings.

## Data Model Strategy

Mutable ENS state lives in current-state entity tables. Immutable history lives in event tables with IDs shaped like the official subgraph. This mirrors Graph Node projections: handlers update current rows as events arrive, while also preserving event records for chronological queries and relationship fields.

The current implementation intentionally starts with current-state reads. `_change_block` predicates use projection-maintained `entity_changes` plus event `block_number` columns. Historical block reads still require snapshot tables or versioned rows, and should be added as an explicit storage milestone rather than hidden inside API code.

## Testing Approach

Use migration smoke tests against Postgres, repository tests with small seeded fixtures, and SQL-shape tests for every filter/order branch that can be reached from GraphQL. Backfill tests should verify both event table inserts and current entity projections for a bounded block range.
