# Rust Implementation Roadmap

This document is the implementation plan for a from-scratch Rust ENS indexer that keeps the current ENS subgraph schema and GraphQL query shape. The goal is not only to decode logs. The indexer must reproduce the official subgraph's projections, entity IDs, derived relationships, event history, and Graph Node-style GraphQL API.

## Target Architecture

Use a Cargo workspace with small crates and clear ownership boundaries:

```text
ensindexer/
  Cargo.toml
  crates/
    types/
    contracts/
    config/
    storage/
    projection/
    ingest/
    api/
    server/
    cli/
  migrations/
  docs/
```

Recommended crate responsibilities:

| Crate        | Responsibility                                                                               | Main dependencies                                   |
| ------------ | -------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `types`      | shared IDs, chain constants, scalar wrappers, DTO primitives, error types                    | `alloy-primitives`, `serde`, `thiserror`            |
| `contracts`  | ABI bindings, event structs, topic constants, decoded event enum                             | `alloy`, `serde`                                    |
| `config`     | `.env` loading and typed runtime configuration shared by CLI, server, and ingest jobs        | `dotenvy`, `serde`, `url`, `thiserror`              |
| `storage`    | SQLx pool, migrations, repositories, transactional writes, query builders                    | `sqlx`, `serde_json`, `thiserror`, `anyhow`         |
| `projection` | deterministic event handlers that convert decoded ENS events into entity/event table updates | `types`, `storage`, `alloy-primitives`              |
| `ingest`     | HyperSync/RPC historical fetching, live tailing, checkpointing, reorg detection, event dispatch | `alloy`, `hypersync-client`, `tokio`, `contracts`, `projection` |
| `api`        | `async-graphql` objects, interfaces, filters, order enums, pagination, resolvers             | `async-graphql`, `storage`, `serde`                 |
| `server`     | Axum HTTP app, GraphQL endpoint, health/readiness, CORS, compression, tracing                | `axum`, `tower`, `tower-http`, `async-graphql-axum` |
| `cli`        | production command shell for `start` and `status`, with strict runtime validation             | `clap`, `tokio`, workspace crates                   |

Keep `projection` free of RPC and HTTP concerns. Projection tests should be able to create a transaction, apply synthetic decoded events, and inspect storage.

Current code should avoid large single-file crates. Each crate keeps a small `lib.rs` or `main.rs` as the public entrypoint and places implementation in modules:

```text
crates/types/src/{lib.rs,core.rs}
crates/contracts/src/{lib.rs,abi.rs,model.rs,events.rs}
crates/config/src/{lib.rs,env.rs}
crates/storage/src/{lib.rs,error.rs,models.rs,inserts.rs,filters.rs,query.rs,store.rs,repositories.rs}
crates/storage/src/repositories/{accounts.rs,domains.rs,registrations.rs,resolvers.rs,wrapped_domains.rs,events.rs,blocks.rs,checkpoints.rs,utils.rs}
crates/projection/src/{lib.rs,error.rs,support.rs,handlers.rs}
crates/projection/src/handlers/{dispatcher.rs,registry.rs,registrar.rs,wrapper.rs,resolver.rs}
crates/ingest/src/{lib.rs,sources.rs,rpc.rs,hypersync.rs,decode.rs,service.rs}
crates/api/src/{lib.rs,filters.rs,objects.rs,pagination.rs,schema.rs}
crates/server/src/{lib.rs,http.rs}
crates/cli/src/{main.rs,app.rs}
```

Keep repository and projection implementation modules split by domain from the start. `storage::repositories` should be a re-exporting facade over entity-specific modules, and `projection::handlers` should be a re-exporting facade over event-family handlers. That keeps the external API stable while preventing a single projection or SQL file from becoming the coordination point for every future ENS feature.

## Implementation Principles

1. Treat the official subgraph as the compatibility contract.
2. Reproduce entity IDs before optimizing anything else.
3. Apply logs in canonical chain order: `(block_number, transaction_index, log_index)`.
4. Store event history separately from mutable entity state.
5. Use GraphQL DTOs rather than exposing SQL rows directly.
6. Generate SQL from typed filter/order enums, never from user-provided strings.
7. Build the ingest and projection layers before polishing the API, because the API can only be correct if projections are correct.

## Step 1: Workspace Foundation

Create the workspace crates and centralize shared dependency versions in the root `Cargo.toml`.

Root workspace settings should include:

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
edition = "2024"
license = "MIT"

[workspace.dependencies]
anyhow = "1"
thiserror = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
alloy = "2.0.5"
alloy-network-primitives = "2.0.5"
alloy-primitives = { version = "1.6.0", features = ["serde"] }
alloy-sol-types = "1.6.0"
sqlx = { version = "0.9.0", features = ["runtime-tokio", "tls-rustls", "postgres", "macros", "migrate", "json", "bigdecimal"] }
async-graphql = { version = "8.0.0-rc.5", features = ["dataloader"] }
async-graphql-axum = "8.0.0-rc.5"
axum = "0.8"
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace", "compression-full", "timeout"] }
```

Add CI or local commands early:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Step 2: Shared Types and Constants

Implement `types` first.

Core newtypes:

```rust
pub struct DomainId(pub B256);       // ENS namehash
pub struct LabelHash(pub B256);      // labelhash / base registrar token ID
pub struct AccountId(pub Address);   // lower-case address ID
pub struct ResolverId {
    pub address: Address,
    pub node: B256,
}
```

Important helpers:

| Helper                            | Required behavior                                                       |
| --------------------------------- | ----------------------------------------------------------------------- |
| `hex_id`                          | lowercase `0x` encoding for IDs and bytes                               |
| `resolver_id(address, node)`      | `{address}-{node}` with official subgraph formatting                    |
| `event_id(ctx)`                   | `{block_number}-{log_index}`                                            |
| `batch_event_id(ctx, item_index)` | `{block_number}-{log_index}-{item_index}`                               |
| `labelhash_to_token_id`           | convert between base registrar `uint256` token ID and 32-byte labelhash |
| `namehash`                        | deterministic ENS namehash for known names                              |
| `dns_name_decode`                 | decode wrapper DNS-encoded names, including invalid-name handling       |
| `validate_label`                  | match official invalid label behavior                                   |

Store chain constants in one place:

| Constant               | Purpose                                           |
| ---------------------- | ------------------------------------------------- |
| ENS registry addresses | old and current registry sources                  |
| Base registrar address | `.eth` ERC-721 registration state                 |
| Name wrapper address   | ERC-1155 wrapped names                            |
| Controller addresses   | known preimage/cost enrichment                    |
| Start blocks           | source-specific historical backfill start         |
| `.eth` node            | parent node for second-level `.eth` registrations |
| grace period           | `7776000` seconds for `.eth` domain expiry        |

## Step 3: ABI Bindings and Decoding

Implement `contracts`.

Use `alloy::sol!` for lightweight event snippets and keep the generated event types in `contracts`. The Alloy 2.0.5 docs document both inline Solidity snippets and JSON ABI generation; JSON ABI files are useful when importing full contract artifacts, while this indexer can keep event-only snippets small and explicit.

Use bindings for:

- registry events: `Transfer`, `NewOwner`, `NewResolver`, `NewTTL`;
- base registrar events: `NameRegistered`, `NameRenewed`, ERC-721 `Transfer`;
- controller registration/renewal events for known labels and cost;
- name wrapper events: `NameWrapped`, `NameUnwrapped`, `FusesSet`, `ExpiryExtended`, ERC-1155 `TransferSingle`, `TransferBatch`;
- resolver events: `AddrChanged`, multicoin `AddressChanged`, `NameChanged`, ABI, pubkey, text, contenthash, interface, authorisation, version changes.

Create one decoded event enum:

```rust
pub enum EnsEvent {
    RegistryTransfer { node: B256, owner: Address, source: RegistrySource },
    RegistryNewOwner { node: B256, label: B256, owner: Address, source: RegistrySource },
    RegistryNewResolver { node: B256, resolver: Address, source: RegistrySource },
    RegistryNewTtl { node: B256, ttl: U256, source: RegistrySource },
    BaseNameRegistered { labelhash: B256, owner: Address, expires: U256 },
    BaseNameRenewed { labelhash: B256, expires: U256 },
    BaseTransfer { from: Address, to: Address, labelhash: B256 },
    ControllerNameRegistered { label: String, labelhash: B256, cost: U256 },
    ControllerNameRenewed { label: String, labelhash: B256, cost: U256 },
    NameWrapped { node: B256, dns_name: Vec<u8>, owner: Address, fuses: u32, expiry: U256 },
    NameUnwrapped { node: B256, owner: Address },
    FusesSet { node: B256, fuses: u32 },
    ExpiryExtended { node: B256, expiry: U256 },
    WrappedTransferSingle { to: Address, token_id: U256 },
    WrappedTransferBatch { to: Address, token_ids: Vec<U256> },
    ResolverAddrChanged { node: B256, addr: Address },
    ResolverMulticoinAddrChanged { node: B256, coin_type: U256, addr: Vec<u8> },
    ResolverTextChanged { node: B256, key: String, value: Option<String> },
    ResolverVersionChanged { node: B256, version: U256 },
    // remaining resolver events
}
```

Every decoded event travels with log metadata:

```rust
pub struct LogContext {
    pub block_number: i64,
    pub block_timestamp: i64,
    pub block_hash: B256,
    pub transaction_hash: B256,
    pub transaction_index: i64,
    pub log_index: i64,
    pub contract_address: Address,
}
```

## Step 4: Database Schema

Use Postgres with `sqlx` migrations. Start with explicit tables that mirror GraphQL entities.

Mutable tables:

| Table             | Purpose                                                  |
| ----------------- | -------------------------------------------------------- |
| `accounts`        | address IDs referenced by ownership and resolver records |
| `domains`         | current domain graph state                               |
| `registrations`   | `.eth` registration state                                |
| `wrapped_domains` | name wrapper state                                       |
| `resolvers`       | resolver contract/node state                             |

Event tables:

| Family              | Tables                                                                                                                                                                                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| domain events       | `transfer_events`, `new_owner_events`, `new_resolver_events`, `new_ttl_events`, `wrapped_transfer_events`, `name_wrapped_events`, `name_unwrapped_events`, `fuses_set_events`, `expiry_extended_events`                                                                 |
| registration events | `name_registered_events`, `name_renewed_events`, `name_transferred_events`                                                                                                                                                                                              |
| resolver events     | `addr_changed_events`, `multicoin_addr_changed_events`, `name_changed_events`, `abi_changed_events`, `pubkey_changed_events`, `text_changed_events`, `contenthash_changed_events`, `interface_changed_events`, `authorisation_changed_events`, `version_changed_events` |

Indexer tables:

| Table                | Purpose                                                                    |
| -------------------- | -------------------------------------------------------------------------- |
| `blocks`             | processed block number/hash/timestamp                                      |
| `source_checkpoints` | last indexed block per source group                                        |
| `indexed_logs`       | optional idempotency table keyed by block hash + tx hash + log index       |
| `entity_changes`     | optional support for `_change_block` filters and future historical queries |

Prefer normalized hex `text` IDs initially. Use `numeric` for GraphQL `BigInt` fields, or decimal `text` if you want simpler SQLx binding. Add indexes before full backfill:

```sql
create index domains_parent_idx on domains(parent_id);
create index domains_owner_idx on domains(owner_id);
create index domains_resolver_idx on domains(resolver_id);
create index domains_name_lower_trgm_idx on domains using gin (lower(name) gin_trgm_ops);
create index domains_label_name_lower_trgm_idx on domains using gin (lower(label_name) gin_trgm_ops);
create index registrations_domain_idx on registrations(domain_id);
create index registrations_registrant_idx on registrations(registrant_id);
create index wrapped_domains_owner_idx on wrapped_domains(owner_id);
create index resolvers_domain_idx on resolvers(domain_id);
create index resolvers_address_idx on resolvers(address);
```

Each event table needs indexes for:

- `id`;
- parent FK (`domain_id`, `registration_id`, or `resolver_id`);
- `block_number`;
- `transaction_id`.

## Step 5: Storage Repositories

Implement `storage` around transactions and typed repository methods.

Do not let projection handlers hand-write SQL. Use methods like:

```rust
accounts.create_if_missing(tx, account_id).await?;
domains.create_if_missing(tx, domain_id, created_at, owner_id).await?;
domains.set_owner(tx, domain_id, owner_id).await?;
domains.set_resolver(tx, domain_id, resolver_id).await?;
registrations.upsert_registration(tx, registration).await?;
events.insert_name_wrapped(tx, event).await?;
```

Repository groups:

| Repository           | Responsibility                                                                    |
| -------------------- | --------------------------------------------------------------------------------- |
| `AccountsRepo`       | upsert and lookup accounts                                                        |
| `DomainsRepo`        | create/update domains, name construction fields, ownership, resolver, TTL, expiry |
| `RegistrationsRepo`  | `.eth` registration rows and controller enrichment                                |
| `WrappedDomainsRepo` | wrapper rows, fuses, expiry, owner                                                |
| `ResolversRepo`      | resolver rows and mutable resolver records                                        |
| `EventsRepo`         | insert concrete event rows and query event interfaces                             |
| `QueryRepo`          | GraphQL list queries with filters, ordering, pagination                           |

All projection writes for one log should run in one transaction. For batch ranges, choose between one transaction per block or one transaction per bounded chunk. One transaction per block is easier to roll back and debug.

## Step 6: Projection Handlers

Implement `projection` by porting behavior from the official mapping files, not by guessing from ABI names.

Suggested handler modules:

```text
projection/src/
  registry.rs
  registrar.rs
  wrapper.rs
  resolver.rs
  names.rs
  dispatcher.rs
