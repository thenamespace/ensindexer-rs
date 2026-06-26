# 08 Execution And Cache

Bigname separates declared state from verified execution.

Declared state comes from indexed events/projections.

Verified state comes from block-pinned execution through admitted entrypoints and persisted traces.

## Execution Owners

### ENS

`ens_execution` owns verified ENS resolution through the Universal Resolver proxy.

### Basenames

`basenames_execution` owns verified Basenames resolution through the L1 Resolver.

`basenames_l1_compat` may reference the same L1 Resolver address, but it owns transport attribution, not execution.

## Execution Flow

```text
API route requests verified/both
  -> select snapshot chain positions
  -> load declared topology snapshot
  -> validate support class
  -> look for reusable execution outcome
  -> on miss, execute against selected snapshot
  -> persist execution_trace
  -> persist execution_steps
  -> persist execution_cache_outcome
  -> return verified section
```

Execution must not target provider `latest` independently of selected route snapshot except for documented route-local ENS/60 fallback behavior.

## Execution Trace

`execution_traces` stores:

- trace id;
- request type;
- request key;
- namespace;
- chain positions;
- manifest versions;
- trace status;
- failure reason;
- final value digest;
- timing.

`execution_steps` stores ordered steps:

- step index;
- step kind;
- input digest;
- output digest;
- latency;
- canonicality dependency;
- details.

Traces and steps are audit artifacts. Reorg invalidation does not delete them.

## Execution Outcomes

`execution_cache_outcomes` stores reusable verified answers.

Cache identity includes:

- request type;
- request key;
- requested chain positions;
- manifest versions;
- topology version boundary;
- record version boundary;
- dependency set.

Outcome reuse fails closed if any identity part mismatches.

## Verified Resolution Support

ENS supported classes:

1. Direct path.
2. Alias-only non-direct path.
3. Wildcard-derived path.

Unsupported ENS classes include:

- non-alias ancestor-selected paths;
- linked-subregistry ancestor paths;
- transport-assisted paths;
- unsupported CCIP classes.

Basenames supported class:

```text
exact-surface transport-assisted direct path
```

It requires:

- resolver path starts at requested surface;
- no wildcard;
- no alias;
- no subregistry path;
- transport from Base to Ethereum through the admitted L1 Resolver.

CCIP participation is allowed for that Basenames class because the upstream L1 Resolver uses OffchainLookup/resolveWithProof.

## Verified Primary Names

Declared primary claim state lives in:

```text
primary_names_current(address, coin_type, namespace)
```

Verified primary result lives in execution outcome readback.

Request key:

```text
{namespace}:{normalized_address}:{coin_type}
```

The matching primary row is the only claim-side anchor.

`primary_names_current` must not persist `execution_trace_id` or verified payload. It can hold claim-side invalidation hooks.

## ENS/60 Fallback

Bigname has a narrow route-local fallback for ENS coin type 60 when persisted tuple is missing:

```text
build addr.reverse node
read registry resolver
call name(bytes32)
normalize claim
optionally verify forward addr:60 via Universal Resolver
return route-local result
```

This fallback:

- does not persist execution trace;
- does not populate `primary_names_current`;
- does not generalize to all namespaces/coin types.

## Cache Invalidation

Execution cache invalidates on:

- reorg;
- manifest change;
- resolver change;
- alias/wildcard topology change;
- relevant record change;
- primary claim change.

Reorg invalidates outcomes whose dependency set includes an orphaned block hash.

Dependencies must tie to explicit block-hash-bearing positions. Block numbers alone are not enough.

## Explain Routes

Execution explain reads persisted trace/steps.

It does not:

- re-execute;
- synthesize topology;
- expose raw gateway transcript;
- mutate cache;
- mutate projections.

For verified resolution, explain surfaces:

- selected entrypoint;
- resolver discovery path;
- wildcard traversal;
- alias rewriting;
- CCIP steps if persisted;
- final comparison/value.

