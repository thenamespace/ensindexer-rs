# 03 Global Name Identity

The central identity is the name in the tree.

```text
name: alice.base.eth
parent: base.eth
root: eth
depth: 3
```

That identity is independent of which chain emitted the latest fact.

## Name IDs

Use a deterministic `name_id`.

Recommended:

```text
name_id = namehash(normalized_name)
```

Why:

- stable across deployments;
- compatible with ENS node identifiers;
- easy to derive from events that only include node/hash;
- no source-family prefix.

Store normalized text when known. Some names are initially hash-only because the log has a labelhash but not the readable label.

## Name Table Responsibilities

The `names` table should store stable identity:

- `name_id`;
- normalized name text when known;
- node/namehash;
- labelhash;
- label text when known;
- root name when known;
- depth when known;
- first seen provenance;
- normalization status.

It should not require parent rows to exist before child rows.

Why:

- L2 or registry events can reveal `alice.base.eth` before `base.eth` has a complete authoritative row;
- old ENS registry logs often reveal child hashes before label preimages;
- strict parent foreign keys make historical replay fragile.

Parent/child truth belongs in `name_hierarchy_current`.

## Unknown Labels

ENS logs often provide:

```text
parent node + labelhash
```

without:

```text
label text
```

The indexer should still create a hash-known name identity.

Readable labels can arrive later from:

- registrar events;
- wrapper events;
- ENS v2 label events;
- resolver reverse/name events;
- local ENS Rainbow import;
- verified preimage backfill.

When a preimage is found, validate:

```text
normalize(label) -> hash(label) == labelhash
```

Then update name display and invalidate affected projections.

## Resource Identity

A resource is the control object behind a name.

Examples:

- ENS v1 registry node authority;
- `.eth` registrar lease;
- NameWrapper token;
- ENS v2 EAC resource;
- Base registrar token;
- L2-managed registry authority.

A name can bind to different resources over time.

```text
alice.eth
  -> registrar resource
  -> wrapper resource
  -> registrar resource after unwrap
  -> new registrar resource after full lapse and re-registration
```

This is why `names` and `resources` must be separate.

## Token Lineage

Token identity is not resource identity.

Reasons:

- ENS v1 registrar ERC-721 tokens transfer between owners;
- NameWrapper ERC-1155 tokens represent wrapped control;
- ENS v2 can regenerate token ids while preserving an underlying resource;
- a new registration after lapse should create a new lineage.

`token_lineages` tracks token history for a resource without changing the public name.

## Authority Scope

Authority scope answers:

```text
Which source is allowed to write current facts for which part of the tree?
```

Examples:

- ENS v1 registry can write registry authority for mainnet ENS names;
- `.eth` registrar can write `.eth` 2LD registration facts;
- Basenames Base contracts can write descendants under `base.eth`;
- a future Linea source can write descendants under `linea.eth`;
- a resolver can write records only for nodes it is bound to during its active interval.

Authority scope is provenance and conflict resolution. It is not public name identity.

## Current State Selection

When multiple facts exist, projections choose current state by explicit rules:

1. discard orphaned chain facts;
2. use only active source manifest versions;
3. respect authority scope;
4. use block/log ordering within the source;
5. apply protocol-specific precedence such as wrapper authority over registry owner where appropriate;
6. keep losing facts in history for audit.

The API should expose the chosen current state and allow history/debug views to explain why.
