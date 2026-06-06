.PHONY: db-up db-down db-logs migrate serve sandbox status reset backfill test lint check

BACKFILL_FROM ?= 9380380
BACKFILL_TO ?= 9381380
DOCKER_DESKTOP_BIN ?= /Applications/Docker.app/Contents/Resources/bin
DOCKER ?= $(shell command -v docker 2>/dev/null || printf $(DOCKER_DESKTOP_BIN)/docker)
DOCKER_ENV = PATH="$(DOCKER_DESKTOP_BIN):$(PATH)"

db-up:
	$(DOCKER_ENV) $(DOCKER) compose up -d postgres

db-down:
	$(DOCKER_ENV) $(DOCKER) compose down

db-logs:
	$(DOCKER_ENV) $(DOCKER) compose logs -f postgres

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
	cargo run -p cli -- backfill --from $(BACKFILL_FROM) --to $(BACKFILL_TO)

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

check:
	cargo fmt --all -- --check
	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings
