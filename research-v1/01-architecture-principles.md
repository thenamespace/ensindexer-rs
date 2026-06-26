# 01 Architecture Principles

The indexer has one job: build a trustworthy, fast, global ENS-style name graph from many chains and contract systems.

## Principle 1: One Name Tree

The product model is a tree, not a set of independent products.

```text
root: .
  eth
    base.eth
      alice.base.eth
        app.alice.base.eth
```

Every name has one canonical place in this tree.

Why this is good:

- users think in names, not source systems;
- parent/child queries are natural;
- a new chain does not require a new public data model;
- subname search works across all sources.

What can go wrong:

- two sources may claim facts about the same name;
- a child may be observed before the parent was indexed;
- L2-managed names may have authority on a different chain than their parent.

How we handle it:

- every fact carries source-family and chain provenance;
- current projections choose the best canonical fact by explicit authority rules;
- hierarchy rows are projections, not hard parent foreign keys on the name identity table.

## Principle 2: Sources Are Adapters

ENS v1, ENS v2, Basenames, Linea names, Celo names, CCIP-read resolvers, wrappers, registrars, and reverse registrars are source families.

They are allowed to be different internally, but they must emit shared normalized events.

```text
source-specific log
  -> source adapter
  -> shared normalized event
```

This keeps hard protocol logic at the edge of the system.

## Principle 3: Raw Facts Are Evidence

Raw chain data is evidence, not the query model.

Raw facts include:

- block identity;
- selected logs;
- selected transactions/receipts;
- selected block-pinned call outputs;
- contract code observations;
- archive chunk metadata.

The API should not scan raw facts for normal queries. Raw facts exist for replay, audit, and repair.

## Principle 4: Normalized Events Are Meaning

A raw `NewResolver` log and a raw `ResolverUpdated` log can both mean:

```text
ResolverChanged
```

Normalized events are where the indexer says, "this chain fact means this semantic change to the name graph."

Every projection must be rebuildable from normalized events plus identity state.

## Principle 5: Identity Is Not Just Name Text

The public name is stable:

```text
alice.eth
```

But the authority behind it can change:

- registry-only authority;
- registrar lease;
- wrapper token;
- ENS v2 resource;
- resolver alias;
- L2-managed authority.

So the design separates:

- `names`: public tree identity;
- `resources`: authority/control object;
- `name_resource_bindings`: time-ranged relation between name and resource;
- `token_lineages`: token ownership history for resources that are tokenized.

## Principle 6: Projections Are Disposable

Current tables serve queries.

They are not truth.

If projection logic changes, we rebuild projections from normalized events and identity tables. This avoids wasting RPC/HyperSync credits and avoids manual data patches.

## Principle 7: Block Hash Beats Block Number

Block number is position. Block hash is identity.

Every reorg-sensitive row must trace back to block hash, transaction hash, and log index or an equivalent block-pinned call.

Reorg repair marks old facts orphaned and rebuilds affected projections. It does not silently delete audit history.

## Principle 8: Manifests Are Code, Snapshots Are Audit

The active source definitions should live in Rust code and typed Alloy event definitions.

Postgres should store immutable snapshots of the active manifest version/hash that produced rows. The database should not be the editable source of truth for event definitions.

Why:

- code gives type safety and speed;
- DB snapshots give replay provenance;
- old rows remain explainable after code changes.

## Principle 9: Keep The Table Set Necessary

Production-grade does not mean adding every possible table.

The required tables are the ones that answer at least one hard question:

- What chain data did we see?
- Which source definition interpreted it?
- What semantic event did it become?
- Which name/resource/token did it affect?
- What is the current answer?
- Can we rebuild or explain it after a bug, reorg, or deployment change?

Tables that do not answer one of those questions should stay out of v1.
