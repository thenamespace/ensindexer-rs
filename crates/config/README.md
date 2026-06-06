# config

Runtime configuration crate.

## Responsibility

`config` loads `.env` values and exposes a typed `AppConfig` shared by the CLI, server, and ingest service.

## Modules

- `env`: `.env` loading, required and optional environment parsing, and config errors.

## Architecture Notes

Configuration is parsed once at command startup and then passed into dependent crates. `DATABASE_URL` and `ETH_RPC_URL` are required. Historical backfill transport is controlled by `BACKFILL_SOURCE`, with `auto` selecting HyperSync when `ENVIO_API_KEY` or `ENVIO_API_TOKEN` is present. `HYPERSYNC_URL` defaults to the Ethereum mainnet endpoint. Runtime knobs such as confirmation depth, batch size, poll interval, bind address, and Apollo Sandbox enablement have defaults. `GRAPHQL_SANDBOX` is the preferred UI flag; `GRAPHQL_PLAYGROUND` remains accepted as a backward-compatible fallback.

## Boundary Rules

- This crate owns environment parsing and default values only.
- This crate should not create database pools, providers, routers, or indexer services.
- Configuration values should be typed before leaving this crate.
- New required variables must be added to `.env.example` and documented in the root README.

## Testing Approach

Use environment-isolated unit tests for required variables, defaults, invalid numeric values, and backward-compatible aliases. Avoid tests that rely on the developer's real `.env`.