```

Projection behavior to preserve:

| Area             | Critical behavior                                                                                                                   |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| registry         | old/new registry migration guard, owner updates, subdomain creation, parent `subdomainCount`, resolver ID construction, TTL updates |
| base registrar   | `.eth` registration creation, renewal, transfer, registrant assignment, domain expiry as registrar expiry plus grace period         |
| controllers      | enrich registration cost and label preimage only when label is valid/known                                                          |
| name wrapper     | wrapped owner, fuses, expiry, DNS name decode, wrapped transfer placeholders, subname unwrap expiry clearing                        |
| resolver         | resolver row by `{address}-{node}`, ETH addr syncing to `Domain.resolvedAddress`, multicoin/text key arrays, contenthash updates    |
| resolver version | reset mutable resolver records when version changes                                                                                 |
| event entities   | insert concrete event rows with official IDs and block/transaction metadata                                                         |

Recommended implementation shape:

```rust
pub async fn apply_indexed_event(
    tx: &mut PgTransaction<'_>,
    event: IndexedEvent,
) -> Result<(), ProjectionError> {
    match event.event {
        EnsEvent::RegistryNewOwner { .. } => registry::handle_new_owner(tx, event).await,
        EnsEvent::BaseNameRegistered { .. } => registrar::handle_name_registered(tx, event).await,
        EnsEvent::NameWrapped { .. } => wrapper::handle_name_wrapped(tx, event).await,
        EnsEvent::ResolverTextChanged { .. } => resolver::handle_text_changed(tx, event).await,
        _ => todo!(),
    }
}
```

The handler input is a decoded event plus `LogContext`; the output is only database writes. This makes unit and integration tests straightforward.

## Step 7: Historical Ingestion

Implement `ingest` as a deterministic backfill engine.

Source groups:

| Source group               | Log query strategy                                                                                                             |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| old/current registry       | fixed contract address plus registry event topics                                                                              |
| base registrar             | fixed contract address plus registrar/ERC-721 topics                                                                           |
| controllers                | fixed contract addresses plus controller event topics                                                                          |
| name wrapper               | fixed contract address plus wrapper/ERC-1155 topics                                                                            |
| resolver dynamic addresses | resolver event topic filters plus resolver contract addresses discovered from `NewResolver` events and persisted resolver rows |

Backfill algorithm:

```text
for block_range in planned_ranges:
  fetch all source logs for the range
  fetch block timestamps for touched blocks
  optionally write raw archive range binary
  decode logs
  sort by block_number, transaction_index, log_index
  group by block
  for each block:
    begin transaction
    assert parent/canonical assumptions if applicable
    apply every decoded event in order
    write block row and source checkpoints
    commit
