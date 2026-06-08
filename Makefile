.PHONY: db-up db-down db-reset db-logs docker-build docker-run migrate serve sandbox status reset backfill archive-only archive-backfill raw-backfill archive-status archive-status-verify labels-import labels-heal benchmark benchmark-all test lint check

RAW_ARCHIVE_DIR ?= .raw-archive
IMAGE ?= ensindexer:local
ORBSTACK_BIN ?= /Applications/OrbStack.app/Contents/MacOS/xbin
DOCKER_DESKTOP_BIN ?= /Applications/Docker.app/Contents/Resources/bin
DOCKER_BIN ?= $(shell if [ -x "$(ORBSTACK_BIN)/docker" ]; then printf "$(ORBSTACK_BIN)"; elif command -v docker >/dev/null 2>&1; then dirname "$$(command -v docker)"; else printf "$(DOCKER_DESKTOP_BIN)"; fi)
DOCKER ?= $(DOCKER_BIN)/docker
DOCKER_ENV = PATH="$(DOCKER_BIN):$(PATH)"

db-up:
	$(DOCKER_ENV) $(DOCKER) compose up -d postgres

db-down:
	$(DOCKER_ENV) $(DOCKER) compose down

db-reset:
	$(DOCKER_ENV) $(DOCKER) compose down -v
	$(MAKE) db-up

db-logs:
	$(DOCKER_ENV) $(DOCKER) compose logs -f postgres

docker-build:
	$(DOCKER_ENV) $(DOCKER) build -t $(IMAGE) .

docker-run:
	$(DOCKER_ENV) $(DOCKER) run --rm --env-file .env -p 8080:8080 $(IMAGE)

migrate:
	cargo run -p cli -- migrate

serve:
	cargo run -p cli -- serve

sandbox: serve

status:
	cargo run -p cli -- status

reset:
	cargo run -p cli -- reset --yes

backfill:
	cargo run -p cli -- backfill

archive-only:
	RAW_ARCHIVE_DIR=$(RAW_ARCHIVE_DIR) cargo run -p cli -- archive

archive-backfill:
	ARCHIVE_BACKFILLS=true RAW_ARCHIVE_DIR=$(RAW_ARCHIVE_DIR) cargo run -p cli -- backfill

raw-backfill:
	BACKFILL_SOURCE=raw RAW_ARCHIVE_DIR=$(RAW_ARCHIVE_DIR) cargo run -p cli -- backfill

archive-status:
	RAW_ARCHIVE_DIR=$(RAW_ARCHIVE_DIR) cargo run -p cli -- archive-status

archive-status-verify:
	RAW_ARCHIVE_DIR=$(RAW_ARCHIVE_DIR) cargo run -p cli -- archive-status --verify

labels-heal:
	cargo run -p cli -- labels-heal --limit $${LABEL_HEAL_LIMIT:-1000}

labels-import:
	cargo run -p cli -- labels-import --input "$${LABELS_FILE:?set LABELS_FILE to a .ensrainbow or TSV label file}"

benchmark:
	cargo run -p cli -- benchmark --output target/benchmark-local.json

benchmark-all:
	cargo run -p cli -- benchmark --local-url http://127.0.0.1:8080/subgraph --official-url "$${SUBGRAPH_URL:-}" --official-auth-token "$${SUBGRAPH_AUTH_TOKEN:-}" --ensnode-url "$${ENSNODE_SUBGRAPH_URL:-}" --ensnode-auth-token "$${ENSNODE_SUBGRAPH_AUTH_TOKEN:-}" --output target/benchmark-all.json

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

check:
	cargo fmt --all -- --check
	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings
