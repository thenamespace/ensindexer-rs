# ENS Indexer

Custom Rust ENS indexer intended to be a drop-in replacement for the current ENS subgraph schema and GraphQL query shape.

The implementation plan and official-subgraph research live in [docs/README.md](docs/README.md).

## Workspace

- `crates/types`: shared IDs, constants, log context, scalar helpers.
- `crates/contracts`: Alloy event bindings and decoded ENS event enum.
- `crates/config`: `.env` based runtime configuration.
- `crates/storage`: SQLx pool, migrations, repository/query foundations.
- `crates/projection`: deterministic projection dispatcher and handler modules.
- `crates/ingest`: backfill/live indexing service skeleton.
- `crates/api`: async-graphql schema and resolvers.
- `crates/server`: Axum HTTP server.
- `crates/cli`: operational CLI entrypoint.

## Setup

```bash
cp .env.example .env
make db-up
make migrate
make serve
```

Configuration is loaded from `.env` via `config`.
Open [http://127.0.0.1:8080/graphql](http://127.0.0.1:8080/graphql) in a browser for Apollo Sandbox when `GRAPHQL_SANDBOX=true`.

Indexer commands:

```bash
make status
make backfill BACKFILL_FROM=9380380 BACKFILL_TO=9381380
cargo run -p cli -- index
make reset
make check
```

Postgres runs through `compose.yml` using `postgres:17`. The default compose credentials match `.env.example`.

## Code Layout

Crates use small entrypoint files and implementation modules instead of keeping all logic in one `lib.rs`:

- library crates expose `src/lib.rs` plus focused domain modules such as `src/schema.rs`, `src/service.rs`, or `src/repositories/*.rs`;
- the CLI keeps `src/main.rs` as the binary entrypoint and `src/app.rs` for command execution;
- larger modules should be split further by ENS domain area as functionality grows.
