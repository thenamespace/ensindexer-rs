# ENS Indexer Benchmarks

This folder contains repeatable GraphQL benchmark fixtures for comparing:

- this Rust indexer;
- the official ENS subgraph on The Graph;
- an ENSNode subgraph-compatible endpoint.

The query files live in [queries](queries). Each `*.graphql` file may have a sibling `*.variables.json` file with the same base name.

## Timing Model

Use compute time where it is actually observable.

- `local-compute.compute_ms`: in-process execution through the Rust `async-graphql` schema with a real Postgres connection. This avoids HTTP, loopback, and external network latency.
- `local-http.wall_ms`: optional localhost HTTP timing through `/subgraph`. This includes local Axum, JSON, and loopback overhead.
- `official.wall_ms` and `ensnode.wall_ms`: endpoint wall-clock timing. This includes internet latency unless the endpoint is local.
- `baseline_wall_ms`: a lightweight `_meta` GraphQL request to the same endpoint. This is the measured provider/network floor for that run.
- `baseline_adjusted_ms`: endpoint wall time minus the endpoint's baseline median, floored at zero. Use this for hosted ENSNode and The Graph when provider execution timing is not available.
- `provider_ms`: only populated when an endpoint exposes execution timing in GraphQL `extensions`. If it is absent, client-side benchmarking cannot remove network latency from hosted providers.

For strict compute-only comparisons between all three systems, run each implementation locally or in the same network environment and compare their provider-reported or in-process execution timings. Hosted The Graph and hosted ENSNode wall time is useful operational data, but it is not pure server compute time. For the comparison table below, use this precedence:

1. `local-compute.compute_ms` for `ensindexer-rs`.
2. `provider_ms` for `ensnode` or `the graph indexer` when the endpoint returns it.
3. `baseline_adjusted_ms` for hosted endpoints when provider timing is absent.

## Running The Fixtures

The production `ensindexer` binary intentionally does not include a benchmark command. Keep benchmarking as external tooling so the public CLI stays focused on operating the indexer.

```bash
cargo make start
```

The production service exposes `/subgraph`, so a benchmark runner can load each `benchmarks/queries/*.graphql` file, pair it with the sibling `*.variables.json` file when present, and POST it to `http://127.0.0.1:8080/subgraph`. Use the same files against hosted The Graph and ENSNode endpoints when the schema supports the query. Mark endpoint-specific schema gaps as `unsupported` rather than changing the canonical local fixture.

## Query Coverage

| Query                    | Purpose                                                                                                    | Source Pattern                                                 |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `01-domain-batch`        | Batch exact domain reads with owner, registrant, resolver, registration, and wrapped-domain relationships. | Common app profile/detail pages                                |
| `02-names-for-address`   | ENSJS `getNamesForAddress`-style owner/registrant/wrapped/resolved address lookup with expiry filtering.   | ENSJS subgraph API                                             |
| `03-eth-subnames`        | High-cardinality `eth` subdomain traversal.                                                                | ENSJS `getSubnames` and ENSNode `Domain.subdomains` index work |
| `04-subnames-search`     | Subdomain traversal with label search.                                                                     | ENSJS search-string behavior                                   |
| `05-decoded-label`       | `labelhash -> labelName` decoded-name lookup.                                                              | ENSJS `getDecodedName`                                         |
| `06-resolver-records`    | Resolver relationship and recent resolver event hydration.                                                 | ENSJS `getSubgraphRecords`-adjacent workload                   |
| `07-registrations`       | Registration lookup by label and expiry with nested domain/events.                                         | Registrar dashboards and expiry tools                          |
| `08-name-history`        | Domain event history with interface fragments.                                                             | ENSJS `getNameHistory`                                         |
| `09-event-scan`          | Recent domain, registration, and resolver event-interface scans.                                           | Indexer explorer/admin workloads                               |
| `10-relationship-filter` | Trailing-underscore relationship filters over owner and resolver.                                          | Graph Node compatibility and generated filters                 |
| `11-text-search`         | Fuzzy/nocase name search.                                                                                  | ENSNode trigram-index workload                                 |

## Why These Queries

ENSJS primarily stresses:

- `domains(where: { and: [{ or: owner/registrant/wrappedOwner/resolvedAddress }, expiry filters] })`;
- `domain(id).subdomains(...)`;
- `domains(where: { labelhash, labelName_not: null })`;
- `domain.events` with interface fragments;
- resolver/registration hydration from domain results.

ENSNode's changelog and schema work highlight the same production risks:

- exact and fuzzy name lookup cannot use unsafe plain btree indexes on arbitrary-length on-chain strings;
- `Domain.subdomains` needs parent traversal indexes;
- generated subgraph-style relationship filters and derived event lists need compound parent/sort indexes.

The Rust indexer should beat hosted endpoints by a clear margin on local compute. Hosted endpoint wall-clock numbers are included only as operational context unless provider execution timing is available.

## Current Production Benchmark

Release run on the full local mainnet database at block `25270169`, plus hosted The Graph and ENSNode comparisons with baseline-adjusted timings. The source queries for the run are the files in `benchmarks/queries`.

