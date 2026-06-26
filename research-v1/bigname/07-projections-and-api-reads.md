# 07 Projections And API Reads

Bigname projections are current read models.

They are disposable and rebuildable. Normalized events and identity rows are the source of truth.

## Projection Rules

- Projection workers own projection tables.
- Adapters never write projections.
- API handlers never rebuild from raw facts.
- Every projection row carries provenance, manifest version, and chain-position context.
- A reader must fail closed when selected positions cannot be served.
- Projections consume canonical/safe/finalized normalized events by default.
- Observed/orphaned rows are audit/history inputs, not normal current state.

## Projection Families

| Projection | Key | Primary Route |
| --- | --- | --- |
| `name_current` | `logical_name_id` | exact-name lookup |
| `address_names_current` | `(address, logical_name_id, relation)` | address-to-names |
| `children_current` | `(parent_logical_name_id, child_logical_name_id, surface_class)` | children |
| `permissions_current` | `(resource_id, subject, scope)` | roles/permissions |
| `resolver_current` | `(chain_id, resolver_address)` | resolver overview |
| `record_inventory_current` | `(resource_id, record_version_boundary_key)` | declared record inventory |
| `primary_names_current` | `(address, coin_type, namespace)` | primary-name claim anchor |

`surface_bindings` is history/identity, not a `_current` projection.

## Projection Change Feed

When `normalized_events` are inserted or their canonicality changes, storage writes:

```text
projection_normalized_event_changes
```

Projection apply workers consume that feed and write:

```text
projection_invalidations
```

`projection_apply_cursors` stores consumed change watermarks.

## Invalidation Queue

`projection_invalidations` is keyed by:

```text
projection
projection_key
```

New invalidation for the same key increments `generation` and returns state to `pending`.

Workers claim pending rows, rebuild the key, then complete only the generation they claimed. This prevents a slow worker from deleting a newer invalidation.

After repeated deterministic failures, the row moves to:

```text
projection_invalidation_dead_letters
```

Dead letters are operator-visible but no longer block live projection lag.

## Apply Order

Bigname applies projections in dependency order:

```text
name_current
children_current
permissions_current
record_inventory_current
resolver_current
address_names_current
primary_names_current
```

`resolver_current` follows `permissions_current` because resolver overview can summarize resolver-scoped permissions.

## `name_current`

Exact-name profile root.

Reads from:

- active `name_surfaces`;
- active `surface_bindings`;
- resources;
- token lineages;
- normalized events;
- permissions;
- resolver facts;
- record inventory summaries.

Contains:

- logical name id;
- namespace/name/display;
- current binding;
- authority/control summary;
- registration status and expiry;
- resolver summary;
- record summaries;
- history head pointers;
- coverage/provenance/chain positions.

ENSv1 reverse `NameChanged` can supply a preimage, but never creates authority/resolver truth.

Basenames exact-name truth comes from Base registry/registrar/resolver families, not primary claim intake or L1 compatibility.

## `address_names_current`

Address-to-name relation rows.

Default unit is public surface, not resource.

Relations include:

```text
registrant
token_holder
effective_controller
```

Some app-facing routes can dedupe by resource, but default truth model is surface relation.

Address filters such as owner/registrant/account read this table first, then join to `name_current`.

## `children_current`

Declared direct children.

Sources:

- ENSv1 registry child edges;
- Basenames registry child edges;
- ENSv2 `SubregistryChanged` and `ParentChanged`;
- alias/wildcard buckets where supported.

Bigname supports unknown-label children.

If registry event has only labelhash, child row can be:

```text
[<labelhash>].parent.eth
```

When `label_preimages` later learns the label, storage invalidates affected children rows and rebuild replaces placeholders with readable names.

Important:

- unknown-label row does not mint exact `name_surfaces`;
- label preimage does not create authority;
- direct-child counts use declared direct-child bucket.

## `permissions_current`

Resource-keyed permissions.

Key:

```text
resource_id
subject
scope
```

Sources:

- ENSv1 registry ownership;
- ENSv1 registrar/wrapper authority;
- ENSv1 wrapper fuses;
- ENSv2 EAC roles;
- resolver-scoped permission events.

Wrapper fuses mask effective powers. If `CANNOT_SET_RESOLVER` is burned, resolver mutation power is not published.

ENSv2 root roles stay distinguishable from resource-specific roles because root fallback can satisfy resource-level checks.

## `record_inventory_current`

Declared resolver record inventory.

Key:

```text
resource_id
record_version_boundary_key
```

It stores selector inventory and observed/cache values by record boundary.

Input events:

- `RecordChanged`;
- `RecordVersionChanged`;
- resolver changes;
- record tenure boundaries.

Record inventory may need earlier resolver events from prior resources of the same logical name to bound current resolver tenure. Cross-resource resolver rows are boundaries, not current record replacement.

## `resolver_current`

Resolver overview keyed by:

```text
chain_id
resolver_address
```

Inputs:

- resolver discovery;
- resolver record events;
- resolver permissions;
- alias/wildcard facts;
- profile admission.

Unknown dynamic resolvers remain pending/unsupported until profile admission proves support.

## `primary_names_current`

Claim-side primary-name projection.

Key:

```text
address
coin_type
namespace
```

It stores declared claim state, not verified result payload.

Claim statuses include:

```text
not_found
success/claimed
invalid_name
unsupported
stale
```

Verified primary readback comes from execution outcomes. `primary_names_current` is the claim anchor and invalidation hook.

## API Read Pattern

Exact-name route:

```text
normalize request
resolve namespace
select snapshot positions
read name_current
join records/permissions/resolver/primary/execution as requested
return coverage/provenance
```

Address route:

```text
normalize address
read address_names_current by address/relation
join name_current for display/readability
use sidecars for counts/feed if route is compact identity facade
```

Children route:

```text
read children_current by parent and surface_class
join name_current when child exact surface exists
return unknown-label placeholder when no readable label exists
```

History route:

```text
read canonical normalized_events
filter by surface/resource/address anchors
keyset paginate by chain position
```

No route falls back to raw logs.

