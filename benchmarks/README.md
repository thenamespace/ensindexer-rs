# ENS Indexer Benchmarks

This folder contains repeatable GraphQL benchmark fixtures for comparing:

- this Rust indexer;
- the official ENS subgraph on The Graph;
- an ENSNode subgraph-compatible endpoint.

The query files live in [queries](queries). Each `*.graphql` file may have a sibling `*.variables.json` file with the same base name.

## Timing Model

The Rust example supports two simple timing modes.

- `TimingMode::Raw`: raw request wall time. This includes local/server compute, gateway work, TLS, network latency, and response transfer.
- `TimingMode::ComputeOnly`: raw request wall time minus the endpoint's lightweight `_meta` request median, floored at zero. This approximates compute/server-side time by subtracting the measured request/network floor.

`ComputeOnly` is still an approximation for hosted providers because the `_meta` request may not follow exactly the same backend path as every query. It is enough for practical comparison when the goal is to remove the obvious network roundtrip floor from Cloudflare/GCP-hosted endpoints.

## Running The Fixtures

The production `ensindexer` binary intentionally does not include a benchmark command. Keep benchmarking as external tooling so the public CLI stays focused on operating the indexer.

```bash
cargo make start
```

The production service exposes `/subgraph`. The Rust benchmark example loads each `benchmarks/queries/*.graphql` file, pairs it with the sibling `*.variables.json` file when present, and POSTs it to every configured endpoint. Providers for the same operation run concurrently, so local, ENSNode, and The Graph requests do not wait on each other. Use the same files against hosted The Graph and ENSNode endpoints when the schema supports the query. Mark endpoint-specific schema gaps as `unsupported` rather than changing the canonical local fixture.

Endpoint and run configuration is intentionally simple Rust code in [`../crates/cli/examples/benchmark.rs`](../crates/cli/examples/benchmark.rs). Edit these constants before a run:

```rust
const WARMUPS: usize = 5;
const ITERATIONS: usize = 25;
const TIMEOUT_MS: u64 = 30_000;
const USE_COMPUTE_ONLY_TIMING: bool = true;

const ENTRIES: &[BenchmarkEntry] = &[
    BenchmarkEntry {
        name: "ensindexer-rs",
        url: "http://127.0.0.1:8080/subgraph",
        auth_bearer_env: None,
        overrides: &[],
    },
    BenchmarkEntry {
        name: "ensnode",
        url: "https://api.alpha.ensnode.io/subgraph",
        auth_bearer_env: None,
        overrides: ENSNODE_OVERRIDES,
    },
    BenchmarkEntry {
        name: "the graph indexer",
        url: "https://gateway.thegraph.com/api/subgraphs/id/5XqPmWe6gjyrJtFn9cLy237i4cWw2j9HcUJEXsP5qGtH",
        auth_bearer_env: Some("SUBGRAPH_AUTH_TOKEN"),
        overrides: &[],
    },
];
```

Timing modes:

| mode                      | meaning                                                                                                                                                                                                                  |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `TimingMode::Raw`         | Raw request time, including local/server compute, network, TLS, gateway, and response transfer.                                                                                                                          |
| `TimingMode::ComputeOnly` | Raw request time minus that endpoint's lightweight `_meta` request median. This removes the measured request/network floor as a practical approximation, but it is not a true database-only timing for hosted providers. |

```bash
cargo make benchmark
```

The example loads `.env` before reading environment values, so `SUBGRAPH_AUTH_TOKEN` can live in the normal project `.env`. Every completed run writes a fresh root-level `BENCHMARK.md` report.

For endpoint-specific schema differences, add simple query rewrite rules to the entry. ENSNode currently rewrites common nocase string filters:

```rust
const ENSNODE_OVERRIDES: &[QueryOverride] = &[
    QueryOverride { from: "$expiry: String!", to: "$expiry: BigInt!" },
    QueryOverride { from: "$ethNode: ID!", to: "$ethNode: String!" },
    QueryOverride { from: "_contains_nocase", to: "_contains" },
    QueryOverride { from: "_starts_with_nocase", to: "_starts_with" },
    QueryOverride { from: "_ends_with_nocase", to: "_ends_with" },
];
```

To debug one provider without running the full matrix:

```bash
cargo run -p cli --release --example benchmark -- --only ensnode
```

Detailed query errors are printed to stderr during the run. The generated `BENCHMARK.md` keeps result cells short: `unsupported`, `error`, or `timeout`.

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

## Adding Queries

Add a new pair:

```text
benchmarks/queries/12-some-workload.graphql
benchmarks/queries/12-some-workload.variables.json
```

Run them with `cargo make benchmark`. Query files are discovered in lexical order, and each run records warmup count, iteration count, raw wall time, and compute-only timing when that mode is selected. The generated report is written to root `BENCHMARK.md`.
