# 15 Authority Scope Examples

Authority scope answers one question:

```text
Which source is allowed to write which kind of current fact for which names?
```

It is not a public product partition.

## Example: `vitalik.eth`

Name identity:

```text
vitalik.eth
```

Likely sources:

- ENS v1 registry for owner/resolver/TTL;
- `.eth` registrar for registration/expiry/registrant;
- NameWrapper if wrapped;
- resolver for records;
- reverse/primary execution for primary-name verification.

Current projection combines those sources by authority rules.

## Example: `alice.base.eth`

Name identity:

```text
alice.base.eth
```

Tree position:

```text
eth -> base.eth -> alice.base.eth
```

Likely sources:

- Ethereum ENS registry for `base.eth`;
- Base registry/registrar/resolver for descendants under `base.eth`;
- L1 compatibility resolver for verified resolution paths;
- Base primary contract for declared address-to-name claims.

Important:

The name stays in the same tree. Only the authority source differs.

## Example: Child Before Parent

Possible observation order:

```text
Base emits alice.base.eth registration
Ethereum base.eth metadata is not indexed yet
```

Correct behavior:

- create `names` row for `alice.base.eth`;
- create hierarchy projection edge when parent identity can be derived;
- mark parent current details as unknown/incomplete if not indexed;
- never fail because a parent FK is missing.

## Example: Two Sources Claim A Fact

Possible conflict:

```text
source A says resolver = 0x111...
source B says resolver = 0x222...
```

Correct behavior:

1. keep both facts in normalized events;
2. check authority scope;
3. choose current resolver by explicit precedence;
4. record conflict/finding if both sources appear authoritative;
5. expose debug provenance.

Never silently overwrite one source without traceability.

## Example: Wrapper Authority

For a wrapped `.eth` name:

```text
registrar token holder != effective controller
```

Correct behavior:

- registrar resource still exists;
- wrapper resource becomes active control resource;
- fuses mask permissions;
- current owner/controller reflects effective authority;
- history still explains the registrar token.

## Example: ENS v2 Token Regeneration

ENS v2 can change token id while preserving resource identity.

Correct behavior:

- keep same `resource_id`;
- update token lineage state;
- do not create a new public name;
- invalidate permissions/current rows tied to token id.

## Long-Term Rule

When adding a future chain, ask:

```text
What part of the existing tree can this source authoritatively update?
```

Do not ask:

```text
What new public bucket/table should this chain get?
```
