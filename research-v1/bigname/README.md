# Bigname Architecture Knowledgebase

This folder documents how bigname processes data from top to bottom.

It is reference research, not the final `ensindexer-rs` design. Bigname's own public product split is documented here because that is how bigname works. The final v1 design for this indexer is the root guide in `research-v1/README.md`: one global DNS/ENS-style tree, source-family adapters, no source-specific public identity.

## Source Material

Primary sources inspected:

- `.repos/bigname/docs/architecture.md`
- `.repos/bigname/docs/chain-intake.md`
- `.repos/bigname/docs/storage.md`
- `.repos/bigname/docs/projections.md`
- `.repos/bigname/docs/manifests.md`
- `.repos/bigname/docs/execution.md`
- `.repos/bigname/docs/adrs/0002-surface-resource-identity.md`
- `.repos/bigname/migrations/*.sql`
- `.repos/bigname/manifests/**/*.toml`
- `.repos/bigname/crates/adapters/src/**`
- `.repos/bigname/crates/storage/src/**`
- `.repos/bigname/crates/execution/src/**`
- `.repos/bigname/crates/manifests/src/**`

## Reading Order

1. [01 System Model](01-system-model.md)
2. [02 Storage Model](02-storage-model.md)
3. [03 Chain Intake, Backfill, Reorgs](03-chain-intake-backfill-reorgs.md)
4. [04 Manifests And Discovery](04-manifests-and-discovery.md)
5. [05 Adapters And Event Handling](05-adapters-and-event-handling.md)
6. [06 Identity And Normalization](06-identity-and-normalization.md)
7. [07 Projections And API Reads](07-projections-and-api-reads.md)
8. [08 Execution And Cache](08-execution-and-cache.md)
9. [09 Operational Replay And Repair](09-operational-replay-and-repair.md)
10. [10 Lessons For ensindexer-rs](10-lessons-for-ensindexer-rs.md)
11. [11 Manifest Event Catalog](11-manifest-event-catalog.md)

## One-Screen Flow

```text
selected manifest profile
  -> manifest sync
  -> contract instances and discovery graph
  -> watch plan
  -> chain intake by block hash
  -> chain_lineage
  -> selected raw logs / tx / receipts / code / call snapshots
  -> adapter routing
  -> identity rows and normalized_events
  -> projection_normalized_event_changes
  -> projection_invalidations
  -> current projection rebuilds
  -> API reads projections and execution outcomes
```

## Most Important Concepts

- Bigname is not an ENS subgraph clone. It is a replayable indexing system with native APIs.
- Bigname uses a source/product split for public identity. This is useful to understand, but it is not copied by the v1 design.
- Four identity anchors matter: `logical_name_id`, `resource_id`, `token_lineage_id`, `contract_instance_id`.
- Chain block hash is truth. Block number is position.
- Raw facts are durable or staging depending on retention mode.
- Normalized events are adapter-owned semantic events.
- Projections are rebuildable read models.
- Reorg repair marks rows orphaned rather than deleting audit history.
- Backfill and live ingestion share the same downstream pipeline.
- Execution traces are durable audit artifacts; execution outcomes are reusable only while their block-hash dependencies remain canonical.
