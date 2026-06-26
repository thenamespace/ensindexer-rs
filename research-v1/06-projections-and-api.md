# 06 Projections And API

Projections are current read models.

They exist because public APIs must be fast and predictable.

## Projection Rules

- Projections are rebuildable.
- Projections do not define truth.
- API handlers should mostly read projections.
- Projection workers consume normalized events and identity rows.
- Reorgs and repairs invalidate projection keys.
- Slow relationship queries should become explicit projections.

## Core Projections

### `name_current`

One row per known name.

Answers:

- exact lookup by name or node;
- owner/registrant/wrapped owner;
- resolver;
- expiry;
- current resource;
- current source provenance;
- display name.

### `name_hierarchy_current`

One row per current parent/child edge.

Answers:

- children of `base.eth`;
- parent of `alice.base.eth`;
- root/2LD/subname classification;
- subtree traversal.

This is a projection, not a strict identity constraint. A child can be known before the parent is complete.

### `address_names_current`

One row per address/name relation.

Relations:

- owner;
- registrant;
- wrapped owner;
- resolver-set address record;
- reverse/primary candidate;
- verified primary.

Answers:

- names owned by an address;
- names resolving to an address;
- primary name for an address;
- compare declared vs verified primary.

### `resolver_records_current`

One row per current resolver record key.

Examples:

- coin type address;
- text key;
- contenthash;
- ABI;
- interface;
- DNS record;
- name/reverse record.

Answers:

- records for a name;
- record inventory;
- resolver-profile coverage.

### `resolver_current`

One row per resolver instance.

Answers:

- resolver address support status;
- source family;
- implementation/code hash;
- profile;
- number of names using it.

### `permissions_current`

One row per effective permission.

Answers:

- who can manage a resource;
- ENS v2 roles;
- wrapper fuse-masked permissions;
- registrar/wrapper/registry control roles.

### `primary_names_current`

One row per address and coin type.

Stores:

- declared claim;
- verification status;
- verified name if proven;
- trace/outcome reference;
- invalidation provenance.

### `search_names_current`

Search-friendly read model.

Stores:

- normalized name;
- display name;
- label segments;
- searchable text;
- rank signals;
- ownership/resolution summary;
- active/expired/unknown status.

This avoids expensive text search over raw identity and event tables.

## API Shape

The storage model should support both:

- GraphQL/subgraph-compatible reads;
- HTTP/REST or internal JSON routes.

Do not shape storage around GraphQL resolver nesting. Shape storage around query access patterns.

Examples:

```graphql
domain(name: "alice.base.eth")
domains(where: { parent: "base.eth" })
domains(where: { owner: "0x..." })
resolverRecords(name: "alice.eth")
```

These should hit projections, not raw event scans.

## Cache

Add cache where invalidation is clear.

Good cache candidates:

- exact name lookup;
- children pages;
- address-to-names pages;
- resolver record bundles;
- verified resolution result;
- search pages.

Invalidation keys are known:

- name id;
- parent name id;
- address;
- resolver address;
- record key;
- source family;
- block/reorg dependency.

Expected impact:

- exact lookups can drop to sub-millisecond DB time or in-memory response;
- repeated children/search pages can avoid heavy sorting;
- verified resolution can avoid repeated block-pinned calls;
- cache will help public endpoints much more than one-off benchmark queries.

## What Not To Do

Do not:

- serve relationships by joining raw logs;
- rebuild projections inside API requests;
- treat cache as truth;
- hide unsupported data by returning guessed current state;
- make a new projection for every field before it is needed.

Add projections when a real route needs speed or predictable pagination.
