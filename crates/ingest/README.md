# ingest

Historical and live chain ingestion crate.

## Responsibility

`ingest` fetches logs and block metadata from Ethereum RPC, decodes logs with `contracts`, applies ordered events through `projection`, and updates storage checkpoints.

## Modules

- `service`: backfill and live indexing orchestration.
- `sources`: fixed source definitions, start blocks, and topic sets.
- `rpc`: Alloy provider helpers for logs and block metadata.
- `decode`: conversion from raw logs plus metadata into projection-ready events.

## Architecture Notes

Backfills fetch bounded ranges, merge logs from all active sources, sort by canonical chain order, and apply decoded events deterministically. Live indexing runs behind a configurable confirmation depth and verifies parent hashes before applying new ranges. Current reorg repair uses a coarse indexed-state reset followed by canonical rebuild.

## Boundary Rules

- This crate owns chain IO, batching, canonical ordering, checkpoints, and live polling.
- This crate should not expose GraphQL or know API DTOs.
- This crate should not contain projection business logic beyond invoking the dispatcher in block/log order.
- RPC calls should stay behind small helpers so provider behavior can be replaced or mocked in tests.

## Indexing Flow

Backfill and live indexing use the same core path:

1. Resolve the inclusive block range to index.
2. Build active fixed sources for that range.
3. Fetch logs in bounded batches from Alloy RPC.
4. Attach block metadata needed by projection and `_meta`.
5. Decode logs into `contracts::EnsEvent`.
6. Sort by block number, transaction index, log index.
7. Apply events through `projection`.
8. Persist block/checkpoint progress only after successful projection.

## Testing Approach

Unit-test source selection and ordering with synthetic logs. Integration-test the 1000-block backfill path against a real RPC endpoint and local Postgres, then compare a small set of resulting GraphQL reads against the official ENS subgraph.
