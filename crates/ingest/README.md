# ingest

Historical and live chain ingestion crate.

## Responsibility

`ingest` fetches historical logs and block metadata from HyperSync or Ethereum RPC, decodes logs with `contracts`, applies ordered events through `projection`, and updates storage checkpoints. Live indexing still uses Ethereum RPC for head tracking and canonical parent checks.

## Modules

- `service`: backfill and live indexing orchestration.
- `sources`: fixed source definitions, start blocks, and topic sets.
- `rpc`: Alloy provider helpers for logs and block metadata.
- `hypersync`: Envio HyperSync historical log and block metadata adapter.
- `archive`: filesystem JSON archives for replaying fetched ranges.
- `decode`: conversion from raw logs plus metadata into projection-ready events.

## Architecture Notes

Backfills fetch bounded ranges, merge logs from all active sources, sort by canonical chain order, and apply decoded events deterministically. `BACKFILL_SOURCE` is explicit and strict: `rpc` uses Alloy JSON-RPC, `hypersync` uses Envio HyperSync and requires `ENVIO_API_KEY`, and `raw` replays archive JSON files. There is no automatic source selection.

When `ARCHIVE_BACKFILLS=true`, each fetched range is written to `RAW_ARCHIVE_DIR/ranges/{from}-{to}.json` after log and block metadata fetching. Replay reads those range files and runs the same decode/projection/checkpoint path as live backfill without touching RPC or HyperSync. This is intended for projection development: archive once, reset indexed state, change projection code, then replay until the output matches the official subgraph.

Live indexing runs behind a configurable confirmation depth and verifies parent hashes before applying new ranges. `INDEXING_SOURCE=http_rpc` uses `ETH_RPC_URL`; `INDEXING_SOURCE=wss` uses `ETH_WS_URL`. Current reorg repair uses a coarse indexed-state reset followed by canonical rebuild.

## Boundary Rules

- This crate owns chain IO, batching, canonical ordering, checkpoints, and live polling.
- This crate should not expose GraphQL or know API DTOs.
- This crate should not contain projection business logic beyond invoking the dispatcher in block/log order.
- Chain IO should stay behind small helpers so provider behavior can be replaced or mocked in tests.

## Indexing Flow

Backfill and live indexing use the same core path:

1. Resolve the inclusive block range to index.
2. Build active fixed sources for that range.
3. Fetch logs in bounded batches from HyperSync or Alloy RPC.
4. Attach block metadata needed by projection and `_meta`.
5. Optionally archive raw logs and block metadata for local replay.
6. Decode logs into `contracts::EnsEvent`.
7. Sort by block number, transaction index, log index.
8. Apply events through `projection`.
9. Persist block/checkpoint progress only after successful projection.

## Testing Approach

Unit-test source selection and ordering with synthetic logs. Integration-test the 1000-block backfill path against a real RPC endpoint and local Postgres, then compare a small set of resulting GraphQL reads against the official ENS subgraph.
