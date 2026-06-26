# 08 Backfill, Reorgs, And Performance

Backfill, live indexing, and raw archive replay must use the same downstream pipeline.

```text
RPC / HyperSync / raw archive
  -> raw facts
  -> adapters
  -> normalized events
  -> identity
  -> projections
```

If these paths diverge, bugs will only appear in production.

## Backfill Sources

Supported sources:

- HyperSync for fast historical log fetching;
- RPC for correctness fallback and small ranges;
- binary raw archives for repeatable local/dev/prod replay;
- live polling from HTTP RPC for new blocks.

Avoid WebSocket-only live indexing for the first production version. Polling with checkpoints and confirmation depth is easier to reason about and recover.

## Raw Archive Strategy

Raw archives are first-class.

They should store enough data to replay adapter logic without paying provider credits again:

- chain id;
- block range;
- block hash metadata where available;
- logs;
- transaction hash/log index;
- contract address;
- topics/data;
- source-family/watch metadata if known;
- archive format version;
- checksum.

Use binary format for scale. JSON is useful for debugging, but not for full mainnet replay.

## Checkpoints

Track progress separately:

- raw intake checkpoint;
- adapter/normalized replay checkpoint;
- projection checkpoint;
- live chain checkpoint;
- archive range manifest.

Why separate checkpoints:

- raw archive can be complete while projections are behind;
- adapter logic can be replayed without refetching raw logs;
- live indexing can continue while projection repair catches up;
- failures become visible at the correct layer.

## Startup Behavior

On startup:

1. start API server;
2. connect to DB;
3. load source manifest snapshot;
4. check chain/source checkpoints;
5. if backfill is enabled, resume from checkpoint;
6. if live indexing is enabled, poll latest confirmed blocks;
7. enqueue projection work;
8. report health by layer.

Do not drop/recreate indexes on every startup.

Bulk index maintenance should only happen for explicit bulk replay modes or controlled maintenance jobs.

## Reorg Handling

Every observed block is identified by hash.

When a new block conflicts with stored lineage:

```text
detect parent/hash mismatch
  -> find common ancestor
  -> mark losing branch facts orphaned
  -> mark affected normalized events orphaned
  -> invalidate projections and cache
  -> ingest winning branch
  -> rebuild affected projections
```

Do not delete old rows by default. Marking rows orphaned preserves audit history.

## Confirmation Depth

For public current state, use a confirmation depth appropriate to the chain.

Examples:

- Ethereum mainnet: small but nonzero for safer public state;
- local/dev: zero is acceptable;
- L2s: source-family specific.

The setting controls when facts become eligible for current projections. It should not prevent raw observed facts from being stored.

## Performance Rules

Fast backfill depends on:

- batch decoding;
- bulk inserts/copy;
- preloading current identity/resource state per range;
- minimizing per-row SQL;
- deferring non-critical secondary index creation only in explicit bulk modes;
- partitioning large append-only tables;
- writing projections in batches;
- avoiding API-time event scans.

Expected bottlenecks:

- Postgres writes for dense ranges;
- projection recompute for high-fanout resolver/name changes;
- index rebuilds;
- network/provider latency during non-archive backfill.

## Production Safety

Before public deployment:

- use persistent Postgres;
- disable destructive reset paths in server startup;
- run migrations independently or behind explicit flags;
- keep backups on;
- monitor checkpoints, range lag, projection lag, dead letters, and DB size;
- keep raw archives in durable storage if they are part of recovery.
