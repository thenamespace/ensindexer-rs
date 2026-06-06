# contracts

Alloy ABI and event decoding crate.

## Responsibility

`contracts` defines ENS event bindings and decodes raw Alloy logs into a unified `EnsEvent` enum consumed by the projection layer.

## Modules

- `abi`: `alloy::sol!` event declarations for registry, registrar, wrapper, controller, and resolver events.
- `events`: fixed-source and resolver wildcard log decoding.
- `model`: decoded event enum, source identifiers, and decode errors.

## Architecture Notes

This crate intentionally contains event-only ABI snippets instead of full contract wrappers. That keeps decoding explicit and lightweight while still using production Alloy event types and topic signatures. Resolver logs are decoded by topic without a fixed address filter, matching the official subgraph wildcard resolver source.

## Boundary Rules

- This crate owns ABI declarations, topic matching, and raw-log-to-event decoding.
- This crate should not project events into database state.
- This crate should not fetch logs from RPC; ingestion provides raw logs and metadata.
- Decoded event structs should preserve enough raw values for projection to reproduce official subgraph IDs and entity fields.

## Indexed Sources

Fixed-source decoding covers ENS registry, old registry, base registrar, ETH registrar controllers, name wrapper, and reverse registrar style contracts as the project adds them. Resolver event decoding is wildcard-style: any log with a supported resolver topic can be decoded even when the resolver address is dynamic.

## Testing Approach

Use topic-level unit tests for every ABI event signature and decode tests with captured mainnet logs. When adding a new event, add tests for both successful decode and ignored-topic behavior so unsupported logs do not accidentally become projection inputs.
