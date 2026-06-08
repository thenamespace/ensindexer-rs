# ENS Indexer

Custom Rust ENS indexer intended to be a drop-in replacement for the current ENS subgraph schema and GraphQL query shape.

The implementation plan and official-subgraph research live in [docs/README.md](docs/README.md).

## Workspace

- `crates/types`: shared IDs, constants, log context, scalar helpers.
- `crates/contracts`: Alloy event bindings and decoded ENS event enum.
- `crates/config`: `.env` based runtime configuration.
- `crates/storage`: SQLx pool, migrations, repository/query foundations.
- `crates/projection`: deterministic projection dispatcher and handler modules.
- `crates/ingest`: backfill, archive, replay, and live indexing service.
- `crates/api`: async-graphql schema and resolvers.
- `crates/server`: Axum HTTP server.
- `crates/cli`: operational CLI entrypoint.

## Setup

```bash
cp .env.example .env
make db-up
make migrate
make serve
```

Configuration is loaded from `.env` via `config`.
Open [http://127.0.0.1:8080/graphql](http://127.0.0.1:8080/graphql) in a browser for Apollo Sandbox. The Sandbox is always available in dev and prod.
`make serve` starts the GraphQL API. Set `ENABLE_BACKFILL=true` to run a startup catchup backfill in the same process, and set `ENABLE_LIVE_INDEXING=true` to keep indexing confirmed live ranges after startup. If both toggles are enabled, startup backfill runs before live indexing.

`BACKFILL_SOURCE` is strict: `rpc`, `hypersync`, or `raw`. There is no automatic transport selection and there are no `BACKFILL_FROM` or `BACKFILL_TO` controls. RPC and HyperSync backfills resume from database checkpoints, archive-only resumes after the last archived range, and raw replay uses the archive bounds plus database checkpoints.
Set `ARCHIVE_BACKFILLS=true` and `RAW_ARCHIVE_DIR=.raw-archive` to persist fetched raw logs and block metadata as binary `.bin` range files. A first run can use `BACKFILL_SOURCE=hypersync` plus archiving; a later fresh database can use `BACKFILL_SOURCE=raw` to replay those archived files without RPC or HyperSync credits. `INDEXING_SOURCE` controls live indexing and must be `http_rpc` or `wss`; `wss` requires `ETH_WS_URL`.

Indexer commands:

```bash
make status
cargo run -p cli -- backfill
cargo run -p cli -- archive
cargo run -p cli -- replay
cargo run -p cli -- index
make labels-import
make labels-heal
make reset
make check
```

`scripts/ens-heal.sh` can download a local ENSRainbow dataset for offline use. The indexer does not call ENSRainbow APIs at runtime: `make labels-import` loads a local ENSRainbow streamed protobuf `.ensrainbow` file or TSV `labelhash<TAB>label` file into Postgres, and `make labels-heal` repairs unknown `Domain.labelName` values from that local dictionary without external API calls, database reset, or replay. Use `LABELS_FILE` for import and `LABEL_HEAL_LIMIT` for one repair batch.

Local label healing workflow:

```bash
# Download and verify ENSRainbow locally, then extract healed-names/ens_names.tsv.
./scripts/ens-heal.sh

# Import the local TSV into label_preimages.
LABELS_FILE=healed-names/ens_names.tsv make labels-import

# Repair already-indexed domain labels/names from imported preimages.
LABEL_HEAL_LIMIT=100000 make labels-heal
```

For the cleanest first full backfill, import labels before replay/backfill so projection can resolve known labelhashes as rows are created. If the database is already backfilled, import labels and run `labels-heal` after the backfill finishes. Avoid running large heal batches concurrently with dense backfill ranges because both compete for Postgres write and index IO.

Archive workflow for repeatable projection testing:

```bash
# Fetch once from BACKFILL_SOURCE=rpc or BACKFILL_SOURCE=hypersync and save binary archive ranges
# without applying projection writes to Postgres.
BACKFILL_SOURCE=hypersync RAW_ARCHIVE_DIR=.raw-archive-full make archive-only

# Or fetch and apply in one pass when you want both archive and database state.
BACKFILL_SOURCE=hypersync RAW_ARCHIVE_DIR=.raw-archive-full make archive-backfill

# Rebuild a fresh dev database without spending RPC/HyperSync credits again.
make db-reset
make migrate
RAW_ARCHIVE_DIR=.raw-archive-full make archive-status
BACKFILL_SOURCE=raw RAW_ARCHIVE_DIR=.raw-archive-full make raw-backfill
```

`make db-reset` deletes the local Postgres compose volume. Use it only for disposable development databases.
`make archive-status` reads the manifest only, so it stays fast for large archives. Use `make archive-status-verify` when you explicitly want the slower checksum pass across range files.
For a complete archive-only run, start with an empty archive directory. The service starts at the first ENS source deployment block, writes `resolvers.json` next to `manifest.json`, and updates it after each completed range so later archive-only resumes can reload discovered resolver addresses. After the archive is complete, use `BACKFILL_SOURCE=raw` or `make raw-backfill` to project from those `.bin` files.

Postgres runs through `compose.yml` using `postgres:17`. The default compose credentials match `.env.example`.

## Docker

Build the unified service image:

```bash
make docker-build
```

Run the API from the image:

```bash
make docker-run
```

The container entrypoint runs `ensindexer serve`. Use `ENABLE_BACKFILL` and `ENABLE_LIVE_INDEXING` in `.env` to run startup backfill and live indexing inside the same process as the GraphQL API.

## Code Layout

Crates use small entrypoint files and implementation modules instead of keeping all logic in one `lib.rs`:

- library crates expose `src/lib.rs` plus focused domain modules such as `src/schema.rs`, `src/service.rs`, or `src/repositories/*.rs`;
- the CLI keeps `src/main.rs` as the binary entrypoint and `src/app.rs` for command execution;
- larger modules should be split further by ENS domain area as functionality grows.
