# 13 Requirement Coverage

| Requirement | Covered By |
| --- | --- |
| No product partitions for Basenames, Linea, Celo, ENS v1, ENS v2, or future chains | `README.md`, `01-architecture-principles.md`, `03-global-name-identity.md`, `12-decision-record.md`, `18-bigname-model-comparison.md` |
| One DNS/ENS-style hierarchical name tree | `README.md`, `03-global-name-identity.md`, `06-projections-and-api.md`, `15-authority-scope-examples.md` |
| Learn from bigname without copying its public split | `18-bigname-model-comparison.md`, `bigname/README.md`, `12-decision-record.md` |
| Support all chains through source-family adapters | `02-source-families-and-contract-patterns.md`, `07-adapter-interface-and-manifests.md`, `09-protocol-specific-mappings.md`, `10-rust-workspace-plan.md` |
| Production-grade auditability | `04-normalized-event-layer.md`, `05-storage-model.md`, `16-production-table-catalog.md`, `19-postgres-storage-schema.md` |
| Raw archive replay without repeated provider calls | `08-backfill-reorgs-and-performance.md`, `16-production-table-catalog.md`, `17-event-flow.md` |
| Reorg-safe indexing | `01-architecture-principles.md`, `08-backfill-reorgs-and-performance.md`, `17-event-flow.md`, `19-postgres-storage-schema.md` |
| Important tables only, with reasons | `05-storage-model.md`, `16-production-table-catalog.md`, `19-postgres-storage-schema.md` |
| Easy-to-understand event flow | `17-event-flow.md`, `14-contract-event-inventory.md`, `09-protocol-specific-mappings.md` |
| GraphQL/API performance through projections | `06-projections-and-api.md`, `16-production-table-catalog.md`, `19-postgres-storage-schema.md` |
| Clear decisions and tradeoffs | `12-decision-record.md`, `18-bigname-model-comparison.md` |
| Edge cases and test plan | `11-edge-cases-and-tests.md` |
