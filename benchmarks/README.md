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

## Running

Local compute-only benchmark against the current database:

```bash
make benchmark
```

Equivalent direct command:

```bash
cargo run -p cli -- benchmark \
  --query-dir benchmarks/queries \
  --iterations 20 \
  --warmup 3 \
  --output target/benchmark-local.json
```

Three-way benchmark:

```bash
SUBGRAPH_URL="https://gateway.thegraph.com/api/subgraphs/id/5XqPmWe6gjyrJtFn9cLy237i4cWw2j9HcUJEXsP5qGtH" \
SUBGRAPH_AUTH_TOKEN="<the-graph-token>" \
ENSNODE_SUBGRAPH_URL="https://api.mainnet.ensnode.io/subgraph" \
make benchmark-all
```

`make benchmark-all` also records localhost HTTP timing if the Rust server is running on `http://127.0.0.1:8080/subgraph`.

Endpoint-only run, useful when testing a provider with schema quirks:

```bash
cargo run -p cli -- benchmark \
  --local-compute false \
  --ensnode-url https://api.alpha.ensnode.io/subgraph \
  --iterations 3 \
  --warmup 1 \
  --output target/benchmark-ensnode.json
```

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

Release run on the full local mainnet database at block `25270169`, plus hosted The Graph and ENSNode comparisons with baseline-adjusted timings:

```bash
ENSNODE_SUBGRAPH_URL=https://api.alpha.ensnode.io/subgraph \
cargo run --release -p cli -- benchmark \
  --iterations 10 \
  --warmup 3 \
  --output target/benchmark-production.json
```

| operation                | ensindexer-rs |     ensnode | the graph indexer |
| ------------------------ | ------------: | ----------: | ----------------: |
| `01-domain-batch`        |       5.985ms |    12.836ms |          80.771ms |
| `02-names-for-address`   |      18.722ms |    13.398ms |         201.511ms |
| `03-eth-subnames`        |      31.281ms |  1903.604ms |         234.277ms |
| `04-subnames-search`     |     161.328ms |  1724.506ms |       504 timeout |
| `05-decoded-label`       |       1.843ms |    64.406ms |          99.493ms |
| `06-resolver-records`    |      10.922ms |    94.708ms |         216.811ms |
| `07-registrations`       |       9.260ms |  1079.505ms |         233.749ms |
| `08-name-history`        |       5.777ms |        <1ms |          95.292ms |
| `09-event-scan`          |      28.043ms | unsupported |        6506.291ms |
| `10-relationship-filter` |       6.828ms | unsupported |         233.165ms |
| `11-text-search`         |     172.538ms |   642.963ms |         144.134ms |

The hosted The Graph column uses `baseline_adjusted_ms.median` because the gateway response did not expose provider execution timing. Raw hosted `wall_ms.median` was about 381ms higher per query in this release run, matching the measured `_meta` baseline median. The hosted ENSNode column also uses `baseline_adjusted_ms.median`; its measured `_meta` baseline median was about 361ms in the full run. `07-registrations` is from an immediate single-query ENSNode retry after the full hosted run hit a transient send failure for that one operation.

ENSNode is not treated as the schema source of truth here. The benchmark runner applies ENSNode-only compatibility rewrites for its public alpha endpoint: `expiry: String!` is sent as `BigInt!`, `ID!` is sent as `String!`, and `*_contains_nocase` filters are downgraded to case-sensitive `*_contains`. ENSNode still does not support the top-level event collections in `09-event-scan` or the generated trailing-underscore relationship filters in `10-relationship-filter`, so those cells are marked `unsupported`.

The two remaining slow local-compute cases are intentionally broad substring searches. Their current GIN trigram plans find matches quickly but still fetch and sort tens of thousands of candidate rows. They need a search-specific optimization rather than another raw btree index over unbounded ENS labels.

## Adding Queries

Add a new pair:

```text
benchmarks/queries/12-some-workload.graphql
benchmarks/queries/12-some-workload.variables.json
```

Then run `make benchmark`. The CLI discovers all `.graphql` files in lexical order.
