# ENS Indexer TODOs

Running implementation and compatibility checklist for the custom Rust ENS indexer. Keep this file updated after each meaningful implementation slice.

Last full verification: `cargo run -p cli -- schema-diff --output target/official-subgraph-schema.json && make check` passed after the raw replay event-buffering and profiling slice. Archive backfill and checksum-backed raw replay were validated locally for blocks `9380380..9380390`. A 1,000-block HyperSync archive backfill was run for `9380380..9381380`; archive coverage reports no gaps.

## Completed

### Workspace And Operations

- [x] Cargo workspace uses unprefixed crate names: `types`, `contracts`, `config`, `storage`, `projection`, `ingest`, `api`, `server`, and `cli`.
- [x] `.env.example` exists and runtime configuration is read from `.env`.
- [x] Docker Compose includes Postgres 17.
- [x] Makefile provides common commands for setup, serving, schema checks, tests, and linting.
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
- [x] Archive-only fetching is supported through `cli archive`, allowing raw range files to be stored without projection writes.
- [x] Archive-only resume persists discovered resolver addresses in `resolvers.json` and can rebuild that cache from existing archive files.
- [x] Raw archive replay is supported for projection rework without spending RPC or HyperSync credits.
- [x] Raw archive replay streams one range file at a time and wraps each range in a single Postgres transaction.
- [x] Raw archive replay drops secondary query indexes before bulk replay and recreates them afterward.
- [x] Raw archive replay attempts to restore dropped secondary indexes if replay fails.
- [x] Raw archive replay buffers `entity_changes` and mutable-entity snapshot writes, flushing one deduplicated change set per block.
- [x] Raw archive replay buffers append-only event rows and flushes them as chunked table-level batch inserts inside the replay transaction.
- [x] Raw archive replay logs per-range timing for block writes, decode, sort, projection, event flush, checkpoint writes, and total apply time.
- [x] Backfill transport is selected explicitly with strict `BACKFILL_SOURCE=rpc|hypersync|raw`; there is no auto mode.
- [x] Live indexing transport is selected explicitly with strict `INDEXING_SOURCE=http_rpc|wss`.
- [x] Serve-time startup backfill and live indexing use separate `ENABLE_BACKFILL` and `ENABLE_LIVE_INDEXING` toggles.
- [x] Backfill ranges can omit `from` and `to`; defaults resolve to the earliest ENS source block and latest available target.
- [x] Backfill archive writes are explicit through `ARCHIVE_BACKFILLS=true` and `RAW_ARCHIVE_DIR`.
- [x] Makefile includes archive and raw replay helpers for repeatable projection testing.
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

### API And Server

- [x] Axum server exposes GraphQL and health/status routes.
- [x] `serve` always starts API, GraphQL, health checks, and Apollo Sandbox, with optional backfill/live indexing controlled by strict env toggles.
- [x] CLI includes `migrate`, `backfill`, `replay`, `index`, `serve`, `status`, `reset --yes`, `compare`, `schema-local`, and `schema-diff`.

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

### Indexing Correctness And Production Hardening

- [x] Run a real 1,000 block mainnet fill with configured HyperSync credentials.
- [ ] Validate the 1,000-block range projections against official subgraph responses for representative domains, resolvers, registrations, and events.
- [ ] Verify stored rows and GraphQL responses from that range against the official subgraph.
- [ ] Add differential validation reports for domains, registrations, resolvers, wrapped domains, and events.
- [ ] Replace coarse reorg reset with efficient common-ancestor rollback.
- [ ] Extend snapshot use to efficient common-ancestor rollback or add reversible change payloads for rollback without full replay.
- [ ] Add stronger live indexing observability: structured metrics, lag reporting, source checkpoint summaries, and failure counters.
- [ ] Add retry/backoff policy hardening for RPC, HyperSync, database, and archive IO failures.
- [ ] Add database indexes tuned from real query plans after representative backfills.
- [ ] Add deeper raw replay batching for high-density ranges: batched current-state upserts and reduced per-event read amplification.
- [ ] Profile raw replay after event buffering with dense resolver ranges and record logs/sec, SQL time, and top write queries.

### Performance And API Quality

- [ ] Add DataLoader or batched repository loading for high-volume nested GraphQL relationship fields.
- [ ] Add integration tests with seeded Postgres fixtures for common GraphQL query shapes.
- [ ] Add pagination stress tests for large event-interface unions.
- [ ] Add query-plan checks for expensive relationship filters and order fields.
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
