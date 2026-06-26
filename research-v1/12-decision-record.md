# 12 Decision Record

This file captures the major design decisions and the tradeoffs behind them.

## Decision: One Global Tree

Decision:

Store all names in one hierarchical tree.

Why:

- matches DNS/ENS mental model;
- supports parent/child traversal;
- avoids product-specific tables for every chain;
- makes search and exact lookup consistent.

Risk:

- authority conflicts must be modeled carefully.

Mitigation:

- source-family provenance, authority scope, and projection precedence rules.

Verdict:

This is the right model for an ENS indexer.

## Decision: Source Families, Not Product Partitions

Decision:

Basenames, Linea, Celo, ENS v1, ENS v2, and future deployments are source families.

Why:

- each source has unique contracts and chains;
- downstream query model stays shared;
- adding a chain does not require new API concepts.

Risk:

- operators may still want source-specific diagnostics.

Mitigation:

- keep source-family fields on raw facts, normalized events, and debug APIs.

## Decision: Keep Manifest Definitions In Code

Decision:

Use Rust/Alloy source definitions as the executable source of truth, and store immutable DB snapshots for provenance.

Why:

- type safety;
- git review;
- faster runtime;
- auditability for old rows.

Risk:

- DB cannot dynamically configure arbitrary new sources.

Mitigation:

- source additions should be code-reviewed anyway; dynamic source loading can be a later explicit feature.

## Decision: Raw Archives Are First-Class

Decision:

Use binary raw archives as a replay source and optionally store raw metadata in Postgres.

Why:

- avoids repeated provider credits;
- supports projection/adaptor bug repair;
- makes dev/prod replay reproducible.

Risk:

- archive format must remain versioned and validated.

Mitigation:

- include archive format version, checksums, and conversion tools.

## Decision: Current Tables Are Projections

Decision:

Serve APIs from current projection tables.

Why:

- low latency;
- predictable pagination;
- simpler GraphQL resolvers;
- easy cache invalidation.

Risk:

- projection bugs can affect user-visible answers.

Mitigation:

- full projection rebuilds from normalized events;
- projection dead letters;
- query comparison benchmarks.

## Decision: Resource Identity Is Separate From Name Identity

Decision:

Use `resources`, `name_resource_bindings`, and `token_lineages`.

Why:

- wrapping changes authority but not public name;
- ENS v2 token ids can regenerate;
- registrar lapse/re-registration needs lineage;
- permissions attach to resources.

Risk:

- more tables than a simple subgraph clone.

Mitigation:

- keep the tables small and explainable; do not add extra side tables without a real audit/query need.

## Decision: Reorgs Mark Orphaned Rows

Decision:

Do not delete chain-derived rows on reorg. Mark them orphaned and rebuild affected current state.

Why:

- preserves audit history;
- explains why answers changed;
- safer than destructive cleanup.

Risk:

- queries must filter canonicality correctly.

Mitigation:

- current projections only consume non-orphaned eligible facts;
- tests assert orphaned rows are ignored.

## Decision: Cache Is Useful But Not Truth

Decision:

Add cache after projections and invalidation keys are reliable.

Why:

- public API traffic repeats common exact/name/address/search queries;
- ENS events provide clear invalidation points.

Risk:

- stale cache can hide correct DB state.

Mitigation:

- cache entries carry dependency keys and block positions;
- invalidation is event-driven and reorg-aware.
