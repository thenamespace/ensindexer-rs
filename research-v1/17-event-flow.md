# 17 Event Flow

This file walks through the full journey from chain data to API response.

## Flow 1: Historical Backfill From HyperSync

```text
start backfill job
  -> load source-family watch targets
  -> fetch logs by block range
  -> write raw archive range
  -> optionally write raw_logs metadata
  -> run adapters
  -> upsert names/resources/bindings
  -> insert normalized_events
  -> append normalized_event_changes
  -> enqueue projection_invalidations
  -> rebuild current projection keys
  -> advance checkpoints
```

Key rule:

Raw fetch and semantic replay are separate enough to resume, but they use the same adapter/projection path as live indexing.

## Flow 2: Replay From Raw Archive

```text
read archive manifest
  -> validate checksum and format version
  -> stream one range
  -> decode logs
  -> run same adapters
  -> write same normalized events
  -> rebuild same projections
```

This is how projection bugs are fixed without provider credits.

## Flow 3: Live Indexing

```text
poll latest block
  -> choose target latest - confirmation_depth
  -> fetch missing blocks/logs
  -> verify parent hash against chain_blocks
  -> ingest raw facts
  -> run adapters
  -> update projections
  -> update live checkpoint
```

If parent hash does not match, enter reorg flow.

## Flow 4: Reorg

```text
new block conflicts with stored parent
  -> walk chain_blocks to common ancestor
  -> mark losing chain blocks orphaned
  -> mark raw facts orphaned
  -> mark normalized events orphaned
  -> invalidate names/resources/addresses/resolvers/cache touched by those events
  -> ingest winning branch
  -> rebuild affected projections
```

Current tables should never use orphaned rows.

## Flow 5: `.eth` Registration

Raw events:

```text
NameRegistered(label, owner, expires)
Transfer(from, to, tokenId)
NewOwner(parent, labelhash, owner)
NewResolver(node, resolver)
```

Indexer behavior:

1. create or update `names` for `label.eth`;
2. validate/store label preimage;
3. create registrar `resource`;
4. bind name to resource;
5. update token lineage and owner role;
6. emit `RegistrationGranted`, `TokenControlChanged`, `AuthorityChanged`, `ResolverChanged`;
7. invalidate exact name, parent children, owner address, resolver, search.

## Flow 6: Subname From Registry

Raw event:

```text
NewOwner(parentNode, labelhash, owner)
```

Indexer behavior:

1. derive child node/namehash;
2. create hash-known name if label text is unknown;
3. create registry resource;
4. bind child name to resource;
5. emit `NameObserved` and `AuthorityChanged`;
6. invalidate parent children and owner address projections.

If label preimage arrives later, update display/search and invalidate children/search.

## Flow 7: NameWrapper Wrap

Raw event:

```text
NameWrapped(node, dnsEncodedName, owner, fuses, expiry)
```

Indexer behavior:

1. decode and normalize name;
2. validate node/namehash;
3. store label preimages from DNS encoded name;
4. create wrapper resource;
5. close prior active binding if required;
6. bind name to wrapper resource;
7. update token lineage;
8. store fuse/permission scope;
9. emit wrap/control/permission events;
10. rebuild name, permissions, address-name, search.

## Flow 8: Resolver Record Change

Raw event:

```text
AddressChanged(node, coinType, value)
TextChanged(node, key, value)
ContenthashChanged(node, hash)
```

Indexer behavior:

1. confirm resolver instance is supported or mark unsupported;
2. find active resolver binding for node at block;
3. emit `RecordChanged`;
4. update `resolver_records_current`;
5. invalidate exact name record bundle, address relation if coin type 60, cache for verified resolution if affected.

## Flow 9: ENS v2 Token Regeneration

Raw event:

```text
TokenRegenerated(oldTokenId, newTokenId)
```

Indexer behavior:

1. find stable resource;
2. update token lineage;
3. emit `TokenRegenerated`;
4. do not create a new name;
5. invalidate permissions/address-name rows that depend on token id.

## Flow 10: L2-Managed `alice.base.eth`

Raw source:

```text
Base registrar/registry/resolver events
```

Indexer behavior:

1. map source authority to descendants of `base.eth`;
2. create/update `names` for `alice.base.eth`;
3. store Base chain provenance;
4. emit the same normalized events as equivalent ENS events;
5. project into the same `name_current`, `name_hierarchy_current`, `address_names_current`, and search tables.

No separate public model is created.

## Flow 11: Verified Primary Name

Inputs:

- declared reverse/primary claim;
- resolver records;
- block-pinned Universal Resolver or source-specific execution;
- forward resolution check.

Indexer behavior:

1. store declared claim as candidate;
2. run block-pinned verification;
3. persist execution trace and steps;
4. store reusable execution outcome;
5. update `primary_names_current`;
6. cache result with dependencies;
7. invalidate when resolver, record, claim, or reorg touches dependencies.

## Flow 12: Projection Rebuild After Logic Change

```text
new projection version deployed
  -> create projection_rebuild_run
  -> scan normalized_event_changes or full identity set
  -> rebuild current rows in batches
  -> swap/commit projection version
  -> record completion
```

Do not refetch raw logs unless adapter semantics changed and raw archives are unavailable.

## Flow 13: Label Healing

```text
hash-only child exists
  -> local ENS Rainbow/import finds label
  -> validate normalization and labelhash
  -> upsert label_preimages
  -> update names display fields
  -> invalidate parent children and search
```

Label healing improves readability. It does not create authority.
