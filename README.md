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
`make serve` starts the GraphQL API. Set `SERVE_INDEXER=true` to run the catchup/live indexer in the same process. If `SERVE_BACKFILL_FROM` and `SERVE_BACKFILL_TO` are both set, the server runs that bounded startup backfill before entering the live confirmed-block loop.

Historical backfills use Envio HyperSync automatically when `ENVIO_API_KEY` is set. Set `BACKFILL_SOURCE=rpc` to force JSON-RPC backfills, or `BACKFILL_SOURCE=hypersync` to fail fast unless HyperSync is configured. `HYPERSYNC_URL` defaults to `https://eth.hypersync.xyz`.
Set `RAW_ARCHIVE_DIR` to persist fetched raw logs and block metadata as JSON range files. After changing projection code, use `cargo run -p cli -- replay --from <block> --to <block>` to rebuild from those files without spending RPC or HyperSync credits again.

Indexer commands:

```bash
make status
make backfill BACKFILL_FROM=9380380 BACKFILL_TO=9381380
cargo run -p cli -- replay --from 9380380 --to 9381380
cargo run -p cli -- index
make reset
make check
```

Postgres runs through `compose.yml` using `postgres:17`. The default compose credentials match `.env.example`.

## Docker

Build the unified service image:

```bash
make docker-build
```

Run the API from the image:

```bash
make docker-run
```

The container entrypoint runs `ensindexer serve`. Set `SERVE_INDEXER=true` in `.env` to run startup backfill/live indexing inside the same process as the GraphQL API.

## Code Layout

Crates use small entrypoint files and implementation modules instead of keeping all logic in one `lib.rs`:

- library crates expose `src/lib.rs` plus focused domain modules such as `src/schema.rs`, `src/service.rs`, or `src/repositories/*.rs`;
- the CLI keeps `src/main.rs` as the binary entrypoint and `src/app.rs` for command execution;
- larger modules should be split further by ENS domain area as functionality grows.
