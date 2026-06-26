# 11 Edge Cases And Tests

The test suite should prove that the tree model stays correct under messy historical behavior.

## Name Identity Tests

- normalize valid ENS names;
- reject invalid names;
- preserve hash-only names;
- heal label preimages later;
- update display name after preimage validation;
- support arbitrary subname depth;
- observe child before parent without failure.

## Authority Tests

- registry owner change;
- registrar registration and renewal;
- registration expiry/lapse/re-registration;
- wrapper wrap/unwrap;
- wrapper fuse masking;
- ENS v2 resource/token regeneration;
- L2-managed descendant under an L1 parent.

## Resolver Tests

- resolver changed;
- address/text/contenthash records;
- resolver version boundary;
- resolver profile unsupported;
- event-silent resolver hydration;
- resolver alias if supported.

## Reverse / Primary Tests

- reverse claim changes;
- reverse name record changes;
- declared claim without forward verification;
- verified primary success;
- verified primary failure;
- cache invalidation after resolver/name change.

## Reorg Tests

- one-block reorg;
- multi-block reorg;
- reorg over registration;
- reorg over resolver change;
- reorg over label preimage;
- orphaned rows ignored by projections;
- projections rebuilt after winning branch ingestion.

## Replay Tests

- raw archive replay equals RPC/HyperSync replay;
- adapter replay is idempotent;
- projection rebuild produces same current tables;
- changed projection logic can rebuild without refetching raw logs;
- manifest snapshot change is explicit.

## Performance Tests

- dense raw archive range replay;
- 100k+ logs per range;
- batch insert timings;
- projection rebuild timings;
- search query latency after full backfill;
- index rebuild timing;
- public API benchmark suite.

## Audit Tests

For any current row, test that the system can trace:

```text
current row
  -> projection rebuild/invalidation
  -> normalized event
  -> raw fact or execution trace
  -> source manifest snapshot
  -> chain block hash
```

If that trace breaks, the system is not production-auditable.
