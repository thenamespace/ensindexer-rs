# cli

Operational command-line entrypoint for the ENS indexer.

## Responsibility

`cli` wires configuration, storage, migrations, ingest jobs, and the HTTP server into commands suitable for local development and production jobs.

## Commands

- `serve`: run migrations and start the Axum GraphQL API.
- `migrate`: apply SQLx migrations.
- `backfill --from <block> --to <block>`: index a bounded historical range.
- `replay --from <block> --to <block> [--archive-dir <dir>]`: rebuild a bounded range from raw JSON archives without chain IO.
- `index`: run the confirmation-depth live indexing loop.
- `status`: print latest stored block and source checkpoints.
- `reset --yes`: clear indexed projection/event/checkpoint state for rebuilds.
- `compare --query-file <file>`: run one GraphQL query against the local API and a reference subgraph, then diff the JSON response.

## Architecture Notes

The binary keeps `main.rs` small and delegates command execution to `app.rs`. Commands share the same `.env` configuration path as the server and ingest crates, so operational behavior does not diverge between local and production runs.

`compare` is intentionally network-only and does not open the database. It reads `SUBGRAPH_URL` and optional `SUBGRAPH_AUTH_TOKEN` from `.env` or CLI flags, posts the same query and optional variables JSON to both endpoints, and fails with both pretty-printed responses when they differ.

## Boundary Rules

- This crate owns process-level orchestration and command UX.
- This crate should not duplicate repository SQL, GraphQL schema definitions, or projection event semantics.
- Long-running commands should emit useful tracing spans and return structured errors.
- Destructive commands must require an explicit confirmation flag.

## Testing Approach

Prefer integration-style command tests for argument parsing and command wiring. For commands that require Postgres or Ethereum RPC, test the inner service crates directly and keep CLI tests focused on selecting the correct operation.
