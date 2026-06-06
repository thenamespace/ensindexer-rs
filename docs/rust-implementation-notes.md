# Rust Implementation Notes

This project is moving to a custom indexer built from scratch with modern Rust crates:

- `axum`, `tower`, `tower-http`
- `async-graphql`, `async-graphql-axum`
- `alloy`
- `sqlx`
- `thiserror`, `anyhow`
- `serde`, `serde_json`

This document translates the official subgraph into maintainable Rust architecture.

For the concrete build sequence, crate creation order, API rollout order, and testing timeline, see [Rust Implementation Roadmap](implementation-roadmap.md).

## Suggested Workspace Crates

The workspace currently uses `crates/*`. Suggested split:

| Crate | Responsibility |
| --- | --- |
| `types` | shared IDs, chain config, constants, GraphQL scalar wrappers, DTO primitives |
| `contracts` | alloy ABI bindings, event structs, topic constants |
| `config` | `.env` loading, typed app config, runtime defaults |
| `storage` | SQLx pool, migrations, repositories, transactional projection writes |
| `projection` | pure projection handlers from decoded events to storage operations |
| `ingest` | RPC log fetching, cursor/checkpointing, reorg strategy, decoding, dispatcher |
| `api` | async-graphql schema, resolvers, filters, pagination |
| `server` | axum HTTP server, health, metrics, GraphQL routes |
| `cli` | backfill, run-indexer, reset, inspect, one-off maintenance commands |

Keep ABI/event decoding separate from projection logic. Projection handlers should be testable without RPC.

## Core DTOs

### Chain Log Context

Every handler needs the same metadata:

```rust
pub struct LogContext {
    pub chain_id: u64,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub block_hash: B256,
    pub transaction_hash: B256,
    pub transaction_index: u64,
    pub log_index: u64,
    pub contract_address: Address,
}

impl LogContext {
    pub fn event_id(&self) -> String {
        format!("{}-{}", self.block_number, self.log_index)
    }

    pub fn batch_event_id(&self, index: usize) -> String {
        format!("{}-{}-{}", self.block_number, self.log_index, index)
    }
}
```

### IDs

Use explicit newtypes to avoid mixing address, namehash, labelhash, and resolver IDs:

```rust
pub struct NodeId(pub B256);
pub struct LabelHash(pub B256);
pub struct AccountId(pub Address);
pub struct ResolverId {
    pub address: Address,
    pub node: B256,
}
```

Serialize external IDs as lowercase `0x` hex strings.

### BigInt Handling

The official schema exposes `BigInt`. Options:

1. SQL `numeric` plus Rust `bigdecimal::BigDecimal`;
2. SQL `numeric` plus decimal-string API mapping;
3. SQL `text` for decimal strings.

For production, prefer `numeric` in Postgres and DTO-level string serialization for GraphQL compatibility. `alloy::primitives::U256` is excellent for event values but SQLx does not natively bind it to `numeric`; convert at repository boundaries.

## Storage Model

Recommended tables:

- `accounts`
- `domains`
- `registrations`
- `wrapped_domains`
- `resolvers`
- concrete event tables:
  - `domain_transfer_events`
  - `new_owner_events`
  - `new_resolver_events`
  - `new_ttl_events`
  - `wrapped_transfer_events`
  - `name_wrapped_events`
  - `name_unwrapped_events`
  - `fuses_set_events`
  - `expiry_extended_events`
  - `name_registered_events`
  - `name_renewed_events`
  - `name_transferred_events`
  - resolver event tables
- ingestion tables:
  - `indexed_logs`
  - `event_checkpoints`
  - `blocks`
  - optional `canonical_chain_segments` for reorg handling

Concrete event tables are more verbose, but they map cleanly to GraphQL object types and non-null fields. A single wide event table is faster to prototype but makes GraphQL interface resolution less precise.

## Ingestion Strategy

### Fixed-address sources

Use `eth_getLogs` with:

- address list for fixed contracts;
- topic0 list for the events each contract needs;
- bounded block ranges.

Decode with alloy generated bindings or `sol!` event definitions.

### Resolver dynamic-address source

The official resolver data source has no address. A direct topic-only `eth_getLogs` query would scan every contract that emits resolver-shaped events, which is too broad for production RPC providers. For Rust:

- maintain resolver contract addresses discovered from registry `NewResolver` events and persisted resolver rows;
- fetch logs with an address chunk plus resolver topic0 signatures;
- include all resolver topic0 signatures;
- decode each log by matching topic0 and ABI;
- resolver contract address is `log.address`.

This still requires conservative batch sizes, address chunking, and persistent checkpoints, but avoids a global mainnet wildcard scan.

### Global Ordering

The official Graph Node applies handlers in log order. To reproduce behavior:

1. fetch logs from all sources for a block range;
2. decode them into a common enum;
3. sort by `(block_number, transaction_index, log_index)`;
4. apply projections in a single transaction or in deterministic chunks.