```

Use separate checkpoints for source groups only if you can still merge logs into canonical order before projection. The safest implementation fetches all needed logs for a range, merges them, and then applies them.

Resolver wildcard ingestion is the largest stream. Start with small block ranges and topic-specific checkpoints. For performance, add adaptive range sizing:

- shrink range when returned log count is near provider limit;
- grow range when log count is low;
- cap retries with exponential backoff.

Raw archive/replay:

- `RAW_ARCHIVE_DIR` writes fetched logs and block metadata to filesystem binary files keyed by inclusive range.
- Archive files store chain ID, range bounds, raw logs with source tags, block metadata, and checkpoint source names.
- `cli replay` reads contiguous archive files from database checkpoints through the last archived range and applies the same decode/projection/checkpoint path as fetched backfill.
- Projection development should prefer archive replay after the first fetch so compatibility fixes do not repeatedly spend RPC or HyperSync credits.

## Step 8: Live Indexing and Reorg Handling

Graph Node handles reorgs for the official subgraph. A custom indexer must implement this explicitly.

Minimum production strategy:

1. Track processed block number, hash, parent hash, and timestamp.
2. Index live blocks behind a confirmation depth.
3. Before applying block `N`, verify its parent hash matches stored block `N - 1`.
4. On mismatch, roll back to a common ancestor.
5. Re-apply canonical blocks.

Rollback options:

| Strategy                                                  | Tradeoff                                                           |
| --------------------------------------------------------- | ------------------------------------------------------------------ |
| delete events and rebuild mutable state from a checkpoint | simplest correctness model, slower after deep reorgs               |
| store entity change logs and reverse mutations            | faster, more complex                                               |
| finality lag only                                         | acceptable for low-risk MVP, not enough for exact live correctness |

For the first production version, use a finality lag plus rebuild-from-checkpoint for reorg repair.

Current implementation state:

- `cli index` runs a continuous live loop.
- The loop computes `safe_head = latest - INDEXER_CONFIRMATION_DEPTH`.
- The next block is derived from source checkpoints, falling back to each source start block when no checkpoint exists.
- The live loop indexes at most `BACKFILL_BATCH_BLOCKS` confirmed blocks per pass and sleeps for `LIVE_POLL_SECONDS` when no confirmed work is available.
- Before applying a live range, block `N` is fetched and its parent hash is compared with locally stored block `N - 1` when that previous block exists.
- Reorg repair currently uses a coarse full indexed-state reset and canonical rebuild from source start blocks. This is correct with the current schema but expensive.
- A more efficient common-ancestor rollback still requires entity change logs or snapshots.

## Step 9: GraphQL API

Implement `api` after storage repositories can return correct rows.

GraphQL layers:

```text
axum /graphql route
  async-graphql Schema
    QueryRoot
      entity resolvers
        QueryRepo
          SQL filter/order/pagination builder
```

Objects must use official field names:

| SQL field             | GraphQL field     |
| --------------------- | ----------------- |
| `label_name`          | `labelName`       |
| `subdomain_count`     | `subdomainCount`  |
| `resolved_address_id` | `resolvedAddress` |
| `wrapped_owner_id`    | `wrappedOwner`    |
| `transaction_id`      | `transactionID`   |
| `block_number`        | `blockNumber`     |
| `expiry_date`         | `expiryDate`      |

Implement root queries in this order:

1. singular entity lookups: `domain`, `registration`, `wrappedDomain`, `account`, `resolver`;
2. plural entity queries with `skip`, `first`, default ordering, and simple `where`;
3. concrete event singular/plural queries;
4. derived relationship fields;
5. event interfaces: `domainEvents`, `registrationEvents`, `resolverEvents`;
6. nested filters and relationship ordering;
7. `_meta`;
8. `block` and `_change_block` compatibility.

Use `DataLoader` or batched repository methods for relationship fields. This matters because GraphQL clients often request nested objects for every row in a list.

Current implementation state:

- Entity singular/plural queries exist for `Domain`, `Registration`, `WrappedDomain`, `Account`, and `Resolver`.
- Concrete event singular/plural queries exist for registry, registrar, wrapper, and resolver event tables.
- Concrete event queries support `id`, `id_not`, `id_in`, `id_not_in`, parent entity ID, `blockNumber`, `blockNumber_gt`, `blockNumber_lt`, `blockNumber_gte`, `blockNumber_lte`, `transactionID`, `transactionID_not`, `transactionID_in`, `transactionID_not_in`, event-specific stored fields, `first`, `skip`, `orderBy`, and `orderDirection`.
- Event DTOs expose parent relationships such as `domain`, `registration`, and `resolver` through GraphQL resolvers.
- Event interfaces (`domainEvent/domainEvents`, `registrationEvent/registrationEvents`, `resolverEvent/resolverEvents`) exist and paginate after a SQL `union all` reference query.
- `Domain.events`, `Registration.events`, and `Resolver.events` expose derived interface relationships.
- `_meta(block: Block_height)` exists and returns the stored indexed block plus `deployment` and `hasIndexingErrors`.
- Mutable-entity, concrete-event, and event-interface roots accept `block: Block_height` and `subgraphError: _SubgraphErrorPolicy_`. Event roots clamp by `blockNumber`; mutable-entity roots read from projection-maintained snapshot tables for historical `block.number`, `block.hash`, and `block.number_gte` requests.
- Entity filters support exact IDs, `id_not`, `id_in`, `id_not_in`, common string predicates for known name fields, labelhash/contenthash predicates, `subdomainCount_*`, `isMigrated`, resolver `texts_contains` and `coinTypes_contains`, numeric comparison filters for core `BigInt`/`Int` fields, and shallow trailing-underscore relationship filters on direct mutable-entity relationships.
- `AccountFilter` supports `and` and `or` composition, including when used in account-backed relationship filters such as `owner_`, `registrant_`, `wrappedOwner_`, `resolvedAddress_`, and `addr_`.
- Scalar-compatible `DomainFilter`, `RegistrationFilter`, `WrappedDomainFilter`, `ResolverFilter`, concrete event filters, and event-interface filters support `and` and `or` composition.
- Relationship order fields compile to explicit SQL expressions for mutable entities, concrete events, and event-interface reference queries. Concrete event ordering uses table-specific parent columns such as `domain_id`, `registration_id`, and `resolver_id`; interface references use the union-level `parent_id`.
- Event-interface reference unions project typed nullable columns for their concrete event-family fields, so interface filters can apply predicates such as `fuses_*`, `expiryDate_*`, `key_*`, `coinType_*`, and `isAuthorized`.
- Event account-like scalar predicates for `owner_*` and `addr_*` are mapped through API conversion and SQL predicates on concrete event tables and event-interface unions.
- Event parent relation scalar predicates for `domain_*`, `registration_*`, and `resolver_*` are mapped through API conversion and SQL predicates on concrete event tables and event-interface unions.
- Event-specific relation scalar predicates for `parentDomain_*`, new-resolver `resolver_*`, `registrant_*`, and `newOwner_*` are mapped through API conversion and SQL predicates on concrete event tables and event-interface unions.
- Event relation predicates are mapped through API conversion and SQL subqueries for domain/account/resolver/registration-backed columns such as `domain_`, `parentDomain_`, `registration_`, `resolver_`, `owner_`, `registrant_`, `newOwner_`, and `addr_`.
- Concrete event and event-interface filters support `_change_block: { number_gte }` by compiling it to `block_number >= number_gte`.
- Mutable entity filters support `_change_block: { number_gte }` through the `entity_changes` table populated by projection when `Account`, `Domain`, `Registration`, `WrappedDomain`, or `Resolver` rows are inserted or updated.
- `Domain_filter.subdomains_` is implemented as a derived child-domain collection subquery against `domains.parent_id`, including nested domain relation paths.
- `Domain_filter.registration_` and `Domain_filter.wrappedDomain_` are implemented as derived one-to-one relation subqueries against `registrations.domain_id` and `wrapped_domains.domain_id`.
- `Domain_filter.events_`, `Registration_filter.events_`, and `Resolver_filter.events_` are implemented as derived collection subqueries against their event-interface unions.
- `DomainFilter` relationship predicates recurse through nested `parent_`, account-backed relations, and resolver scalar predicates, including when the only condition is inside a nested `and`/`or` branch.
- `RegistrationFilter` and `WrappedDomainFilter` `and`/`or` composition applies nested `domain_`, `registrant_`, and `owner_` relation predicates instead of dropping relation-only branches.
- `ResolverFilter` `and`/`or` composition applies nested `domain_` and `addr_` relation predicates instead of dropping relation-only branches.
- Storage query helpers use delimiter-safe `sqlx::QueryBuilder` fragments and have SQL-shape unit tests for scalar and relationship predicates.
- Historical block snapshots are implemented for mutable root entity reads. Reversible rollback payloads, nested historical context audits, and deeper generated-filter audits are still compatibility-expansion work.

## Step 10: Filters, Ordering, and Joins

The hosted subgraph exposes Graph Node generated filters. Implement them in tiers.

Tier 1 filters:

| Object          | Filters                                                                                                                                                                                        |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| all objects     | `id`, `id_not`, `id_in`, `id_not_in`                                                                                                                                                           |
| `Domain`        | `name`, `name_contains`, `name_contains_nocase`, `name_starts_with`, `name_ends_with`, `labelName`, `parent`, `owner`, `resolver`, `registrant`, `wrappedOwner`, `expiryDate_*`, `createdAt_*` |
| `Registration`  | `domain`, `registrant`, `labelName`, `registrationDate_*`, `expiryDate_*`, `cost_*`                                                                                                            |
| `WrappedDomain` | `domain`, `owner`, `name`, `fuses_*`, `expiryDate_*`                                                                                                                                           |
| `Resolver`      | `domain`, `address`, `addr`, `contentHash`, `texts_contains`, `coinTypes_contains`                                                                                                             |
| events          | parent FK, `blockNumber_*`, `transactionID`, event-specific fields                                                                                                                             |

Current implementation covers a subset of Tier 1:

- `id`, `id_not`, `id_in`, and `id_not_in` on entity queries.
- `name`, `name_contains`, `name_contains_nocase`, `name_starts_with`, and `name_ends_with` on `Domain` and `WrappedDomain`.
- `labelName`, `labelName_contains`, `labelName_contains_nocase`, `labelName_starts_with`, and `labelName_ends_with` on `Domain` and `Registration`.
- `createdAt_*`, `expiryDate_*`, and `ttl_*` on `Domain`.
- `labelhash`, `subdomainCount_*`, `resolvedAddress`, and `isMigrated` on `Domain`.
- `registrationDate_*`, `expiryDate_*`, and `cost_*` on `Registration`.
- `expiryDate_*` and `fuses_*` on `WrappedDomain`.
- `address`, `address_in`, `addr`, `contentHash_*`, `texts_contains`, and `coinTypes_contains` on `Resolver`.
- event parent filters, ID predicates, full `blockNumber_*` comparisons, `transactionID` equality/list predicates, and concrete/interface event-specific predicates for stored fields such as `domain_*`, `registration_*`, parent and new-resolver `resolver_*`, `parentDomain_*`, `owner_*`, `registrant_*`, `newOwner_*`, `addr_*`, `fuses_*`, `expiryDate_*`, `coinType_*`, `key`, `hash`, `interfaceID`, `isAuthorized`, and `version_*`.
- event `_change_block.number_gte` predicates for concrete event and event-interface queries.
- mutable-entity `_change_block.number_gte` predicates for `Account`, `Domain`, `Registration`, `WrappedDomain`, and `Resolver` queries.
- shallow trailing-underscore relationship filters on mutable entities: `Domain.parent_`, `Domain.owner_`, `Domain.resolver_`, `Domain.registrant_`, `Domain.wrappedOwner_`, `Registration.domain_`, `Registration.registrant_`, `WrappedDomain.domain_`, `WrappedDomain.owner_`, `Resolver.domain_`, and `Resolver.addr_`.

Tier 2 filters:

- `_not` and comparison operators for every scalar;
- `_contains_nocase`, `_starts_with_nocase`, `_ends_with_nocase` for all strings;
- recursive trailing-underscore filters beyond current mutable-entity and event relation composition;
- deeper event-interface relationship filtering over union queries;
- deeper recursive `and` and `or` semantics across relationship-filter boundaries.

Tier 3 compatibility:

- `_change_block`;
- historical `block` argument;
- list-field edge cases;
- audit-driven generated filter edge cases found by official/local query comparisons.

Implementation pattern:

```rust
pub enum DomainOrderBy {
    Id,
    Name,
    LabelName,
    ParentName,
    OwnerId,
    ResolverAddress,
    RegistrationExpiryDate,
}

pub struct DomainFilter {
    pub id: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub name_contains_nocase: Option<String>,
    pub parent: Option<String>,
    pub parent_: Option<Box<DomainFilter>>,
    pub owner_: Option<Box<AccountFilter>>,
    pub and: Option<Vec<DomainFilter>>,
    pub or: Option<Vec<DomainFilter>>,
}
```

Compile filters to parameterized SQL. Nested filters should become `exists` subqueries or explicit joins chosen by the query builder. Keep enum-to-SQL mappings exhaustive and reviewed.

## Step 11: Server and Operations

Implement `server` as the deployable HTTP service.

Routes:

| Route           | Purpose                                       |
| --------------- | --------------------------------------------- |
| `POST /graphql` | GraphQL queries                               |
| `GET /graphql`  | optional GraphQL playground in non-production |
| `GET /healthz`  | process is alive                              |
| `GET /readyz`   | database reachable and indexer initialized    |
| `GET /metrics`  | optional Prometheus metrics                   |

Tower layers:

- tracing;
- timeout;
- request body limit;
- CORS;
- compression;
- panic/error handling.

Runtime config:

| Variable                     | Purpose                                                     |
| ---------------------------- | ----------------------------------------------------------- |
| `DATABASE_URL`               | Postgres connection string                                  |
| `ETH_RPC_URL`                | Ethereum JSON-RPC endpoint                                  |
| `ETH_WS_URL`                 | Ethereum websocket endpoint, required when `LIVE_INDEXING_SOURCE=wss` |
| `ENVIO_API_KEY`              | Envio HyperSync API key for fast historical backfills       |
| `HYPERSYNC_URL`              | HyperSync endpoint, default `https://eth.hypersync.xyz`     |
| `ENABLE_BACKFILL`            | run configured startup backfill in `ensindexer start`       |
| `ENABLE_LIVE_INDEXING`       | run live confirmed-block indexing in `ensindexer start`     |
| `BACKFILL_SOURCE`            | strict historical source selector: `rpc`, `hypersync`, or `raw` |
| `LIVE_INDEXING_SOURCE`       | strict live source selector: `rpc` or `wss`                 |
| `ARCHIVE_BACKFILLS`          | write fetched backfill ranges to `RAW_ARCHIVE_DIR` when true |
| `RAW_ARCHIVE_DIR`            | archive directory used for backfill writes and raw replay   |
| `CHAIN_ID`                   | chain selector                                              |
| `INDEXER_CONFIRMATION_DEPTH` | live indexing lag                                           |
| `BACKFILL_BATCH_BLOCKS`      | initial range size                                          |
| `LIVE_POLL_SECONDS`          | live loop sleep interval when the safe head has no new work |
| `SUBGRAPH_URL`               | official or reference GraphQL endpoint for diff checks      |
| `SUBGRAPH_AUTH_TOKEN`        | optional bearer token for the reference GraphQL endpoint    |

## Step 12: CLI Commands

The production CLI is intentionally small:

```text
ensindexer start
ensindexer status
```

`ensindexer start` always runs HTTP, GraphQL, and Apollo Sandbox. Startup backfill, live indexing, historical source selection, live transport selection, and raw archive writes are controlled through strict env values and equivalent startup flags. `ensindexer status` prints the latest locally stored block and per-source checkpoints.

Schema diffing, reference-subgraph comparisons, benchmark execution, label healing, archive inspection, and destructive reset helpers are internal tooling concerns. They should live in separate development binaries or scripts instead of the public production binary.

Example:

```text
ENABLE_BACKFILL=true \
BACKFILL_SOURCE=hypersync \
ARCHIVE_BACKFILLS=true \
RAW_ARCHIVE_DIR=.raw-archive \
ensindexer start
```

Future internal comparison work should add fixture sets, response normalization for intentionally unordered lists, and a mode that compares many query/variables pairs in one run.

## Testing Strategy

Unit tests:

| Area              | Examples                                                              |
| ----------------- | --------------------------------------------------------------------- |
| IDs               | namehash, labelhash conversion, resolver ID, event ID, batch event ID |
| names             | DNS name decoding, bracketed unknown labels, invalid labels           |
| scalar formatting | lowercase hex, decimal BigInt strings                                 |
| filters           | operator-to-SQL compilation with expected parameters                  |

Projection integration tests:

- start a transaction;
- create synthetic decoded events;
- apply handlers;
- query tables;
- roll back transaction.

Cover at least:

| Handler   | Test cases                                                                                           |
| --------- | ---------------------------------------------------------------------------------------------------- |
| registry  | new owner creates subdomain, transfer updates owner, old registry does not overwrite migrated domain |
| registrar | register, renew, transfer, controller enrichment                                                     |
| wrapper   | wrap, unwrap, transfer single, transfer batch, fuses, expiry extension                               |
| resolver  | ETH addr, multicoin addr, text updates, contenthash, version reset                                   |

SQL integration tests:

- use a local Postgres test database or testcontainers;
- run migrations from scratch;
- verify repository methods and indexes;
- test pagination and ordering queries.

GraphQL tests:

- execute `async-graphql` schema in-process;
- snapshot representative JSON responses;
- verify field nullability;
- verify nested relationship fields;
- verify interface query results include concrete event fields.

Differential tests:

1. Pick stable entity IDs from mainnet.
2. Query official subgraph and local Rust API.
3. Normalize JSON key order.
4. Compare exact entity IDs, scalar values, event IDs, and nested relationships.

Example query classes:

```graphql
query DomainSmoke {
  domain(id: "0x...") {
    id
    name
    owner { id }
    resolver { id address addr { id } texts coinTypes }
    registration { id expiryDate registrant { id } }
    wrappedDomain { id fuses expiryDate owner { id } }
  }
}
```

