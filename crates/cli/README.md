# cli

The `cli` crate builds the `ensindexer` production binary. It is intentionally small: operators start the unified service with one command and inspect checkpoint state with one status command.

## Flow

```mermaid
sequenceDiagram
    participant User
    participant CLI as ensindexer
    participant Config as config crate
    participant Storage as storage crate
    participant Server as server crate
    participant Ingest as ingest crate

    User->>CLI: ensindexer start
    CLI->>Config: load .env and apply CLI overrides
    CLI->>CLI: validate strict source/toggle combinations
    CLI->>Storage: connect and run migrations
    CLI->>Server: start API, GraphQL, and playground
    Server->>Ingest: optional backfill/live workers
```

## Commands

- `ensindexer start`: starts health routes, GraphQL API, Apollo Sandbox, and optional indexing workers.
- `ensindexer status`: prints the latest indexed block and per-source checkpoints.

`start` does not accept runtime config flags. Environment variables are the source of truth for Postgres components, RPC URL, HyperSync URL/API key, raw archive directory, chain ID, bind address, backfill/live toggles, the backfill source selector, archive writes, batch size, confirmation depth, backfill/live gap, and polling interval.

## Projection Awareness

The CLI does not project events itself. It validates configuration, opens storage, runs migrations, and delegates all indexing behavior to `server` and `ingest`.

Backfill behavior is controlled by:

- `ENABLE_BACKFILL=true`
- `BACKFILL_SOURCE=rpc|hypersync|raw`
- `ARCHIVE_BACKFILLS=true`
- `RAW_ARCHIVE_DIR=<path>`

Live behavior is controlled by:

- `ENABLE_LIVE_INDEXING=true`

There is no automatic source selection and no `BACKFILL_FROM` or `BACKFILL_TO` setting. RPC and HyperSync resume from database checkpoints; raw replay resumes from archive coverage and database checkpoints.

When backfill and live indexing are enabled together, startup backfill intentionally stops before the live safe head. The target is `latest - INDEXER_CONFIRMATION_DEPTH - BACKFILL_LIVE_GAP_BLOCKS`, and the live worker starts afterward from checkpoint + 1. This prevents skipped blocks while keeping the live worker behind the current chain tip.

## Storage Shape Used

The CLI owns no SQL table definitions. It connects through `storage`, runs the workspace migrations, and reads checkpoint/block status for the `status` command.

## Main Files

- `src/app.rs`: Clap definitions, startup validation, storage connection, and command dispatch.
- `src/main.rs`: Tokio runtime entrypoint with a larger worker stack for GraphQL/indexing workloads.

## Summary

`cli` is the stable production shell around the indexer. Development-only tools such as schema diffing, benchmarks, label healing, and comparison runners should live outside this public binary surface.

## Implemented

- `ensindexer start`.
- `ensindexer status`.
- Env and flag override support for startup config.
- Strict validation for raw replay, archive writes, and HyperSync credentials.
- Automatic migration execution on startup.

## Future Improvements

- Add machine-readable `status --json`.
- Add richer status output for indexing lag, active workers, and archive coverage.
- Move internal benchmark/schema/label-heal tooling into separate dev binaries or scripts.