If processing event streams independently, you must add placeholder creation and conflict-safe updates. That works for many cases but can diverge subtly from the subgraph.

## Event Enum

Create one decoded event enum:

```rust
pub enum EnsEvent {
    RegistryTransfer { node: B256, owner: Address },
    RegistryNewOwner { node: B256, label: B256, owner: Address, migrated: bool },
    RegistryNewResolver { node: B256, resolver: Address, old_registry: bool },
    RegistryNewTtl { node: B256, ttl: U256, old_registry: bool },
    BaseNameRegistered { id: U256, owner: Address, expires: U256 },
    BaseNameRenewed { id: U256, expires: U256 },
    BaseTransfer { from: Address, to: Address, token_id: U256 },
    ControllerNamePreimage { label: String, labelhash: B256, cost: U256 },
    NameWrapped { node: B256, dns_name: Bytes, owner: Address, fuses: u32, expiry: U256 },
    NameUnwrapped { node: B256, owner: Address },
    FusesSet { node: B256, fuses: u32 },
    ExpiryExtended { node: B256, expiry: U256 },
    WrappedTransferSingle { to: Address, id: U256 },
    WrappedTransferBatch { to: Address, ids: Vec<U256> },
    ResolverAddrChanged { node: B256, addr: Address },
    // ...resolver events
}
```

Attach `LogContext` separately:

```rust
pub struct IndexedEvent {
    pub ctx: LogContext,
    pub event: EnsEvent,
}
```

## Projection Handler Pattern

Handlers should be small and transaction-oriented:

```rust
pub async fn apply_event(tx: &mut PgTransaction<'_>, event: IndexedEvent) -> Result<(), ProjectionError> {
    match event.event {
        EnsEvent::RegistryNewOwner { .. } => registry::new_owner(tx, event).await,
        EnsEvent::NameWrapped { .. } => wrapper::name_wrapped(tx, event).await,
        _ => todo!(),
    }
}
```

Use repository functions for persistence:

- `accounts::create_if_missing`
- `domains::create_if_missing`
- `domains::set_owner`
- `registrations::upsert`
- `events::insert_name_wrapped`

Avoid embedding SQL in projection handlers once the behavior is stable.

## GraphQL API Design

Use `async-graphql` objects for GraphQL DTOs, not storage rows directly. Storage rows are optimized for SQL; API DTOs should preserve subgraph naming:

- `labelName`, not `label_name`;
- `subdomainCount`, not `subdomain_count`;
- `resolvedAddress`, not `resolved_address_id`;
- `transactionID`, not `transaction_hash`.

Suggested layers:

```text
HTTP axum route
  -> async_graphql schema
    -> query resolver
      -> repository query with filters
        -> row -> GraphQL DTO mapping
```

Derived fields should be resolver methods:

- `Domain.subdomains`: query domains by `parent_id`;
- `Domain.registration`: query registration by `domain_id`;
- `Domain.wrappedDomain`: query wrapped domain by `domain_id`;
- `Domain.events`: aggregate domain event tables;
- `Account.domains`: query domains by `owner_id`;
- `Resolver.events`: aggregate resolver event tables.

## Compatibility Checklist

Before claiming drop-in compatibility, verify:

- all official schema entities exist in GraphQL;
- all official fields have matching nullability;
- entity IDs match the official subgraph;
- event IDs match `{block}-{log}` and batch suffixes;
- old registry migration guards match;
- resolver IDs match `{address}-{node}`;
- `TextChanged` overloads both map into `TextChanged`;
- `VersionChanged` resets resolver mutable records;
- `.eth` registration domain expiry is `expires + 7776000`;
- `.eth` unwrapping keeps expiry, subname unwrapping clears expiry;
- invalid labels are rejected exactly as official helper does;
- wrapper transfer placeholder rows are created;
- controller preimage events only enrich existing registrations;
- GraphQL derived relationships work;
- pagination/order/filter semantics are close enough for existing clients.

## Open Design Choices

### Exact Graph Node Query Compatibility

Graph Node supports a large generated filtering API. Recreating every filter suffix (`_not`, `_in`, `_contains`, nested filters, etc.) is possible but not MVP-sized. Decide which filters are required by your target clients.

### Byte Storage

`bytea` is more natural for Postgres. Hex `text` is simpler for GraphQL compatibility and debugging. For a first production MVP, normalized hex `text` is acceptable if indexed.

### Reorg Handling

The official subgraph relies on Graph Node reorg support. A custom indexer needs its own strategy:

- store block hash per processed block;
- on new head, compare parent hash to stored canonical hash;
- roll back events and mutable projections for reverted blocks, or rebuild from a safe checkpoint;
- use a confirmation depth for live indexing.

For MVP mainnet historical indexing, process finalized ranges first. For live indexing, use a reorg-safe lag.
