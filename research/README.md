# ENS Indexer Research Archive

This directory preserves the original source-grounded research that informed the Rust indexer. It is kept for provenance and deeper official-subgraph comparison. The maintained operator and implementation knowledge base now lives in [`../docs`](../docs).

The source of truth for this research was the official ENS subgraph checked into:

- `.repos/ens-subgraph/subgraph.yaml`
- `.repos/ens-subgraph/schema.graphql`
- `.repos/ens-subgraph/src/ensRegistry.ts`
- `.repos/ens-subgraph/src/ethRegistrar.ts`
- `.repos/ens-subgraph/src/nameWrapper.ts`
- `.repos/ens-subgraph/src/resolver.ts`
- `.repos/ens-subgraph/src/utils.ts`

## Documents

- [Official Subgraph Overview](official-subgraph-overview.md)
  - indexed contracts, addresses, start blocks, event signatures, and runtime assumptions.
- [Schema and GraphQL Shape](schema-and-graphql-shape.md)
  - entity definitions, interfaces, IDs, relationships, GraphQL DTO shapes, and SQL-oriented table notes.
- [Projection Logic](projection-logic.md)
  - exact handler behavior for registry, registrar/controllers, name wrapper, resolver, event IDs, name construction, and edge cases.
- [Rust Implementation Notes](rust-implementation-notes.md)
  - suggested crate boundaries, alloy/sqlx DTOs, ingestion model, ordering concerns, and compatibility checklist.
- [Rust Implementation Roadmap](implementation-roadmap.md)
  - step-by-step workspace structure, crate responsibilities, database/API plan, filter implementation, testing strategy, and suggested delivery order.

## How To Use This Archive

Use this directory when you need to answer questions like:

- What did the official ENS subgraph mappings do?
- Which contracts and events were originally researched?
- How were the first Rust crate boundaries and schemas derived?
- Which compatibility edge cases were identified before implementation?

Use [`../docs`](../docs) when you need to run, extend, debug, or benchmark the current Rust codebase.

## Compatibility Goal Preserved Here

The official subgraph exposes an entity graph, not just raw logs. The research here tracks why a drop-in Rust replacement must reproduce:

- the same top-level entities and event entities;
- the same entity IDs;
- the same derived relationships;
- the same historical event projection behavior;
- the same edge cases around old registry migration, resolver IDs, `.eth` registration expiry, wrapped-domain fuses, and invalid labels.

The Graph has derived fields and interfaces built into its query layer. In Rust, those must be modeled explicitly in the database schema and GraphQL resolvers.
