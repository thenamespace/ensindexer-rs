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
- `provider_ms`: only populated when an endpoint exposes execution timing in GraphQL `extensions`. If it is absent, client-side benchmarking cannot remove network latency from hosted providers.

For strict compute-only comparisons between all three systems, run each implementation locally or in the same network environment and compare their provider-reported or in-process execution timings. Hosted The Graph and hosted ENSNode wall time is useful operational data, but it is not pure server compute time.

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

## Query Coverage

| Query | Purpose | Source Pattern |
| --- | --- | --- |
| `01-domain-batch` | Batch exact domain reads with owner, registrant, resolver, registration, and wrapped-domain relationships. | Common app profile/detail pages |
| `02-names-for-address` | ENSJS `getNamesForAddress`-style owner/registrant/wrapped/resolved address lookup with expiry filtering. | ENSJS subgraph API |
| `03-eth-subnames` | High-cardinality `eth` subdomain traversal. | ENSJS `getSubnames` and ENSNode `Domain.subdomains` index work |
| `04-subnames-search` | Subdomain traversal with label search. | ENSJS search-string behavior |
| `05-decoded-label` | `labelhash -> labelName` decoded-name lookup. | ENSJS `getDecodedName` |
| `06-resolver-records` | Resolver relationship and recent resolver event hydration. | ENSJS `getSubgraphRecords`-adjacent workload |
| `07-registrations` | Registration lookup by label and expiry with nested domain/events. | Registrar dashboards and expiry tools |
| `08-name-history` | Domain event history with interface fragments. | ENSJS `getNameHistory` |
| `09-event-scan` | Recent domain, registration, and resolver event-interface scans. | Indexer explorer/admin workloads |
| `10-relationship-filter` | Trailing-underscore relationship filters over owner and resolver. | Graph Node compatibility and generated filters |
| `11-text-search` | Fuzzy/nocase name search. | ENSNode trigram-index workload |

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

## Current Local Baseline

Smoke run on the full local mainnet database at block `25270169`:

```bash
cargo run -p cli -- benchmark --iterations 2 --warmup 1 --output target/benchmark-smoke.json
```

| Query | Local Compute Median |
| --- | ---: |
| `01-domain-batch` | 12.307ms |
| `02-names-for-address` | 24.933ms |
| `03-eth-subnames` | 41.456ms |
| `04-subnames-search` | 163.359ms |
| `05-decoded-label` | 1.703ms |
| `06-resolver-records` | 16.083ms |
| `07-registrations` | 12.169ms |
| `08-name-history` | 7.550ms |
| `09-event-scan` | 45.371ms |
| `10-relationship-filter` | 8.839ms |
| `11-text-search` | 198.007ms |

The two remaining slow local-compute cases are intentionally broad substring searches. Their current GIN trigram plans find matches quickly but still fetch and sort tens of thousands of candidate rows. They need a search-specific optimization rather than another raw btree index over unbounded ENS labels.

## Adding Queries

Add a new pair:

```text
benchmarks/queries/12-some-workload.graphql
benchmarks/queries/12-some-workload.variables.json
```

Then run `make benchmark`. The CLI discovers all `.graphql` files in lexical order.
