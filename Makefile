.PHONY: db-up db-down db-logs migrate serve sandbox status reset backfill test lint check

BACKFILL_FROM ?= 9380380
BACKFILL_TO ?= 9381380

db-up:
	docker compose up -d postgres

db-down:
	docker compose down

db-logs:
	docker compose logs -f postgres

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
