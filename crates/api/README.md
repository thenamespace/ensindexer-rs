# api

GraphQL schema crate for the ENS indexer.

## Responsibility

`api` owns the async-graphql public API shape. It converts storage rows into GraphQL DTOs, exposes root query methods, maps Graph Node-style filter/order inputs into typed storage filters, and defines current compatibility behavior for `_meta`, `block`, and `subgraphError`.

## Modules

- `schema`: root query object, schema construction, root entity/event query wiring.
- `objects`: GraphQL entity DTOs and relationship resolvers.
- `objects/events`: concrete event DTOs and event interfaces.
- `filters`: GraphQL input objects and order enums mapped to storage filters.
- `meta`: `_meta`, `Block_height`, `_Block_`, and `_SubgraphErrorPolicy_` compatibility types.
- `pagination`: shared `first`/`skip` normalization.

## Architecture Notes

The API returns Graph-compatible scalar values as strings where The Graph would expose `Bytes` or `BigInt`. Storage remains the source of truth; GraphQL resolvers do not project or mutate chain data. Historical `block` arguments are accepted for schema compatibility, but non-current block reads currently return an explicit compatibility error until snapshot support is implemented.

## Boundary Rules

- This crate may depend on `storage` row/query APIs, but it should not know SQL strings.
- This crate may define GraphQL DTOs that mirror official subgraph entity names and field names.
- This crate should not decode logs, call Ethereum RPC, run migrations, or mutate indexed state.
- Relationship resolvers should remain thin: normalize pagination/filter inputs, call storage, and map rows to DTOs.

## Compatibility Targets

The public schema should stay a drop-in replacement for the official ENS subgraph where practical:

- Root entity queries expose singular and plural fields such as `domain`, `domains`, `registration`, `registrations`, `resolver`, `resolvers`, `wrappedDomain`, `wrappedDomains`, `account`, and event collections.
- Root query arguments should keep The Graph conventions: `id`, `first`, `skip`, `orderBy`, `orderDirection`, `where`, `block`, and `subgraphError`.
- Filter input names and enum names should match Graph Node naming, including nested relation filters where supported.
- `_meta` should expose deployment, block number/hash, and a `hasIndexingErrors` boolean.

## Testing Approach

Use resolver-level tests for argument normalization and DTO mapping, and integration tests for GraphQL query shapes against a seeded Postgres database. The high-value regression tests are official-subgraph compatibility snapshots: execute the same query against this server and the hosted ENS subgraph, normalize ordering where needed, and compare JSON response shape before comparing exact values.
