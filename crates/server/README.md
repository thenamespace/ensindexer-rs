# server

HTTP server crate for the ENS indexer API.

## Responsibility

`server` exposes the GraphQL endpoint and operational health checks through Axum and Tower middleware.

## Modules

- `http`: router construction, GraphQL request handling, Apollo Sandbox HTML, health checks, and middleware stack.

## Architecture Notes

The server receives a preconfigured `Storage` handle and builds the `api` schema from it. `/graphql` accepts POST GraphQL requests and serves Apollo Sandbox on GET when enabled. `/healthz` is a lightweight liveness check, and `/readyz` verifies database connectivity. Tower layers provide compression, permissive CORS for local/API clients, timeout handling, and request tracing.

## Boundary Rules

- This crate owns HTTP transport and middleware only.
- This crate should not contain GraphQL entity definitions, SQL, projection handlers, or RPC ingestion logic.
- Operational endpoints should stay cheap and predictable so they can be used by containers and load balancers.
- UI helpers such as Apollo Sandbox should be optional and controlled by configuration.

## Testing Approach

Use router tests for health endpoints, GraphQL POST routing, disabled/enabled Sandbox behavior, and readiness failures. End-to-end API tests should live at the workspace level when they need seeded database state.
