# config

Runtime configuration crate.

## Responsibility

`config` loads `.env` values and exposes a typed `AppConfig` shared by the CLI, server, and ingest service.

## Modules

- `env`: `.env` loading, required and optional environment parsing, and config errors.

## Architecture Notes

Configuration is parsed once at command startup and then passed into dependent crates. `DATABASE_URL` and `ETH_RPC_URL` are required. `ETH_WS_URL` is required only when `INDEXING_SOURCE=wss`. Historical backfill transport is controlled by strict `BACKFILL_SOURCE=rpc|hypersync|raw`; there is no automatic HyperSync fallback. `HYPERSYNC_URL` defaults to the Ethereum mainnet endpoint and `ENVIO_API_KEY` is required only for HyperSync. Serve-time work is controlled by `ENABLE_BACKFILL` and `ENABLE_LIVE_INDEXING`. `BACKFILL_FROM` and `BACKFILL_TO` are optional and default to the first ENS source block and latest available target. `ARCHIVE_BACKFILLS=true` plus `RAW_ARCHIVE_DIR` writes filesystem raw-log archives for replaying projection changes without chain IO. Apollo Sandbox is always served by the HTTP server.

## Boundary Rules

- This crate owns environment parsing and default values only.
- This crate should not create database pools, providers, routers, or indexer services.
- Configuration values should be typed before leaving this crate.
- New required variables must be added to `.env.example` and documented in the root README.

## Testing Approach

Use environment-isolated unit tests for required variables, defaults, invalid numeric values, and backward-compatible aliases. Avoid tests that rely on the developer's real `.env`.
