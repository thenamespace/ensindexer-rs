# cli

Operational command-line entrypoint for the ENS indexer.

## Responsibility

`cli` wires configuration, storage, migrations, ingest jobs, and the HTTP server into commands suitable for local development and production jobs.

## Commands

- `serve`: run migrations and start the Axum GraphQL API.
- `migrate`: apply SQLx migrations.
- `backfill --from <block> --to <block>`: index a bounded historical range.
- `replay --from <block> --to <block> [--archive-dir <dir>]`: rebuild a bounded range from raw JSON archives without chain IO.
- `archive-status [--from <block>] [--to <block>] [--archive-dir <dir>]`: verify raw archive checksums and report range coverage gaps.
- `index`: run the confirmation-depth live indexing loop.
- `status`: print latest stored block and source checkpoints.
- `reset --yes`: clear indexed projection/event/checkpoint state for rebuilds.
- `compare --query-file <file>`: run one GraphQL query against the local API and a reference subgraph, then diff the JSON response.
- `schema-local [--output <file>]`: print or write the local `async-graphql` SDL.
- `schema-diff [--output <file>]`: fetch the official subgraph introspection schema and fail if local query fields, query args, input fields, enum values, or their owning types are missing.

## Architecture Notes

The binary keeps `main.rs` small and delegates command execution to `app.rs`. Commands share the same `.env` configuration path as the server and ingest crates, so operational behavior does not diverge between local and production runs.

`compare` is intentionally network-only and does not open the database. It reads `SUBGRAPH_URL` and optional `SUBGRAPH_AUTH_TOKEN` from `.env` or CLI flags, posts the same query and optional variables JSON to both endpoints, and fails with both pretty-printed responses when they differ. Use `--operation-name` for named query documents; the request body and user agent intentionally match the official Graph gateway's documented curl shape.

`schema-local` and `schema-diff` are schema-contract checks. `schema-diff` uses the same `.env` gateway configuration as `compare`, writes the official introspection JSON when `--output` is provided, and compares Graph Node compatibility names before any data is indexed. It intentionally exits non-zero while required official query fields, arguments, filter fields, or order enum values are missing.

Example against the official ENS subgraph gateway:

```bash
cargo run -p cli -- compare \
  --query-file fixtures/domains.graphql \
  --operation-name Subgraphs

cargo run -p cli -- schema-diff \
  --output target/official-subgraph-schema.json
```

## Boundary Rules

- This crate owns process-level orchestration and command UX.
- This crate should not duplicate repository SQL, GraphQL schema definitions, or projection event semantics.
- Long-running commands should emit useful tracing spans and return structured errors.
- Destructive commands must require an explicit confirmation flag.

## Testing Approach

Prefer integration-style command tests for argument parsing and command wiring. For commands that require Postgres or Ethereum RPC, test the inner service crates directly and keep CLI tests focused on selecting the correct operation.
