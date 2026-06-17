# Operations

This page describes how to run the indexer locally or in production-like environments.

## Setup

```bash
cp .env.example .env
cargo make db-up
cargo make start
```

`cargo make db-up` starts Postgres through Docker Compose. `cargo make start` runs the production `ensindexer start` command.

## Required Configuration

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ensindexer
ETH_RPC_URL=https://...
CHAIN_ID=1
BIND_ADDRESS=127.0.0.1:8080
```

Optional but commonly used:

```env
ETH_WS_URL=wss://...
ENVIO_API_KEY=...
HYPERSYNC_URL=https://eth.hypersync.xyz
RAW_ARCHIVE_DIR=.raw-archive-full
```

## Indexing Toggles

Indexing is explicit. There is no automatic source mode.

```env
ENABLE_BACKFILL=true
BACKFILL_SOURCE=hypersync    # rpc | hypersync | raw
ENABLE_LIVE_INDEXING=false
LIVE_INDEXING_SOURCE=rpc     # rpc | wss
ARCHIVE_BACKFILLS=false
```

`BACKFILL_FROM` and `BACKFILL_TO` do not exist. The indexer resumes from database source checkpoints. Raw replay also uses archive coverage from `manifest.json`.

## Command Surface

```bash
ensindexer start
ensindexer status
```

`start` always starts:

- `/graphql` GraphQL endpoint and Apollo Sandbox;
- `/subgraph` POST endpoint for subgraph clients;
- `/healthz`;
- `/readyz`.

Backfill/live workers are optional and controlled by env or flags.

## Archive Once, Replay Many Times

Fetch logs once with HyperSync or RPC and archive them:

```bash
ENABLE_BACKFILL=true \
BACKFILL_SOURCE=hypersync \
ARCHIVE_BACKFILLS=true \
RAW_ARCHIVE_DIR=.raw-archive-full \
cargo make start
```

Replay the local archive into a fresh database:

```bash
cargo make db-reset
cargo make db-up

ENABLE_BACKFILL=true \
BACKFILL_SOURCE=raw \
RAW_ARCHIVE_DIR=.raw-archive-full \
cargo make start
```

The archive path must contain:

```text
.raw-archive-full/
  manifest.json
  ranges/
    00000000000003327417-00000000000003352416.bin
    ...
```

If raw replay says no range files were found, verify that `RAW_ARCHIVE_DIR` points to the archive root, not to `ranges/`.

## Live Indexing

Live indexing uses confirmed ranges:

```env
ENABLE_LIVE_INDEXING=true
LIVE_INDEXING_SOURCE=wss
ETH_WS_URL=wss://...
INDEXER_CONFIRMATION_DEPTH=12
LIVE_POLL_SECONDS=12
```

The live loop checks parent hashes before applying the next range. The current repair strategy is coarse: reset indexed state and rebuild from source starts. Efficient common-ancestor rollback is future work.

## Status

```bash
cargo make status
```

Status prints:

- latest indexed block in `blocks`;
- per-source checkpoint rows.

Backfills resume from source checkpoints, not from the latest row in `blocks`. If an interrupted run wrote block metadata but did not update checkpoints, the next backfill can re-apply that range idempotently.

## Logging

Default CLI logging is:

```text
info,sqlx=error
```

This keeps app logs visible and hides noisy SQLx notices/warnings. For debugging:

```bash
RUST_LOG=server=info,ingest=debug,storage=debug,sqlx=warn cargo make start
```

## Docker

```bash
cargo make docker-build
cargo make docker-run
```

The image entrypoint runs `ensindexer start`. Use `.env` for runtime behavior.

## Local Database Reset

```bash
cargo make db-reset
cargo make db-up
```

`db-reset` deletes the local Docker Compose Postgres volume. Use it only for disposable development databases.

## Operational Notes

- Raw replay intentionally drops secondary query indexes during bulk replay and recreates them afterward.
- If you interrupt raw replay, recreate indexes before doing query benchmarks.
- The server binds HTTP before spawning indexing workers, so a port collision will fail before raw replay can mutate the database.
- Use release builds for meaningful throughput tests.
- Keep benchmark tooling outside the production CLI.
