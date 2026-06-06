# Schema and GraphQL Shape

This document turns the official `schema.graphql` into implementation notes for SQL storage and an `async-graphql` API.

## Scalar Mapping

| Graph scalar | Meaning | Rust/API representation | SQL representation |
| --- | --- | --- | --- |
| `ID!` | stable entity ID string | `String` | `text primary key` |
| `String` | UTF-8 text | `Option<String>` / `String` | `text` |
| `Bytes` | hex-encoded bytes in GraphQL | custom bytes scalar or hex `String` | `bytea` or normalized hex `text` |
| `BigInt` | arbitrary precision integer | `alloy::primitives::U256`, `num_bigint`, or decimal string in API DTOs | `numeric` or decimal `text` |
| `Int` | GraphQL 32-bit signed int | `i32` | `integer` |
| `Boolean` | bool | `bool` | `boolean` |

For a subgraph-compatible public API, returning bytes as `0x`-prefixed lowercase hex strings is the safest external representation. Internally, `bytea` is more compact, but `text` avoids repeated encode/decode complexity. Pick one and normalize at the boundary.

## Entity: Domain

`Domain` is the central entity. Its ID is the ENS namehash.

GraphQL fields:

| Field | Type | Stored? | Notes |
| --- | --- | --- | --- |
| `id` | `ID!` | yes | namehash hex |
| `name` | `String` | yes | full known name, e.g. `foo.eth`; unknown labels become bracketed hashes |
| `labelName` | `String` | yes | known label only |
| `labelhash` | `Bytes` | yes | 32-byte labelhash |
| `parent` | `Domain` | yes | parent domain ID FK |
| `subdomains` | `[Domain!]!` | derived | query `domains where parent = this.id` |
| `subdomainCount` | `Int!` | yes | incremented when a domain first gets a parent |
| `resolvedAddress` | `Account` | yes | synced from active resolver `addr` record |
| `resolver` | `Resolver` | yes | resolver ID `{address}-{node}` |
| `ttl` | `BigInt` | yes | registry TTL |
| `isMigrated` | `Boolean!` | yes | old/new registry migration guard |
| `createdAt` | `BigInt!` | yes | block timestamp when created |
| `owner` | `Account!` | yes | registry owner |
| `registrant` | `Account` | yes | `.eth` ERC-721 owner |
| `wrappedOwner` | `Account` | yes | ERC-1155 name wrapper owner |
| `expiryDate` | `BigInt` | yes | registration expiry + grace or wrapped expiry if PCC burned |
| `registration` | `Registration` | derived | query `registrations where domain = this.id` |
| `wrappedDomain` | `WrappedDomain` | derived | query `wrapped_domains where domain = this.id` |
| `events` | `[DomainEvent!]!` | derived | union/interface over domain event tables |

Suggested SQL table:

```sql
create table domains (
  id text primary key,
  name text,
  label_name text,
  labelhash text,
  parent_id text references domains(id),
  subdomain_count integer not null default 0,
  resolved_address_id text references accounts(id),
  resolver_id text references resolvers(id),
  ttl numeric,
  is_migrated boolean not null default false,
  created_at numeric not null,
  owner_id text not null references accounts(id),
  registrant_id text references accounts(id),
  wrapped_owner_id text references accounts(id),
  expiry_date numeric
);
```

## Domain Event Interface

The official schema uses `interface DomainEvent`. SQL cannot store GraphQL interfaces directly. Use either:

1. one table per event type, closest to Graph Node; or
2. one `domain_events` table with an `event_type` discriminator and nullable fields.

For subgraph-compatible GraphQL, `async-graphql` should expose an interface or union with concrete object types.

Shared fields:

| Field | Type |
| --- | --- |
| `id` | `ID!` |
| `domain` | `Domain!` |
| `blockNumber` | `Int!` |
| `transactionID` | `Bytes!` |

Concrete domain events:

