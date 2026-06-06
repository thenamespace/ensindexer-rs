# projection

Deterministic ENS projection crate.

## Responsibility

`projection` converts decoded ENS events into storage mutations for mutable entities and immutable event tables.

## Modules

- `handlers/dispatcher`: top-level event dispatch.
- `handlers/registry`: current and old registry ownership, resolver, TTL, and subdomain behavior.
- `handlers/registrar`: base registrar and controller registration behavior.
- `handlers/wrapper`: name wrapper ownership, fuses, expiry, and transfer behavior.
- `handlers/resolver`: resolver record mutation and resolver event history.
- `support`: shared name, expiry, fuse, and ID helpers.
- `error`: projection error type.

## Architecture Notes

Projection handlers do not fetch RPC data and do not serve HTTP. They receive decoded events and write through storage repositories. This keeps projection deterministic and testable with synthetic events. Event IDs and entity IDs are shaped to match the official subgraph.

## Boundary Rules

- This crate owns ENS event semantics: how each decoded event mutates current entities and event history.
- This crate should not perform RPC, parse `.env`, expose HTTP, or build GraphQL objects.
- Every handler should be deterministic from `(event, previous storage state)` to storage mutations.
- If a handler needs derived names or IDs, add reusable helpers under `support` instead of duplicating formatting logic.

## Projection Strategy

Handlers mirror the official subgraph mappings:

- Registry events create/update domains, ownership, resolver links, TTL, and subdomain relationships.
- Registrar/controller events update registrations and preserve registration lifecycle history.
- Wrapper events update wrapped ownership, fuses, expiry, and wrapped-domain history.
- Resolver events update resolver records and append resolver event history.

The dispatcher is the only public event application entrypoint. That keeps event ordering and future transaction-level behavior centralized.

## Testing Approach

Projection tests should use synthetic decoded events and an isolated Postgres database. For each event family, assert both immutable event rows and current entity rows. Regression tests should include official-subgraph fixtures for edge cases such as unknown labels, wrapped expiries, resolver multicoin records, and ownership transfers.
