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
cargo test --workspace
cargo run -p cli -- serve
```

Configuration is loaded from `.env` via `config`.

Indexer commands:

```bash
cargo run -p cli -- migrate
cargo run -p cli -- status
cargo run -p cli -- backfill --from 9380380 --to 9381000
cargo run -p cli -- index
cargo run -p cli -- reset --yes
```

## Code Layout

Crates use small entrypoint files and implementation modules instead of keeping all logic in one `lib.rs`:

- library crates expose `src/lib.rs` plus focused domain modules such as `src/schema.rs`, `src/service.rs`, or `src/repositories/*.rs`;
- the CLI keeps `src/main.rs` as the binary entrypoint and `src/app.rs` for command execution;
- larger modules should be split further by ENS domain area as functionality grows.