| Type | Extra fields |
| --- | --- |
| `Transfer` | `owner: Account!` |
| `NewOwner` | `parentDomain: Domain!`, `owner: Account!` |
| `NewResolver` | `resolver: Resolver!` |
| `NewTTL` | `ttl: BigInt!` |
| `WrappedTransfer` | `owner: Account!` |
| `NameWrapped` | `name: String`, `fuses: Int!`, `owner: Account!`, `expiryDate: BigInt!` |
| `NameUnwrapped` | `owner: Account!` |
| `FusesSet` | `fuses: Int!` |
| `ExpiryExtended` | `expiryDate: BigInt!` |

## Entity: Registration

`Registration` represents a `.eth` second-level registration.

ID:

```text
0x + 32-byte labelhash
```

Fields:

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `ID!` | labelhash hex |
| `domain` | `Domain!` | namehash of `{label}.eth` |
| `registrationDate` | `BigInt!` | base registrar event block timestamp |
| `expiryDate` | `BigInt!` | expiry from base registrar |
| `cost` | `BigInt` | set by controller events when preimage is known |
| `registrant` | `Account!` | ERC-721 owner |
| `labelName` | `String` | known label preimage |
| `events` | `[RegistrationEvent!]!` | derived |

Concrete registration events:

| Type | Extra fields |
| --- | --- |
| `NameRegistered` | `registrant: Account!`, `expiryDate: BigInt!` |
| `NameRenewed` | `expiryDate: BigInt!` |
| `NameTransferred` | `newOwner: Account!` |

## Entity: WrappedDomain

ID is the wrapped node/namehash.

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `ID!` | namehash hex |
| `domain` | `Domain!` | same ID as domain |
| `expiryDate` | `BigInt!` | wrapper expiry |
| `fuses` | `Int!` | `uint32` from wrapper events, stored as GraphQL `Int` |
| `owner` | `Account!` | wrapped owner |
| `name` | `String` | decoded DNS-encoded name from `NameWrapped` |

Placeholder behavior: ERC-1155 transfer events can create a `WrappedDomain` with `expiryDate = 0`, `fuses = 0`, and owner set from `to`.

## Entity: Account

ID is address hex.

Fields:

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `ID!` | address hex |
| `domains` | `[Domain!]!` | derived by `owner` |
| `wrappedDomains` | `[WrappedDomain!]` | derived by `owner` |
| `registrations` | `[Registration!]` | derived by `registrant` |

## Entity: Resolver

ID:

```text
{resolver_address}-{node}
```

Fields:

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `ID!` | resolver address plus domain |
| `domain` | `Domain` | node/namehash |
| `address` | `Bytes!` | resolver contract address |
| `addr` | `Account` | current ETH address record |
| `contentHash` | `Bytes` | current contenthash |
| `texts` | `[String!]` | observed text keys |
| `coinTypes` | `[BigInt!]` | observed multicoin coin types |
| `events` | `[ResolverEvent!]!` | derived |

Resolver event types:

| Type | Extra fields |
| --- | --- |
| `AddrChanged` | `addr: Account!` |
| `MulticoinAddrChanged` | `coinType: BigInt!`, `addr: Bytes!` |
| `NameChanged` | `name: String!` |
| `AbiChanged` | `contentType: BigInt!` |
| `PubkeyChanged` | `x: Bytes!`, `y: Bytes!` |
| `TextChanged` | `key: String!`, `value: String` |
| `ContenthashChanged` | `hash: Bytes!` |
| `InterfaceChanged` | `interfaceID: Bytes!`, `implementer: Bytes!` |
| `AuthorisationChanged` | `owner: Bytes!`, `target: Bytes!`, `isAuthorized: Boolean!` |
| `VersionChanged` | `version: BigInt!` |

## GraphQL API Shape

Graph Node typically exposes:

```graphql
type Query {
  domain(id: ID!): Domain
  domains(...): [Domain!]!
  registration(id: ID!): Registration
  registrations(...): [Registration!]!
  wrappedDomain(id: ID!): WrappedDomain
  wrappedDomains(...): [WrappedDomain!]!
  account(id: ID!): Account
  accounts(...): [Account!]!
  resolver(id: ID!): Resolver
  resolvers(...): [Resolver!]!
}
```

It also exposes concrete event entity queries. A compatible Rust API should support singular and plural queries for each entity and event type.

The Graph supports filtering, ordering, pagination, and derived relationships. For MVP compatibility, prioritize:

- `id` lookups for every entity;
- `first`, `skip`, `orderBy`, `orderDirection` for list queries;
- common filters: `id`, `id_in`, `name`, `owner`, `parent`, `labelName`, `resolver`, `registrant`, `wrappedOwner`, `domain`;
- derived relationship resolvers.

`async-graphql` implementation options:

- use `#[derive(SimpleObject)]` for concrete DTOs;
- use `#[derive(Interface)]` for `DomainEvent`, `RegistrationEvent`, and `ResolverEvent`;
- expose bytes and big integers as strings unless you implement custom scalars that exactly match The Graph responses.

## Hosted Subgraph Query Surface

The public hosted subgraph exposes singular and plural root queries for every object type in the schema, including mutable entities, concrete event entities, and event interfaces.

For each entity/event `T`, Graph Node generates:

```graphql
t(id: ID!, block: Block_height, subgraphError: _SubgraphErrorPolicy_): T
ts(
  skip: Int = 0
  first: Int = 100
  orderBy: T_orderBy
  orderDirection: OrderDirection
  where: T_filter
  block: Block_height
  subgraphError: _SubgraphErrorPolicy_ = deny
): [T!]!
```

The root query also exposes:

```graphql
_meta(block: Block_height): _Meta_
```

Current implementation exposes `_meta(block: Block_height)` and supports `hash`, `number`, and `number_gte` lookup against stored indexed blocks. Mutable-entity, concrete-event, and event-interface root queries accept `block: Block_height` and `subgraphError: _SubgraphErrorPolicy_`; current-state reads work, while non-current block reads return a clear compatibility error because historical snapshots are not implemented yet.

`OrderDirection` is:

```graphql
enum OrderDirection {
  asc
  desc
}
```

`Block_height` contains:

```graphql
input Block_height {
  hash: Bytes
  number: Int
  number_gte: Int
}
```

For a Rust replacement, support the arguments even if the first version only serves canonical current-state data. If historical `block` reads are not implemented at first, keep the input in the GraphQL schema and return a clear compatibility error for non-current block requests.

### Root Queries

The hosted subgraph exposes at least these query pairs:

| Entity/interface | Singular | Plural |
| --- | --- | --- |
| `Domain` | `domain` | `domains` |
| `Registration` | `registration` | `registrations` |
| `WrappedDomain` | `wrappedDomain` | `wrappedDomains` |
| `Account` | `account` | `accounts` |
| `Resolver` | `resolver` | `resolvers` |
| `DomainEvent` | `domainEvent` | `domainEvents` |
| `RegistrationEvent` | `registrationEvent` | `registrationEvents` |
| `ResolverEvent` | `resolverEvent` | `resolverEvents` |
| `Transfer` | `transfer` | `transfers` |
| `NewOwner` | `newOwner` | `newOwners` |
| `NewResolver` | `newResolver` | `newResolvers` |
| `NewTTL` | `newTTL` | `newTTLs` |
| `WrappedTransfer` | `wrappedTransfer` | `wrappedTransfers` |
| `NameWrapped` | `nameWrapped` | `nameWrappeds` |
| `NameUnwrapped` | `nameUnwrapped` | `nameUnwrappeds` |
| `FusesSet` | `fusesSet` | `fusesSets` |
| `ExpiryExtended` | `expiryExtended` | `expiryExtendeds` |
| `NameRegistered` | `nameRegistered` | `nameRegistereds` |
| `NameRenewed` | `nameRenewed` | `nameReneweds` |
| `NameTransferred` | `nameTransferred` | `nameTransferreds` |
| `AddrChanged` | `addrChanged` | `addrChangeds` |
| `MulticoinAddrChanged` | `multicoinAddrChanged` | `multicoinAddrChangeds` |
| `NameChanged` | `nameChanged` | `nameChangeds` |
| `AbiChanged` | `abiChanged` | `abiChangeds` |
| `PubkeyChanged` | `pubkeyChanged` | `pubkeyChangeds` |
| `TextChanged` | `textChanged` | `textChangeds` |
| `ContenthashChanged` | `contenthashChanged` | `contenthashChangeds` |
| `InterfaceChanged` | `interfaceChanged` | `interfaceChangeds` |
| `AuthorisationChanged` | `authorisationChanged` | `authorisationChangeds` |
| `VersionChanged` | `versionChanged` | `versionChangeds` |

