# 06 Identity And Normalization

Bigname's identity model solves a core ENS problem: public name text, backing authority, token ID, resolver identity, and control state are not the same thing.

## Normalization Boundary

Bigname uses one pinned ENSIP-15 normalizer:

```text
ensip15@ens-normalize-0.1.1
```

This boundary applies to:

- API input normalization;
- adapter name-surface admission;
- reverse claim names;
- resolver alias targets;
- DNS-encoded names;
- namehash;
- labelhash path.

It does not fall back to:

- ASCII lowercasing;
- route-local trimming;
- IDNA/UTS-46 conversion.

Blank reverse claim strings are no claim. Nonblank invalid strings become invalid-name state.

## NameSurface

`name_surfaces` stores canonical public name surface identity.

It includes:

- `logical_name_id`;
- namespace;
- normalized name;
- canonical display name;
- DNS wire name;
- namehash;
- labelhashes;
- normalizer version;
- warnings/errors;
- provenance;
- canonicality.

Alternate spellings do not create more surfaces. They remain preimage observations.

## Resource

`resources` stores backing authority objects.

ENS v1 examples:

- registry-only authority for `sub.alice.eth`;
- registrar lease for `alice.eth`;
- wrapper-backed authority for wrapped `alice.eth`.

ENS v2 example:

- EAC resource returned by registry, not current ERC-1155 token id.

Basenames example:

- Base-side registry/registrar authority object for `basenames:alice.base.eth`.

## SurfaceBinding

`surface_bindings` links surface to resource through time.

Examples:

```text
ens:alice.eth -> registrar resource
ens:alice.eth -> wrapper resource
ens:alice.eth -> prior registrar resource after unwrap
```

Transfers do not create a new binding if the authority anchor remains the same.

Wrap, unwrap, re-registration, migration, alias path, or wildcard observation can create or close bindings.

## TokenLineage

`token_lineages` tracks tokenized ownership history.

ENSv1:

- registry-only authority has no token lineage;
- registrar lease has one;
- wrapper token has one;
- wrap rotates lineage;
- unwrap before lease ends can reactivate prior registrar lineage.

ENSv2:

- `TokenRegenerated` changes token id but preserves resource and token lineage.

## ContractInstance

Contract identity is stable across:

- manifest version changes;
- active range changes;
- proxy implementation changes.

Proxy and implementation are separate instances connected by edges.

## ENSv1 Authority Continuity

Bigname's ENSv1 authority model distinguishes these cases:

| Case | Surface | Resource | Token Lineage |
| --- | --- | --- | --- |
| Registry-only subname | same | registry resource | none |
| Registrar registration | same | registrar resource | registrar lineage |
| Registry owner diverges from registrar holder | same | registry-only resource | none |
| Registry owner realigns before release | same | prior registrar resource | prior registrar lineage |
| Wrap | same | wrapper resource | wrapper lineage |
| Unwrap before lease ends | same | prior registrar resource | prior registrar lineage |
| Expiry/grace | same | same resource | same lineage |
| Re-registration after lapse | same | new registrar resource | new lineage |

This avoids collapsing ownership/control histories into one flat owner field.

## ENSv2 Resource Continuity

ENSv2 upstream has:

```text
getResource(anyId)
getTokenId(anyId)
TokenResource(tokenId, resource)
TokenRegenerated(oldTokenId, newTokenId)
```

Bigname keys `resource_id` by upstream EAC resource.

Token regeneration updates token attributes, but does not create a new resource.

Unregister/re-register creates a new resource and token lineage.

## Preimage Observations

`PreimageObserved` and `label_preimages` make labels readable.

Sources:

- ENS registrar labels;
- wrapper full names;
- ENSv2 label events;
- resolver name-bearing events;
- reverse claims;
- ENS Rainbow import.

Rules:

- preimage must normalize correctly;
- normalized label must hash back to labelhash;
- preimage does not create authority;
- preimage can invalidate children projections so unknown placeholders become readable.

## Invalid Names

Bigname does not silently coerce invalid names.

Examples:

- embedded NUL in registrar label;
- invalid resolver `name()` preimage;
- nonnormalizable reverse claim.

These may stay raw facts or resolver record events, but they do not mint/update `NameSurface` if they cannot pass the normalization boundary.

## Our Takeaway

The identity/resource/token split is worth copying.

The namespace prefix is not.

For our indexer:

```text
bigname: logical_name_id = namespace:name
ours:    name_id = normalized full name
```

But these concepts should remain:

```text
resources
surface_bindings
token_lineages
contract_instances
label_preimages
normalization findings
```

