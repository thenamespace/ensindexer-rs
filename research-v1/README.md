# ensindexer-rs v1 Research

This folder is the production design guide for `ensindexer-rs`.

The core product model is one DNS/ENS-style hierarchical tree:

```text
.
eth
base.eth
alice.base.eth
sub.alice.base.eth
```

There are no product-level buckets for Basenames, Linea names, Celo names, ENS v1, ENS v2, or future deployments. Those are source families: different chains and contracts that emit facts about names in the same tree.

## The Design In One Page

```text
source-family definitions in Rust
  -> active manifest snapshot in Postgres
  -> chain intake by block hash
  -> raw archive / raw log metadata
  -> source adapter
  -> normalized events
  -> name/resource/token identity
  -> projection invalidations
  -> current read tables
  -> GraphQL / HTTP APIs
```

The important split is simple:

- raw data proves what came from the chain;
- normalized events explain what the raw data means;
- identity tables preserve stable names, authority resources, and token history;
- projections make queries fast;
- audit tables explain how the indexer got into its current state.

## Non-Negotiable Product Rule

`alice.base.eth` is a child of `base.eth`.

It does not become a separate product partition because Base emitted the facts. The source family is Base/Basenames; the name still belongs in the global tree.

The same rule applies to every future chain:

- `x.linea.eth` is a child under `linea.eth`;
- `x.celo.eth` is a child under `celo.eth`;
- ENS v2 resources still describe names in the same tree;
- CCIP-read or L2-managed records are source behavior, not a separate name model.

## Reading Order

1. [Architecture Principles](01-architecture-principles.md)
2. [Source Families And Contract Patterns](02-source-families-and-contract-patterns.md)
3. [Global Name Identity](03-global-name-identity.md)
4. [Normalized Event Layer](04-normalized-event-layer.md)
5. [Storage Model](05-storage-model.md)
6. [Projections And API](06-projections-and-api.md)
7. [Adapter Interface And Manifests](07-adapter-interface-and-manifests.md)
8. [Backfill, Reorgs, And Performance](08-backfill-reorgs-and-performance.md)
9. [Protocol-Specific Mappings](09-protocol-specific-mappings.md)
10. [Rust Workspace Plan](10-rust-workspace-plan.md)
11. [Edge Cases And Tests](11-edge-cases-and-tests.md)
12. [Decision Record](12-decision-record.md)
13. [Requirement Coverage](13-requirement-coverage.md)
14. [Contract Event Inventory](14-contract-event-inventory.md)
15. [Authority Scope Examples](15-authority-scope-examples.md)
16. [Production Table Catalog](16-production-table-catalog.md)
17. [Event Flow](17-event-flow.md)
18. [Bigname Model Comparison](18-bigname-model-comparison.md)
19. [Postgres Storage Schema](19-postgres-storage-schema.md)

Reference material:

- [Bigname Architecture Knowledgebase](bigname/README.md)

## What We Learned From Bigname

Keep:

- raw facts, normalized events, and projections as separate layers;
- block-hash based chain lineage for reorg safety;
- stable resource identity separate from public name text;
- token lineage separate from resource identity;
- route-shaped projections instead of query-time scans;
- durable execution traces for verified resolution;
- repair/replay workflows that never patch current tables by hand.

Do not copy:

- bigname's public product split;
- source-family-specific public identity;
- storing every possible operational side table before the indexer needs it.

## Practical Target

This design is for a production-grade experimental indexer:

- auditable enough to explain every row;
- safe enough to survive reorgs and restarts;
- fast enough to serve public comparison traffic;
- modular enough to add ENS v2 and L2-managed names;
- simple enough that the table model can be understood and operated.