### Filter Shape

Graph Node generates broad filter inputs for every object. The same pattern repeats across `Domain_filter`, `Registration_filter`, `Resolver_filter`, concrete event filters, and interface filters.

Scalar fields support:

| Field type | Generated operators |
| --- | --- |
| `ID`, `String` | base equality, `_not`, `_gt`, `_lt`, `_gte`, `_lte`, `_in`, `_not_in`, `_contains`, `_contains_nocase`, `_not_contains`, `_not_contains_nocase`, `_starts_with`, `_starts_with_nocase`, `_not_starts_with`, `_not_starts_with_nocase`, `_ends_with`, `_ends_with_nocase`, `_not_ends_with`, `_not_ends_with_nocase` |
| `Bytes` | base equality, `_not`, `_gt`, `_lt`, `_gte`, `_lte`, `_in`, `_not_in`, `_contains`, `_not_contains` |
| `Int`, `BigInt` | base equality, `_not`, `_gt`, `_lt`, `_gte`, `_lte`, `_in`, `_not_in` |
| `Boolean` | base equality, `_not`, `_in`, `_not_in` |
| lists | base equality and list-specific generated operators where Graph Node supports them |

Every filter also includes:

```graphql
and: [T_filter]
or: [T_filter]
_change_block: BlockChangedFilter
```

Current implementation supports `and` and `or` composition for `Account_filter`, including account-backed relationship filters such as `owner_`, `registrant_`, `wrappedOwner_`, `resolvedAddress_`, and `addr_`. It also supports scalar-compatible `and` and `or` composition for `Domain_filter`, `Registration_filter`, `Resolver_filter`, and `WrappedDomain_filter`. The same composition pattern still needs to be expanded to event filters and deeper recursive relationship predicates.

Relationship filters use Graph Node's trailing underscore convention. Examples:

```graphql
domains(where: { parent_: { name: "eth" }, owner_: { id: "0x..." } })
registrations(where: { domain_: { labelName_contains_nocase: "foo" } })
resolvers(where: { addr_: { id_in: ["0x...", "0x..."] } })
nameWrappeds(where: { domain_: { name_ends_with: ".eth" }, owner_: { id: "0x..." } })
textChangeds(where: { resolver_: { address: "0x..." }, key: "avatar" })
```

In SQL, trailing-underscore filters should compile into joins or `exists` subqueries. Prefer `exists` for optional relationships because it preserves Graph Node behavior around null relationships:

```sql
where exists (
  select 1
  from domains parent
  where parent.id = domains.parent_id
    and lower(parent.name) = lower($1)
)
```

Current implementation supports shallow trailing-underscore filters on mutable entities:

- `Domain_filter`: `parent_`, `owner_`, `resolver_`, `registrant_`, `wrappedOwner_`;
- `Registration_filter`: `domain_`, `registrant_`;
- `WrappedDomain_filter`: `domain_`, `owner_`;
- `Resolver_filter`: `domain_`, `addr_`.

Those relationship filters apply scalar predicates on the directly related entity. Fully recursive relationship filtering and boolean `and`/`or` composition remain compatibility-expansion work.

Current scalar filter coverage includes the main stored mutable-entity fields:

- `Domain_filter`: ID predicates, `name_*`, `labelName_*`, `labelhash`, `parent`, `subdomainCount_*`, `resolvedAddress`, `owner`, `resolver`, `registrant`, `wrappedOwner`, `isMigrated`, `createdAt_*`, `expiryDate_*`, and `ttl_*`;
- `Registration_filter`: ID predicates, `domain`, `registrant`, `labelName_*`, `registrationDate_*`, `expiryDate_*`, and `cost_*`;
- `WrappedDomain_filter`: ID predicates, `domain`, `owner`, `name_*`, `expiryDate_*`, and `fuses_*`;
- `Resolver_filter`: ID predicates, `domain`, `address`, `address_in`, `addr`, `contentHash_*`, `texts_contains`, and `coinTypes_contains`.

Concrete event filters use the shared `EventFilter` input plus table-aware event-specific predicates. The current implementation applies event-specific fields only on concrete event queries, not on interface union queries:

- ownership/account fields: `owner`, `parentDomain`, `registrant`, `newOwner`, and `addr`;
- domain wrapper fields: `name`, `name_contains`, `name_contains_nocase`, `fuses_*`, and `expiryDate_*`;
- registration event fields: `registrant`, `newOwner`, and `expiryDate_*`;
- resolver record fields: `coinType_*`, `contentType_*`, `x`, `y`, `key`, `key_contains`, `value`, `value_contains`, `hash`, `interfaceID`, `implementer`, `target`, `isAuthorized`, and `version_*`;
- registry TTL field: `ttl_*`.

### Ordering Shape

Graph Node allows ordering by local scalar fields and selected fields from directly related objects. Relationship ordering uses a double underscore. Examples from the hosted subgraph:

| Object | Important `orderBy` values |
| --- | --- |
| `Domain` | `id`, `name`, `labelName`, `labelhash`, `parent`, `parent__name`, `subdomainCount`, `resolvedAddress`, `resolver`, `resolver__address`, `ttl`, `isMigrated`, `createdAt`, `owner`, `owner__id`, `registrant`, `wrappedOwner`, `expiryDate`, `registration__expiryDate`, `wrappedDomain__fuses`, `events` |
| `Registration` | `id`, `domain`, `domain__name`, `registrationDate`, `expiryDate`, `cost`, `registrant`, `registrant__id`, `labelName`, `events` |
| `WrappedDomain` | `id`, `domain`, `domain__name`, `expiryDate`, `fuses`, `owner`, `owner__id`, `name` |
| `Resolver` | `id`, `domain`, `domain__name`, `address`, `addr`, `addr__id`, `contentHash`, `texts`, `coinTypes`, `events` |
| `Account` | `id`, `domains`, `wrappedDomains`, `registrations` |
| event interfaces | `id`, parent relation, parent relation fields, `blockNumber`, `transactionID` |
| concrete events | shared event fields plus event-specific fields such as `owner__id`, `expiryDate`, `key`, `value`, `fuses` |

For implementation, start with scalar and FK order fields that real clients use. Add relationship ordering by mapping enum variants to explicit SQL join clauses. Do not build order SQL from raw strings.

### Derived Joins

The official schema uses `@derivedFrom` heavily. In Rust these become resolver-time queries:

| GraphQL field | SQL lookup |
| --- | --- |
| `Domain.subdomains` | `domains.parent_id = domain.id` |
| `Domain.registration` | `registrations.domain_id = domain.id` |
| `Domain.wrappedDomain` | `wrapped_domains.domain_id = domain.id` |
| `Domain.events` | union of concrete domain event tables by `domain_id` |
| `Registration.events` | union of concrete registration event tables by `registration_id` |
| `Resolver.events` | union of concrete resolver event tables by `resolver_id` |
| `Account.domains` | `domains.owner_id = account.id` |
| `Account.wrappedDomains` | `wrapped_domains.owner_id = account.id` |
| `Account.registrations` | `registrations.registrant_id = account.id` |

Use `async-graphql` `DataLoader` or batched repository methods for derived fields. Without batching, common queries like `domains(first: 100) { id owner { id } resolver { id } }` will become N+1 query patterns.
