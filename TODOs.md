# ENS Indexer TODOs

Running implementation and compatibility checklist for the custom Rust ENS indexer. Keep this file updated after each meaningful implementation slice.

Last full verification: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p storage`, `cargo test -p api`, and `cargo check --workspace` passed after adding API DataLoader batching for hot `Domain` account/resolver relationships and cleaning the production CLI/docs. Schema diff previously had no missing fields, args, input fields, enum values, or mismatched query arg types; the only extra type remains `Aggregation_current`. Archive backfill and checksum-backed raw replay were validated locally for blocks `9380380..9380390`. A 1,000-block HyperSync archive backfill was run for `9380380..9381380`; archive coverage reports no gaps. Full mainnet raw replay later reached block `25270169`; exact domain-name GraphQL lookup was optimized from roughly 10 seconds to roughly 7-23ms on the local full database. ENSJS-style `getNamesForAddress` over the full local database was optimized from roughly 9-17 seconds to roughly 18-22ms warm through `/subgraph`. A release-mode benchmark suite now covers 11 ENSJS/ENSNode/official-subgraph query workloads with baseline-adjusted hosted timings; the current production run has 9/11 local in-process query medians under 32ms, with broad substring searches still at roughly 161-173ms before DataLoader relationship batching.

## Latest Production Benchmark

The production binary intentionally has no benchmark command. The latest benchmark used the query fixtures in `benchmarks/queries` with external tooling against the local `/subgraph`, hosted ENSNode, and hosted The Graph endpoints.

The local column is in-process Rust GraphQL compute time. Hosted columns use provider timing if exposed, otherwise baseline-adjusted wall time; The Graph and ENSNode did not expose provider timing in this run.

| operation | ensindexer-rs | ensnode | the graph indexer |
| --- | ---: | ---: | ---: |
| `01-domain-batch` | 5.985ms (13.5x faster than slowest, 92.6% lower) | 12.836ms (6.3x faster than slowest, 84.1% lower) | 80.771ms (slowest) |
| `02-names-for-address` | 18.722ms (10.8x faster than slowest, 90.7% lower) | 13.398ms (15.0x faster than slowest, 93.4% lower) | 201.511ms (slowest) |
| `03-eth-subnames` | 31.281ms (60.9x faster than slowest, 98.4% lower) | 1903.604ms (slowest) | 234.277ms (8.1x faster than slowest, 87.7% lower) |
| `04-subnames-search` | 161.328ms (10.7x faster than slowest, 90.6% lower) | 1724.506ms (slowest) | 504 timeout |
| `05-decoded-label` | 1.843ms (54.0x faster than slowest, 98.1% lower) | 64.406ms (1.5x faster than slowest, 35.3% lower) | 99.493ms (slowest) |
| `06-resolver-records` | 10.922ms (19.9x faster than slowest, 95.0% lower) | 94.708ms (2.3x faster than slowest, 56.3% lower) | 216.811ms (slowest) |
| `07-registrations` | 9.260ms (116.6x faster than slowest, 99.1% lower) | 1079.505ms (slowest) | 233.749ms (4.6x faster than slowest, 78.3% lower) |
| `08-name-history` | 5.777ms (16.5x faster than slowest, 93.9% lower) | 11.905ms (near-baseline/noisy; 8.0x faster than slowest, 87.5% lower) | 95.292ms (slowest) |
| `09-event-scan` | 28.043ms (232.0x faster than slowest, 99.6% lower) | unsupported | 6506.291ms (slowest) |
| `10-relationship-filter` | 6.828ms (34.1x faster than slowest, 97.1% lower) | unsupported | 233.165ms (slowest) |
| `11-text-search` | 172.538ms (3.7x faster than slowest, 73.2% lower) | 642.963ms (slowest) | 144.134ms (4.5x faster than slowest, 77.6% lower) |

Benchmark notes:

- [x] The Graph hosted gateway benchmark ran with a measured `_meta` baseline median of roughly 381ms.
- [x] ENSNode hosted benchmark ran with a measured `_meta` baseline median of roughly 361ms.
- [x] ENSNode `08-name-history` was rerun after a suspicious near-1ms adjusted result. The fresh 25-sample probe measured raw wall median `656.481ms`, baseline median `644.576ms`, and adjusted median `11.905ms`; this is still near-baseline hosted timing noise rather than precise provider compute.
- [x] ENSNode-only benchmark compatibility rewrites were isolated in the removed internal benchmark runner; canonical local fixtures remain aligned with this indexer's schema/source of truth.
- [x] ENSNode `09-event-scan` and `10-relationship-filter` are recorded as unsupported because its public alpha schema does not expose those official-compatible query shapes.
- [x] The Graph `04-subnames-search` is recorded as `504 timeout` for this production run, not unsupported; the provider can run nearby queries but timed out under the chosen release benchmark settings.
- [x] After API DataLoader batching for `Domain` account/resolver relationships, local release benchmark slices improved `04-subnames-search` from roughly 161ms to 138ms median and `11-text-search` from roughly 173ms to 166ms median. This reduces N+1 relationship hydration but does not fully solve broad substring search cost.
- [x] After expanding API DataLoader batching to `Domain.registration` and `Domain.wrappedDomain`, local release slices measured `01-domain-batch` at roughly 5.1ms, `02-names-for-address` at roughly 5.4ms, and `03-eth-subnames` at roughly 7.3ms.

## Completed

### Workspace And Operations

- [x] Cargo workspace uses unprefixed crate names: `types`, `contracts`, `config`, `storage`, `projection`, `ingest`, `api`, `server`, and `cli`.
- [x] `.env.example` exists and runtime configuration is read from `.env`.
- [x] Docker Compose includes Postgres 17.
- [x] `cargo-make` provides common commands for setup, serving, status, Docker, tests, and checks.
- [x] Apollo Sandbox is used for the GraphQL UI instead of GraphiQL and is always served on `GET /graphql`.
- [x] Each crate has a README describing its purpose, architecture, and responsibilities.
- [x] Root docs describe the official ENS subgraph schema, projection logic, and Rust implementation plan.
- [x] Repository files are split into modules rather than single large `lib.rs` files.
- [x] Current touched implementation files are kept under the requested 300-500 line range.

### Chain Contracts And Ingestion

- [x] Alloy-based event ABI decoding is implemented in `contracts`.
- [x] Historical ingestion can fetch from RPC.
- [x] Historical ingestion can fetch from HyperSync when configured.
- [x] Raw log archive writing is supported.
- [x] Backfill can archive fetched logs through `ARCHIVE_BACKFILLS=true`, allowing future raw replay without projection refetches.
- [x] Raw archive replay no longer depends on resolver-cache side files; binary ranges contain the logs needed for projection replay.
- [x] Raw archive replay is supported for projection rework without spending RPC or HyperSync credits.
- [x] Raw archive ranges are binary-only `.bin` files; legacy archive JSON range parsing and conversion helpers were removed after migration.
- [x] `.raw-archive-full/ranges` contains only binary range files after deleting migrated JSON range payloads.
- [x] Removed temporary archive conversion and resolver-cache rebuild CLI/Make/script helpers.
- [x] Raw replay prefetches the next archive range while applying the current range.
- [x] Raw replay keeps a replay-level current-state projection cache across range files.
- [x] Backfill and replay ranges are resume-only; RPC/HyperSync/raw replay start from source checkpoints.
- [x] Raw archive replay streams one range file at a time and wraps each range in a single Postgres transaction.
- [x] Raw archive replay drops secondary query indexes before bulk replay and recreates them afterward.
- [x] Raw archive replay attempts to restore dropped secondary indexes if replay fails.
- [x] Raw archive replay buffers `entity_changes` and mutable-entity snapshot writes, flushing one deduplicated change set per block.
- [x] Raw archive replay buffers append-only event rows and flushes them as chunked table-level batch inserts inside the replay transaction.
- [x] Raw archive replay logs per-range timing for block writes, decode, sort, projection, event flush, checkpoint writes, and total apply time.
- [x] Raw archive replay batch-upserts block metadata instead of writing one block row per SQL roundtrip.
- [x] Raw archive replay caches `Account`, `Domain`, and `Resolver` existence checks for repeated `create_if_missing` calls inside the range transaction.
- [x] Raw archive replay batch-flushes `entity_changes` and mutable-entity snapshots per entity type and block instead of writing one snapshot row per SQL roundtrip.
- [x] Raw archive replay logs `change_flush_ms` separately so snapshot/change flush time can be separated from event handler projection time.
- [x] Historical RPC, HyperSync, and raw archive fills share the same transactional buffered apply path.
- [x] Historical fills use a current-state projection cache for `Account`, `Domain`, `Registration`, `Resolver`, and `WrappedDomain`, flushing dirty rows in dependency order before block snapshots.
- [x] Historical fill logs include `current_flush_rows` and `current_flush_ms`.
- [x] Historical fills write mutable-entity block snapshots directly from the current-state projection cache instead of selecting the current tables at each block boundary.
- [x] Historical fills flush dirty current-state rows once at the end of each range instead of once per indexed block.
- [x] Historical fills preload touched current-state accounts, domains, registrations, resolvers, and wrapped domains before projection to reduce per-entity point queries.
- [x] Resolver event projection avoids ensuring the parent domain when the resolver already exists, reducing resolver-heavy range work.
- [x] Historical fills capture exact block snapshot rows in memory when entities are marked changed, preserving historical block reads without per-block flushes.
- [x] Historical fills batch `entity_changes` and all mutable-entity snapshots range-wide per table instead of issuing snapshot writes per `(entity, block)` group.
- [x] Release raw replay benchmark improved from roughly `3.0k logs/sec` in the preloader-only sample to roughly `8.6k logs/sec` overall and `9.3k logs/sec` on 50k+ log ranges after range-wide snapshot batching.
- [x] Historical fill batch sizes are capped below Postgres' bind-parameter limit for wide current-state and snapshot inserts.
- [x] Current-state domain cache flushes dirty domains in parent-first order to satisfy the self-referential `domains.parent_id` foreign key during range-level batch writes.
- [x] Backfill transport is selected explicitly with strict `BACKFILL_SOURCE=rpc|hypersync|raw`; there is no auto mode.
- [x] Live indexing transport is selected explicitly with strict `LIVE_INDEXING_SOURCE=rpc|wss`.
- [x] Serve-time startup backfill and live indexing use separate `ENABLE_BACKFILL` and `ENABLE_LIVE_INDEXING` toggles.
- [x] Backfill ranges can omit `from` and `to`; defaults resolve to the earliest ENS source block and latest available target.
- [x] Backfill archive writes are explicit through `ARCHIVE_BACKFILLS=true` and `RAW_ARCHIVE_DIR`.
- [x] `cargo-make` includes production start helpers for raw replay, HyperSync archive backfill, and WSS live indexing.
- [x] Raw archive replay was validated against a fresh dev Postgres database for a small ENS deployment range.
- [x] Raw archives include manifest entries with SHA-256 checksums and can be inspected for coverage gaps.
- [x] Archive status uses manifest coverage by default for large archives, with explicit checksum verification available through `--verify`.
- [x] Raw replay verifies archive checksums when a manifest is present.
- [x] A 1,000-block mainnet HyperSync backfill was run and archived for blocks `9380380..9381380`; archive status reports contiguous coverage and checksum verification.
- [x] Source checkpoints are stored per indexed source.
- [x] Live indexing loop runs with configurable confirmation depth.
- [x] Live indexing verifies parent hashes before applying new confirmed ranges.
- [x] Coarse reorg repair resets indexed state and rebuilds canonical state from source starts.

### Storage And Projection

- [x] SQL migrations create current-state entity tables, event tables, checkpoints, indexed blocks, and archive-supporting state.
- [x] Registry, registrar, wrapper, and resolver events are projected into event history tables.
- [x] Current-state tables are projected for `Domain`, `Registration`, `WrappedDomain`, `Resolver`, and `Account`.
- [x] ENS entity IDs and event IDs follow the documented official-subgraph-compatible shapes.
- [x] Namehash, labelhash, DNS name decoding, unknown label formatting, and batch event ID helpers have tests.
- [x] Current-state projection supports registry ownership, resolver links, TTL, registrar registrations/renewals/transfers, wrapper fuses/expiry, and resolver record changes.
- [x] Indexed block metadata is stored for `_meta` and live reorg checks.
- [x] Entity snapshot tables exist for `Account`, `Domain`, `Registration`, `WrappedDomain`, and `Resolver`.
- [x] Projection change markers write snapshot rows for mutable entities.
- [x] Wrapped-domain deletions write historical tombstones so past `block` reads do not resurrect deleted current state.

### GraphQL Schema Surface

- [x] Official subgraph schema diff currently reports zero missing/extra query fields.
- [x] Official subgraph schema diff currently reports zero missing/extra query args.
- [x] Official subgraph schema diff currently reports zero missing/extra input fields.
- [x] Official subgraph schema diff currently reports zero missing/extra enum values.
- [x] Singular and plural entity roots exist for `Domain`, `Registration`, `WrappedDomain`, `Account`, and `Resolver`.
- [x] Concrete event singular and plural roots exist for registry, registrar, wrapper, and resolver event tables.
- [x] Event interfaces exist: `domainEvent/domainEvents`, `registrationEvent/registrationEvents`, and `resolverEvent/resolverEvents`.
- [x] `_meta(block: Block_height)` exists and returns stored indexed block metadata.
- [x] Entity and event roots accept `block` and `subgraphError` compatibility arguments.
- [x] Current-state `block` reads work when the requested block is current or omitted.
- [x] Event roots honor non-current `block` arguments by clamping event rows to `blockNumber <= requestedBlock`.
- [x] Singular event roots return `null` when the requested block is before that event was emitted.
- [x] Mutable entity roots use snapshot-backed historical reads for `block.number`, `block.hash`, and `block.number_gte`.
- [x] Historical entity list roots reuse existing filters and order fields against latest snapshots at the requested block.

### GraphQL Relationships

- [x] Entity DTOs expose core relationships such as domain owner, parent, resolver, registrant, wrapped owner, resolver addr, registration domain, and wrapped-domain owner.
- [x] Event DTOs expose parent relationships such as `domain`, `registration`, and `resolver`.
- [x] Derived event relationships exist on `Domain.events`, `Registration.events`, and `Resolver.events`.
- [x] Account-derived collections exist for related domains, registrations, and wrapped domains.

### Filters And Ordering

- [x] Entity filters support `id`, `id_not`, `id_in`, and `id_not_in`.
- [x] Domain and wrapped-domain name filters support common exact, contains, nocase, prefix, and suffix predicates.
- [x] Domain and registration `labelName` filters support common string predicates.
- [x] Domain numeric/scalar filters include `createdAt_*`, `expiryDate_*`, `ttl_*`, `subdomainCount_*`, `labelhash`, `resolvedAddress`, and `isMigrated`.
- [x] Registration numeric/scalar filters include `registrationDate_*`, `expiryDate_*`, and `cost_*`.
- [x] Wrapped-domain filters include `expiryDate_*` and `fuses_*`.
- [x] Resolver filters include `address`, `address_in`, `addr`, `contentHash_*`, `texts_contains`, and `coinTypes_contains`.
- [x] `AccountFilter` supports `and` and `or` composition.
- [x] `DomainFilter`, `RegistrationFilter`, `WrappedDomainFilter`, and `ResolverFilter` support scalar-compatible `and` and `or` composition.
- [x] Concrete event filters support base event fields, event-specific fields, `and`, and `or`.
- [x] Event-interface filters support base event fields, projected event-specific fields, `and`, and `or`.
- [x] Shallow mutable-entity relationship filters are implemented for `Domain.parent_`, `Domain.owner_`, `Domain.resolver_`, `Domain.registrant_`, `Domain.wrappedOwner_`, `Registration.domain_`, `Registration.registrant_`, `WrappedDomain.domain_`, `WrappedDomain.owner_`, `Resolver.domain_`, and `Resolver.addr_`.
- [x] `DomainFilter` relationship predicates recurse through nested parent/account/resolver predicates, including relation-only `and`/`or` branches.
- [x] `RegistrationFilter`, `WrappedDomainFilter`, and `ResolverFilter` composition preserves nested relation-only predicates.
- [x] `Domain_filter.subdomains_` compiles to derived child-domain collection subqueries over `domains.parent_id`, including nested domain relation paths.
- [x] `Domain_filter.registration_` and `Domain_filter.wrappedDomain_` compile to derived one-to-one relation subqueries over `registrations` and `wrapped_domains`.
- [x] `Domain_filter.events_`, `Registration_filter.events_`, and `Resolver_filter.events_` compile to event-interface derived collection subqueries.
- [x] Event owner/addr scalar operators are mapped through API conversion and SQL predicates.
- [x] Event parent relation scalar operators are mapped for `domain_*`, `registration_*`, and `resolver_*` predicates.
- [x] Event-specific relation scalar operators are mapped for `parentDomain_*`, new-resolver `resolver_*`, `registrant_*`, and `newOwner_*` predicates.
- [x] Event relation predicates are wired for domain/account/resolver/registration-backed columns such as `domain_`, `parentDomain_`, `registration_`, `resolver_`, `owner_`, `registrant_`, `newOwner_`, and `addr_`.
- [x] Concrete event and event-interface filters support `_change_block: { number_gte }` through event `block_number` predicates.
- [x] Mutable entity filters support `_change_block: { number_gte }` through projection-maintained `entity_changes` records for `Account`, `Domain`, `Registration`, `WrappedDomain`, and `Resolver`.
- [x] Relationship order fields map to explicit static SQL expressions for entity queries, concrete event queries, and event-interface queries.
- [x] Query-builder SQL-shape tests cover scalar filters, relationship filters, ordering, and event filter composition.
- [x] Domain exact `name` and `labelName` filters use hash-backed equality predicates with exact rechecks, avoiding unsafe large-text btree indexes while keeping official GraphQL semantics.
- [x] Domain lookup indexes include fixed-size `labelhash` and MD5 expression indexes for high-volume ENSJS-style decoded-name queries.
- [x] Registrar/controller and NameWrapper projection persist valid label preimages in `label_preimages` so later registry subdomains can decode labels beyond the hardcoded core names.
- [x] Existing full-mainnet local DB was repaired from observed label preimages, changing common ENSJS decoded-name lookups such as `labelhash(vitalik)` from bracketed labels to decoded labels.
- [x] Local ENSRainbow streamed protobuf or TSV label dictionaries can be prepared offline without external API calls for future external healing tooling.
- [x] Removed unused storage import/repair helpers after the production CLI healing commands were deleted.
- [x] Removed runtime external ENSRainbow repair probing; any future label healing should stay local-file based and outside the production CLI until intentionally supported.
- [x] Verified local `labelhash(7261111)` response matches the official subgraph after label repair: `7261111.eth` in ~17ms local vs ~595ms official.
- [x] Audited ENSNode/Ponder subgraph indexes and added matching production query indexes: domain trigram/fuzzy name indexes, domain relation indexes, registration date indexes, wrapped-domain domain index, and compound derived-event parent indexes.
- [x] Added ENSJS-oriented address/sort indexes for `owner`, `registrant`, `wrappedOwner`, and `resolvedAddress` ordered by `expiryDate` and `createdAt`.
- [x] Added a storage fast path for the common ENSJS names-for-address filter shape, preserving GraphQL filter semantics while avoiding nested `id in (select ...)` plans on the full mainnet database.
- [x] Optimized no-op local label repair candidate scans with a bracketed-labelhash partial index and preimage join so completed heal passes return immediately.
- [x] Added repeatable benchmark fixtures and CLI runner for local compute, localhost HTTP, official The Graph, and ENSNode endpoint comparisons.
- [x] Added hash-backed exact/in query paths and indexes for `Domain.name`, `Domain.labelName`, and `Registration.labelName` to avoid long-text btree hazards and slow exact batch scans.
- [x] Benchmark runner records hosted endpoint `_meta` baseline timing and `baseline_adjusted_ms` so hosted provider comparisons are not dominated by internet round-trip time.
- [x] Benchmark runner supports `--local-compute false` for provider-only runs.
- [x] Benchmark runner records per-provider unsupported/errors per operation instead of aborting the whole comparison.
- [x] Benchmark runner keeps ENSNode schema quirks isolated to endpoint-only rewrites for the public alpha endpoint.
- [x] Production benchmark report is documented in `benchmarks/README.md` and this TODO file with the requested `operation | ensindexer-rs | ensnode | the graph indexer` table.
- [x] Added API DataLoader batching for `Domain.owner`, `Domain.resolvedAddress`, `Domain.registrant`, `Domain.wrappedOwner`, `Domain.resolver`, `Domain.registration`, and `Domain.wrappedDomain`.
- [x] Added storage batch lookup helpers for accounts, resolvers, registrations, and wrapped domains.

### API And Server

- [x] Axum server exposes GraphQL and health/status routes.
- [x] `/subgraph` POST aliases the GraphQL endpoint for official-subgraph and ENSJS-style clients.
- [x] `ensindexer start` always starts API, GraphQL, health checks, and Apollo Sandbox, with optional backfill/live indexing controlled by strict env toggles.
- [x] Public production CLI is reduced to `ensindexer start` and `ensindexer status`.
- [x] `ensindexer start` exposes strict env/arg controls for backfill, live indexing, archive writes, RPC/WSS, HyperSync, raw archive path, chain ID, bind address, batch size, confirmation depth, and polling interval.
- [x] Docker image builds and runs the `ensindexer` binary with `start` as the default command.
- [x] Split the bulk replay index registry into smaller current-state and event-index modules.
- [x] Removed dead archive-only ingestion and resolver-cache code after reducing the production CLI.

## Pending

### Full Subgraph Compatibility

- [ ] Audit nested relationship fields under historical entity results; root objects are snapshot-backed, but nested complex fields still need explicit historical context propagation checks.
- [ ] Add regression tests for `block.number`, `block.hash`, and `block.number_gte` across all entity and event root categories with seeded Postgres fixtures.
- [ ] Verify Graph Node edge semantics for `block.number_gte`; current implementation resolves to the latest indexed block at or after the requested minimum.
- [ ] Finish any deeper recursive trailing-underscore filter gaps found during official/local query audits.
- [ ] Audit and complete generated scalar operators for every remaining official scalar field, including less common `_not`, comparison, nocase, starts-with, and ends-with variants.
- [ ] Audit list-field edge cases against Graph Node behavior.
- [ ] Add compatibility tests that execute the same representative GraphQL queries against the local API and the official hosted ENS subgraph.
- [ ] Compare local and official JSON response shapes and values after real mainnet backfill ranges.
- [ ] Import a full local ENSRainbow dictionary, run larger repair batches across remaining unknown labelhashes, and measure how many `[hash]` names remain after repair.
- [ ] Add optional local dictionary lookup during live projection if production requires automatic `ens.nameByHash` parity during live indexing.

### Indexing Correctness And Production Hardening

- [x] Run a real 1,000 block mainnet fill with configured HyperSync credentials.
- [x] Consolidate all migrations into one final initial migration for external project setup.
- [x] Remove old CLI support modules for benchmark, schema diff, compare, and label-heal commands from the production binary crate.
- [x] Run strict workspace clippy and fix production cleanup warnings in storage and CLI.
- [x] Remove broad storage `clippy::collapsible_if` suppressions by simplifying cache-update control flow.
- [x] Remove unused label-preimage bulk import and repair methods from the production storage API.
- [ ] Continue dead-code cleanup across crates after the public CLI contraction.
- [x] Replace remaining README and docs references to removed `make` targets and old CLI commands.
- [ ] Validate the 1,000-block range projections against official subgraph responses for representative domains, resolvers, registrations, and events.
- [ ] Verify stored rows and GraphQL responses from that range against the official subgraph.
- [ ] Add differential validation reports for domains, registrations, resolvers, wrapped domains, and events.
- [ ] Replace coarse reorg reset with efficient common-ancestor rollback.
- [ ] Extend snapshot use to efficient common-ancestor rollback or add reversible change payloads for rollback without full replay.
- [ ] Add stronger live indexing observability: structured metrics, lag reporting, source checkpoint summaries, and failure counters.
- [ ] Add retry/backoff policy hardening for RPC, HyperSync, database, and archive IO failures.
- [x] Add first database indexes tuned from full-mainnet query timings: exact `Domain.name`, exact `Domain.labelName`, and exact `Domain.labelhash`.
- [x] Add ENSNode/Ponder-inspired indexes from `.repos/ensnode/packages/ensdb-sdk/src/ensindexer-abstract/subgraph.schema.ts`.
- [x] Add full-mainnet query optimization for ENSJS-style address lookups; local warm latency is now roughly 18-22ms for `domains(where: { and: [{ or: owner/registrant/wrappedOwner/resolvedAddress }, { or: expiryDate_gt/null }] })`.
- [x] Add benchmark harness and initial full-mainnet smoke report for common ENSJS/ENSNode query classes.
- [ ] Continue adding database indexes from real query plans after representative official/ENSJS query audits.
- [ ] Profile historical fills with a real flamegraph or `tokio-console` style instrumentation on dense 250k+ log ranges and record top CPU/SQL paths.
- [ ] Audit range-wide buffered historical snapshots against seeded fixtures for block-boundary correctness across repeated mutations in the same range.
- [ ] Benchmark the upcoming 250k+ and 500k+ log ranges after range-wide snapshot batching; current measured throughput is still short of the 3-hour target for 150M raw logs.
- [ ] Evaluate Postgres `COPY` or staging-table merge paths for event tables, current-state tables, entity changes, and snapshots to target 14k+ sustained logs/sec.
- [ ] Evaluate partitioned/unlogged staging tables during historical replay, followed by indexed merge into durable tables.
- [ ] Add per-table flush timing inside current-state, event, and snapshot flushes to identify which tables dominate dense ranges.
- [ ] Audit current-state cache behavior against official subgraph fixtures for representative registry, registrar, wrapper, and resolver ranges.

### Performance And API Quality

- [ ] Expand DataLoader or batched repository loading to resolver-domain, registration-domain, wrapped-domain owner, and event-parent hydration paths.
- [ ] Add integration tests with seeded Postgres fixtures for common GraphQL query shapes.
- [ ] Add pagination stress tests for large event-interface unions.
- [ ] Add query-plan checks for expensive relationship filters and order fields.
- [ ] Add query-plan checks for exact domain-name, labelhash decoded-name, and parent subdomain traversal queries.
- [ ] Add query-plan checks for ENSJS names-for-address queries to prevent regressions in the storage fast path.
- [ ] Design a search-specific optimization for broad substring workloads such as `name_contains_nocase: "art"` and `Domain.subdomains(where: { labelName_contains_nocase: "art" })`; current GIN plans still fetch/sort 40k+ candidate rows and sit above the sub-100ms target.
- [ ] Investigate a search-specific plan for `04-subnames-search`; local release median is roughly 161ms, ENSNode is roughly 1.7s, and hosted The Graph returned a 504 timeout in the production benchmark.
- [ ] Investigate why `11-text-search` is slower locally than hosted The Graph in the release benchmark; likely candidate volume/sort behavior after trigram match.
- [ ] Add benchmark report generation that converts `target/benchmark-production.json` directly into the Markdown comparison table to avoid manual transcription.
- [ ] Add optional per-query timeout/retry settings for hosted benchmarks so transient provider failures are clearly separated from schema-incompatible unsupported operations.
- [ ] Add production Docker image build and serve verification.

### Documentation Maintenance

- [ ] Update this file after every compatibility or indexing slice.
- [ ] Keep `docs/README.md`, `docs/implementation-roadmap.md`, and `docs/schema-and-graphql-shape.md` in sync with this checklist.
- [ ] Record every verified mainnet range fill and official-subgraph comparison result.

## Verified Mainnet Ranges

### `9380380..9381380`

- Source: `BACKFILL_SOURCE=hypersync` with `ARCHIVE_BACKFILLS=true`.
- Archive: `.raw-archive`, manifest chain id `1`, 3 range files, 177,696 bytes, 188 raw logs, no coverage gaps.
- Database checkpoint: all configured sources at block `9381380` with hash `0x5dde9defc28957eacf0489a827dce4e030f6ab383c17ea3e959e00fed6a65f95`.
- Projected rows after the run: 78 domains, 45 accounts, 44 resolvers, 1 registration, 99 indexed blocks, 270 entity changes, 72 `new_owner_events`, 37 `new_resolver_events`, 26 `addr_changed_events`, 22 `multicoin_addr_changed_events`, 12 `transfer_events`, and 1 `name_registered_event`.
- Remaining validation: compare representative local GraphQL responses for this range against the official hosted ENS subgraph.
