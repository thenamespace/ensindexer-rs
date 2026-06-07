# server

HTTP server crate for the ENS indexer API.

## Responsibility

`server` exposes the GraphQL endpoint and operational health checks through Axum and Tower middleware. It can also run the ingest service in the same process when configured.

## Modules

- `http`: router construction, GraphQL request handling, Apollo Sandbox HTML, health checks, and middleware stack.
- `runtime`: process orchestration for HTTP-only and unified HTTP+indexer modes.

## Architecture Notes

The server receives a preconfigured `Storage` handle and builds the `api` schema from it. `/graphql` accepts POST GraphQL requests and always serves Apollo Sandbox on GET in both dev and prod. `/healthz` is a lightweight liveness check, and `/readyz` verifies database connectivity. Tower layers provide compression, permissive CORS for local/API clients, timeout handling, and request tracing.

`ENABLE_BACKFILL=true` starts a startup backfill beside the HTTP server, and `ENABLE_LIVE_INDEXING=true` starts the live confirmed-block loop. If both toggles are enabled, the process runs the configured backfill first and then enters live indexing. `BACKFILL_SOURCE=rpc|hypersync|raw` controls startup backfill and CLI backfill behavior. `INDEXING_SOURCE=http_rpc|wss` controls live provider selection. If the background indexer returns an error, the unified service exits instead of continuing to serve stale data silently.

## Boundary Rules

- This crate owns HTTP transport, middleware, and process-level task orchestration.
- This crate should not contain GraphQL entity definitions, SQL, projection handlers, or RPC ingestion logic.
- Operational endpoints should stay cheap and predictable so they can be used by containers and load balancers.
- Apollo Sandbox is always available on `GET /graphql`.

## Testing Approach

Use router tests for health endpoints, GraphQL POST routing, Sandbox HTML behavior, and readiness failures. End-to-end API tests should live at the workspace level when they need seeded database state.