| operation                | ensindexer-rs |     ensnode | the graph indexer |
| ------------------------ | ------------: | ----------: | ----------------: |
| `01-domain-batch`        | 5.985ms (13.5x faster than slowest, 92.6% lower) | 12.836ms (6.3x faster than slowest, 84.1% lower) | 80.771ms (slowest) |
| `02-names-for-address`   | 18.722ms (10.8x faster than slowest, 90.7% lower) | 13.398ms (15.0x faster than slowest, 93.4% lower) | 201.511ms (slowest) |
| `03-eth-subnames`        | 31.281ms (60.9x faster than slowest, 98.4% lower) | 1903.604ms (slowest) | 234.277ms (8.1x faster than slowest, 87.7% lower) |
| `04-subnames-search`     | 161.328ms (10.7x faster than slowest, 90.6% lower) | 1724.506ms (slowest) | 504 timeout |
| `05-decoded-label`       | 1.843ms (54.0x faster than slowest, 98.1% lower) | 64.406ms (1.5x faster than slowest, 35.3% lower) | 99.493ms (slowest) |
| `06-resolver-records`    | 10.922ms (19.9x faster than slowest, 95.0% lower) | 94.708ms (2.3x faster than slowest, 56.3% lower) | 216.811ms (slowest) |
| `07-registrations`       | 9.260ms (116.6x faster than slowest, 99.1% lower) | 1079.505ms (slowest) | 233.749ms (4.6x faster than slowest, 78.3% lower) |
| `08-name-history`        | 5.777ms (16.5x faster than slowest, 93.9% lower) | 11.905ms (near-baseline/noisy; 8.0x faster than slowest, 87.5% lower) | 95.292ms (slowest) |
| `09-event-scan`          | 28.043ms (232.0x faster than slowest, 99.6% lower) | unsupported | 6506.291ms (slowest) |
| `10-relationship-filter` | 6.828ms (34.1x faster than slowest, 97.1% lower) | unsupported | 233.165ms (slowest) |
| `11-text-search`         | 172.538ms (3.7x faster than slowest, 73.2% lower) | 642.963ms (slowest) | 144.134ms (4.5x faster than slowest, 77.6% lower) |

Relative speed is calculated against the slowest supported numeric result in each row. Timeout and unsupported cells are excluded from the numeric baseline.

The hosted The Graph column uses `baseline_adjusted_ms.median` because the gateway response did not expose provider execution timing. Raw hosted `wall_ms.median` was about 381ms higher per query in this release run, matching the measured `_meta` baseline median. The hosted ENSNode column also uses `baseline_adjusted_ms.median`; its measured `_meta` baseline median was about 361ms in the full run. `07-registrations` is from an immediate single-query ENSNode retry after the full hosted run hit a transient send failure for that one operation. `08-name-history` was rerun again with 25 baseline samples and 25 query samples after a suspicious near-1ms adjusted result; the rerun measured a raw wall median of 656.481ms, baseline median of 644.576ms, and adjusted median of 11.905ms. That value is still close enough to the hosted timing floor that it should be read as baseline-subtraction noise, not precise provider compute.

ENSNode is not treated as the schema source of truth here. The historical benchmark run applied ENSNode-only compatibility rewrites for its public alpha endpoint: `expiry: String!` was sent as `BigInt!`, `ID!` was sent as `String!`, and `*_contains_nocase` filters were downgraded to case-sensitive `*_contains`. ENSNode still does not support the top-level event collections in `09-event-scan` or the generated trailing-underscore relationship filters in `10-relationship-filter`, so those cells are marked `unsupported`.

The two remaining slow local-compute cases are intentionally broad substring searches. Their current GIN trigram plans find matches quickly but still fetch and sort tens of thousands of candidate rows. They need a search-specific optimization rather than another raw btree index over unbounded ENS labels.

After adding API DataLoader batching for hot `Domain` account/resolver relationships, local release slices improved:

| operation | before | after |
| --- | ---: | ---: |
| `04-subnames-search` | 161.328ms | 137.814ms |
| `11-text-search` | 172.538ms | 165.903ms |

This optimization reduces nested relationship roundtrips. It does not remove the main broad-search cost, which is still matching and sorting tens of thousands of domain rows for common substrings such as `art`.

After expanding the same DataLoader layer to `Domain.registration` and `Domain.wrappedDomain`, local release slices measured:

| operation | production baseline | latest slice |
| --- | ---: | ---: |
| `01-domain-batch` | 5.985ms | 5.143ms |
| `02-names-for-address` | 18.722ms | 5.366ms |
| `03-eth-subnames` | 31.281ms | 7.258ms |
| `04-subnames-search` | 161.328ms | 158.942ms |
| `11-text-search` | 172.538ms | 168.660ms |

The largest wins are relationship-heavy domain result sets that request registration or wrapped-domain fields for many rows.

## Adding Queries

Add a new pair:

```text
benchmarks/queries/12-some-workload.graphql
benchmarks/queries/12-some-workload.variables.json
```

Run them with an external benchmark runner. Query files should be discovered in lexical order, and each run should record warmup count, iteration count, raw wall time, provider-reported execution time when available, and baseline-adjusted timing for hosted endpoints.
