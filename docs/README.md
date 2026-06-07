# ENS Indexer Knowledgebase

This directory is the source-grounded implementation guide for building a custom ENS indexer in Rust without rindexer or The Graph runtime.

The source of truth for this research is the official ENS subgraph checked into:

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

## Current Implementation Scaffold

The workspace uses unprefixed crate names: `types`, `contracts`, `config`, `storage`, `projection`, `ingest`, `api`, `server`, and `cli`. The `config` crate owns `.env` loading and typed runtime configuration so the CLI, server, and ingest jobs share one configuration path.

The implementation currently has typed storage repositories and GraphQL DTOs for mutable entities, concrete event entities, GraphQL event interfaces with derived entity event relationships, `_meta`, current-state `block`/`subgraphError` compatibility arguments on entity and event roots, generated-style scalar filters for the main stored entity fields, relationship ordering, `and/or` composition for `AccountFilter`, account relationship filters, scalar-compatible `DomainFilter`, `RegistrationFilter`, `ResolverFilter`, `WrappedDomainFilter`, and event filter predicates, concrete and event-interface-specific filters, event and mutable-entity `_change_block` predicates, projection-maintained `entity_changes` records, event parent/owner/addr scalar operators, event-specific relation scalar operators for `parentDomain`, new-resolver `resolver`, `registrant`, and `newOwner` columns, event relation predicates for domain/account/resolver/registration-backed columns, shallow trailing-underscore relationship filters, recursive `DomainFilter` relationship predicates, `Domain_filter.registration_` and `Domain_filter.wrappedDomain_` derived one-to-one relation predicates, relation-aware `RegistrationFilter`, `WrappedDomainFilter`, and `ResolverFilter` composition, operational CLI commands, and a confirmation-depth live indexing loop with parent-hash reorg detection. Reorg repair currently performs a coarse full indexed-state reset and canonical rebuild. The remaining API parity work is historical block reads and derived collection filters such as `subdomains_`/`events_`; the remaining indexing hardening work is efficient common-ancestor rollback and differential validation.

## Compatibility Goal

The official subgraph exposes an entity graph, not just raw logs. A drop-in Rust replacement must reproduce:

- the same top-level entities and event entities;
- the same entity IDs;
- the same derived relationships;
- the same historical event projection behavior;
- the same edge cases around old registry migration, resolver IDs, `.eth` registration expiry, wrapped-domain fuses, and invalid labels.

The Graph has derived fields and interfaces built into its query layer. In Rust, those must be modeled explicitly in the database schema and GraphQL resolvers.