```graphql
query FilterSmoke {
  domains(
    first: 25
    orderBy: name
    orderDirection: asc
    where: { name_contains_nocase: "eth", owner_: { id: "0x..." } }
  ) {
    id
    name
    owner { id }
  }
}
```

Backfill smoke tests:

- run against a small historical block range;
- assert known events are inserted;
- assert checkpoints advance only after successful commit;
- rerun the same range and verify idempotency.

Reorg tests:

- create fake blocks `N`, `N+1`, `N+2`;
- apply events;
- replace `N+1` with another parent/hash path;
- verify rollback and replay restore canonical state.

## Suggested Delivery Order

Milestone 0: documentation and scaffolding.

- Workspace crates exist.
- Shared dependencies are pinned.
- Formatting, clippy, and tests run.

Milestone 1: types, constants, and ABI decoding.

- IDs and scalar wrappers are implemented.
- Contract event bindings compile.
- Synthetic log decoding tests pass.

Milestone 2: database migrations and repositories.

- Mutable tables and event tables exist.
- Repositories can upsert accounts/domains/registrations/wrapped domains/resolvers.
- SQL integration tests pass.

Milestone 3: projection parity for fixed-address contracts.

- Registry, base registrar, controllers, and name wrapper handlers are implemented.
- Projection integration tests cover official edge cases.
- A small historical backfill produces expected rows.

Milestone 4: resolver indexing.

- Wildcard resolver logs are fetched by topic.
- Resolver rows and resolver event tables are projected.
- `Domain.resolvedAddress` syncs from active resolver ETH address records.

Milestone 5: GraphQL base API.

- Entity and concrete event singular/plural queries work.
- `skip`, `first`, scalar ordering, and tier 1 filters work.
- Derived relationship fields work without obvious N+1 behavior.
- Current code has the entity queries and concrete event queries; batched relationship loading should still be added before heavy production traffic.

Milestone 6: GraphQL compatibility expansion.

- Event interfaces work.
- Nested trailing-underscore filters work.
- Relationship order fields work for the common official enum variants.
- `_meta` is implemented.

Milestone 7: live indexing and reorg handling.

- Confirmation-depth live tailing works.
- Parent hash checks detect reorgs.
- Coarse reset-and-rebuild repair exists.
- Common-ancestor rollback path is tested.

Milestone 8: differential validation and production hardening.

- Query fixture comparison against the official subgraph passes for target queries.
- Backfill can resume from checkpoints.
- Metrics, tracing, health checks, and deployment config are in place.

## Rough Timeline

This assumes one engineer working steadily with the official subgraph behavior already researched.

| Time    | Focus                                                                 | Expected output                                                 |
| ------- | --------------------------------------------------------------------- | --------------------------------------------------------------- |
| Week 1  | workspace, types, constants, ABI decoding, migrations                 | compiling crates, first migrations, decoding tests              |
| Week 2  | storage repositories and registry/registrar projections               | entity state for `.eth` registrations and registry ownership    |
| Week 3  | name wrapper projections and historical backfill engine               | wrapper state, fixed-address backfill, checkpointing            |
| Week 4  | dynamic resolver ingestion and resolver projections                   | resolver records, addr/text/contenthash events                  |
| Week 5  | GraphQL base API and tier 1 filters/order                             | usable subgraph-compatible API for core queries                 |
| Week 6  | event interfaces, nested filters, live indexing, reorg path           | compatibility expansion and production readiness                |
| Week 7+ | differential validation, performance tuning, nested historical context audits, rollback hardening | confidence against official subgraph and client-specific polish |

The critical path is projection correctness. API completeness can be expanded incrementally once the database state is trustworthy.

## Compatibility Risks

| Risk                                  | Mitigation                                                                                              |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Graph Node historical `block` queries | start by accepting the argument, then add entity change tracking or snapshot tables                     |
| Historical block reads and rollback    | add entity snapshots or reversible change payloads keyed by block                                       |
| resolver log volume                   | adaptive batch sizing, resolver-address chunking, topic-specific checkpoints, provider-limit handling   |
| invalid/unknown name labels           | port official helper behavior exactly and test it with official fixture cases                           |
| old registry migration semantics      | keep explicit `is_migrated` behavior in projection tests                                                |
| event interface ordering              | query all concrete event tables with `union all`, sort by requested order, and paginate after the union |
| BigInt/Bytes serialization drift      | snapshot GraphQL responses against official subgraph samples                                            |

## First Build Slice

The smallest useful vertical slice is:

1. `types`: IDs, constants, event context.
2. `contracts`: registry and base registrar decoding.
3. `storage`: accounts, domains, registrations, basic event tables.
4. `projection`: registry plus base registrar handlers.
5. `ingest`: bounded backfill over registry/base registrar.
6. `api`: `domain`, `domains`, `registration`, `registrations`, `account`, `accounts`.
7. `server`: `/graphql`, `/healthz`, `/readyz`.

After that slice works on a small historical range, add name wrapper, then dynamic resolver logs, then full GraphQL compatibility.
