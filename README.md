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
Open [http://127.0.0.1:8080/graphql](http://127.0.0.1:8080/graphql) in a browser for Apollo Sandbox. The Sandbox is always available in dev and prod.
`make serve` starts the GraphQL API. Set `ENABLE_BACKFILL=true` to run a startup catchup backfill in the same process, and set `ENABLE_LIVE_INDEXING=true` to keep indexing confirmed live ranges after startup. If both toggles are enabled, startup backfill runs before live indexing.

`BACKFILL_SOURCE` is strict: `rpc`, `hypersync`, or `raw`. There is no automatic transport selection. `BACKFILL_FROM` and `BACKFILL_TO` are optional for serve-time backfills and `cargo run -p cli -- backfill`; omitted `BACKFILL_FROM` defaults to the earliest ENS indexed source block, omitted `BACKFILL_TO` defaults to the latest RPC block for `rpc`/`hypersync`, and raw replay can infer archive bounds.
Set `ARCHIVE_BACKFILLS=true` and `RAW_ARCHIVE_DIR=.raw-archive` to persist fetched raw logs and block metadata as JSON range files. A first run can use `BACKFILL_SOURCE=hypersync` plus archiving; a later fresh database can use `BACKFILL_SOURCE=raw` to replay those archived files without RPC or HyperSync credits. `INDEXING_SOURCE` controls live indexing and must be `http_rpc` or `wss`; `wss` requires `ETH_WS_URL`.

Indexer commands:

```bash
make status
cargo run -p cli -- backfill --from 9380380 --to 9381380
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

The container entrypoint runs `ensindexer serve`. Use `ENABLE_BACKFILL` and `ENABLE_LIVE_INDEXING` in `.env` to run startup backfill and live indexing inside the same process as the GraphQL API.

## Code Layout

Crates use small entrypoint files and implementation modules instead of keeping all logic in one `lib.rs`:

- library crates expose `src/lib.rs` plus focused domain modules such as `src/schema.rs`, `src/service.rs`, or `src/repositories/*.rs`;
- the CLI keeps `src/main.rs` as the binary entrypoint and `src/app.rs` for command execution;
- larger modules should be split further by ENS domain area as functionality grows.
