# Future Work

This page summarizes the important remaining work. The authoritative running checklist is [`../TODOs.md`](../TODOs.md).

## Compatibility

- Add regression tests for `block.number`, `block.hash`, and `block.number_gte` across all entity and event roots.
- Verify exact Graph Node semantics for `block.number_gte`.
- Audit nested relationship fields under historical entity results.
- Complete recursive trailing-underscore relationship filter audits.
- Audit less common scalar operators across every official input type.
- Add official/local differential test reports for representative mainnet domains, resolvers, registrations, wrappers, and event history.

## Projection Correctness

- Build seeded fixture tests from known mainnet receipts for each event family.
- Add official-subgraph response comparisons for selected backfill ranges.
- Audit wrapper unwrap/re-wrap behavior and same-block repeated mutations.
- Add structured projection trace tooling for debugging one entity across a block range.
- Decide whether local ENSRainbow dictionary healing should become a supported maintenance binary.

## Reorg And Live Indexing

- Replace coarse reset-and-rebuild with common-ancestor rollback.
- Use snapshots or reversible change payloads to roll back without full replay.
- Add stronger source retry/backoff policies.
- Add lag reporting and failure counters.
- Add graceful shutdown for active backfill/live ranges.

## Raw Replay And Backfill Scale

- Add per-table flush timings.
- Profile dense 250k+ and 500k+ log ranges with flamegraph or equivalent tooling.
- Evaluate `COPY` or staging-table merge paths.
- Evaluate unlogged staging for historical replay.
- Add adaptive range sizing.
- Make index restoration robust on interrupted replay.

## GraphQL API Quality

- Add query complexity/depth limits before public deployment.
- Expand DataLoader batching to remaining relationship hydration paths.
- Add pagination stress tests for large event-interface unions.
- Add query-plan tests for exact name, labelhash, parent traversal, relationship filters, and ENSJS address lookups.
- Design specialized search support for broad substring workloads.

## Operations

- Add `status --json`.
- Add richer status output for lag, active workers, archive coverage, and index state.
- Add Prometheus/OpenTelemetry metrics.
- Add production Docker build-and-serve verification.
- Add redacted config diagnostics at startup.

## Documentation

- Keep `docs/` current with implemented behavior.
- Keep `research/` as provenance, not operational guidance.
- Keep benchmark result tables in `benchmarks/README.md`.
- Keep crate READMEs focused on crate internals.
