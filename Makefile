.PHONY: db-up db-down db-reset db-logs docker-build docker-run migrate serve sandbox status reset backfill archive-only archive-backfill raw-backfill archive-status archive-status-verify labels-heal test lint check

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
	cargo run -p cli -- labels-heal --limit $${LABEL_HEAL_LIMIT:-1000} --concurrency $${LABEL_HEAL_CONCURRENCY:-16}

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

check:
	cargo fmt --all -- --check
	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings
