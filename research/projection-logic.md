# Projection Logic

This document captures how the official ENS subgraph constructs entities from logs.

## Shared Helpers

### Event IDs

Most events use:

```text
createEventID(event) = "{block_number}-{log_index}"
```

`TransferBatch` wrapped transfers append an index:

```text
"{block_number}-{log_index}-{i}"
```

### Accounts

`createOrLoadAccount(address)` creates an `Account` if it does not exist. Most handlers create accounts before assigning ownership fields.

### Domains

`createOrLoadDomain(node)` creates an empty `Domain` with only `id` when necessary. Some fields are filled later by other events.

The root domain is special:

- ID is `ROOT_NODE`;
- owner is `EMPTY_ADDRESS`;
- `isMigrated = true`;
- `createdAt = timestamp`;
- `subdomainCount = 0`.

### Valid Labels

Labels are rejected if they contain:

- null byte `\0`;
- dot `.`;
- `[`;
- `]`.

Invalid labels are not assigned as `labelName`. Registry name construction falls back to bracket notation:

```text
[labelhash_without_0x]
```

### Namehash Construction

Subnode namehash:

```text
keccak256(parent_node_bytes || labelhash_bytes)
```

`.eth` second-level node:

```text
keccak256(ETH_NODE || labelhash)
```

### Labelhash Conversion From `uint256`

Base registrar IDs are `uint256`, but registration IDs are 32-byte labelhash hex strings.

Algorithm:

```text
hex = uint256.to_hex_without_0x().left_pad_to_64()
labelhash = "0x" + hex
```

## Registry Projection

Source: `src/ensRegistry.ts`.

### `NewOwner(node, label, owner)`

1. Create/save `Account(owner)`.
2. Compute `subnode = keccak256(node || label)`.
3. Load existing `Domain(subnode)`.
4. Load parent `Domain(node)`.
5. If missing, create `Domain(subnode)` with:
   - `createdAt = block.timestamp`;
   - `subdomainCount = 0`.
6. If `domain.parent` was null and parent exists:
   - increment `parent.subdomainCount`;
   - save parent.
7. If `domain.name` is null:
   - try `ens.nameByHash(label)`;
   - if label valid, set `domain.labelName`;
   - if invalid or missing, use bracketed labelhash;
   - if parent is root, domain name is the label;
   - otherwise, if parent has a name, domain name is `{label}.{parent.name}`.
8. Set:
   - `domain.owner = owner`;
   - `domain.parent = node`;
   - `domain.labelhash = label`;
   - `domain.isMigrated = isMigrated`.
9. Save via `saveDomain(domain)`.
10. Create immutable `NewOwner` event:
    - `id = block-log`;
    - `parentDomain = node`;
    - `domain = subnode`;
    - `owner = owner`.

`saveDomain` runs `recurseDomainDelete` before saving. It recursively decrements parent subdomain counts for empty domains with:

- no resolver or resolver address is zero;
- owner is empty address;
- subdomain count is zero.

For a SQL implementation, this logic should be carefully tested before enabling physical deletes. The subgraph implementation appears to use it to maintain subdomain counts around empty domains.

### `Transfer(node, owner)`

1. Create/save `Account(owner)`.
2. Load `Domain(node)`.
3. Set `domain.owner = owner`.
4. Save domain.
5. Create immutable `Transfer` event.

### `NewResolver(node, resolver)`

1. If resolver is zero address, resolver ID is `null`.
2. Otherwise resolver ID is `{resolver_address}-{node}`.
3. Load `Domain(node)`.
4. Set `domain.resolver = id`.
5. If resolver ID exists:
   - load/create `Resolver(id)`;
   - on create set `domain`, `address`;
   - if new resolver, set `domain.resolvedAddress = null`;
   - if existing resolver, set `domain.resolvedAddress = resolver.addr`.
6. If resolver ID is null:
   - set `domain.resolvedAddress = null`.
7. Save domain.
8. Create immutable `NewResolver` event.
   - Official event `resolver` field is set to the resolver ID, or `EMPTY_ADDRESS` when cleared.

### `NewTTL(node, ttl)`

1. Load `Domain(node)`.
2. If found, set `domain.ttl = ttl`.
3. Create immutable `NewTTL` event even if domain was missing.

### Old Registry Migration Rules

`ENSRegistryOld` handlers call the same core logic with guards:

- `handleNewOwnerOldRegistry`: only update if domain missing or `isMigrated == false`.
- `handleNewResolverOldRegistry`: update if node is root or `!domain.isMigrated`.
- `handleNewTTLOldRegistry`: update only if `!domain.isMigrated`.
- `handleTransferOldRegistry`: update only if `!domain.isMigrated`.

The new registry marks `isMigrated = true`; the old registry marks `false`.

## Registrar Projection

Source: `src/ethRegistrar.ts`.

### `BaseRegistrar.NameRegistered(id, owner, expires)`

1. Create/save `Account(owner)`.
2. Convert `id: uint256` to `labelhash: bytes32`.
3. Compute domain ID: `keccak256(ETH_NODE || labelhash)`.
4. Load the domain. Official subgraph expects it to exist.
5. Create new `Registration(labelhash)`.
6. Set:
   - `registration.domain = domain.id`;
   - `registration.registrationDate = block.timestamp`;
   - `registration.expiryDate = expires`;
   - `registration.registrant = owner`.
7. Set:
   - `domain.registrant = owner`;
   - `domain.expiryDate = expires + 7776000`.
8. Try `ens.nameByHash(labelhash)`.
   - if valid, set `domain.labelName`, `domain.name = "{label}.eth"`, and `registration.labelName`.
9. Save domain and registration.
10. Create immutable `NameRegistered` event.

### Controller Preimage Events

Controller handlers call `setNamePreimage(name, label, cost)`.

Behavior:

1. Reject invalid labels.
2. Compute `.eth` domain ID.
3. Load domain. Official code assumes it exists.
4. If `domain.labelName != name`, set:
   - `domain.labelName = name`;
   - `domain.name = "{name}.eth"`.
5. Load `Registration(label)`.
6. If no registration exists, return.
7. Set:
   - `registration.labelName = name`;
   - `registration.cost = cost`.

Cost rules:

- legacy registered/renewed: `cost`;
- wrapped registered: `baseCost + premium`;
- wrapped renewed: `cost`;
- unwrapped registered: `baseCost + premium`;
- unwrapped renewed: `cost`.

### `BaseRegistrar.NameRenewed(id, expires)`

1. Convert `id` to labelhash.
2. Load `Registration(labelhash)` and `.eth` domain.
3. Set:
   - `registration.expiryDate = expires`;
   - `domain.expiryDate = expires + 7776000`.
4. Save registration and domain.
5. Create immutable `NameRenewed` event.

### `BaseRegistrar.Transfer(from, to, tokenId)`

1. Create/save `Account(to)`.
2. Convert `tokenId` to labelhash.
3. Load registration.
4. If no registration exists, return.
5. Load `.eth` domain.
6. Set:
   - `registration.registrant = to`;
   - `domain.registrant = to`.
7. Save both.
8. Create immutable `NameTransferred` event.

## Name Wrapper Projection

Source: `src/nameWrapper.ts`.

### DNS Name Decoding

`NameWrapped.name` is DNS-encoded bytes.

Decoder behavior:

1. Read length byte.
2. If first length is zero, return `["", "."]`.
3. Decode each label.
4. Reject if any decoded label fails `checkValidLabel`.
5. Return `[firstLabel, fullNameWithDots]`.

If decoding fails, handler continues with `label = null`, `name = null`.

### `NameWrapped(node, name, owner, fuses, expiry)`

1. Decode DNS-encoded name.
2. Create/load owner account.
3. Create/load domain.
4. If domain has no `labelName` and decoded label exists:
   - set `domain.labelName = label`;
   - set `domain.name = decoded full name`.
5. If `PARENT_CANNOT_CONTROL` is burned and expiry is newer than domain expiry:
   - set `domain.expiryDate = expiry`.
6. Set `domain.wrappedOwner = owner`.
7. Save domain.
8. Create or replace `WrappedDomain(node)`:
   - `domain = domain.id`;
   - `expiryDate = expiry`;
   - `fuses = fuses.toI32()`;
   - `owner = owner`;
   - `name = decoded full name`.
9. Create immutable `NameWrapped` event.

### `NameUnwrapped(node, owner)`

1. Create/load owner account.
2. Create/load domain.
3. Set `domain.wrappedOwner = null`.
4. If `domain.expiryDate` exists and `domain.parent != ETH_NODE`, set `domain.expiryDate = null`.
   - `.eth` 2LD domains keep expiry from registration.
   - subnames lose wrapper-derived expiry.
5. Save domain.
6. Create immutable `NameUnwrapped` event.
7. Remove `WrappedDomain(node)`.

### `FusesSet(node, fuses)`

1. Load `WrappedDomain(node)`.
2. If found:
   - set `wrappedDomain.fuses = fuses`;
   - save;
   - if wrapped expiry exists and PCC is burned:
     - create/load `Domain(node)`;
     - if domain expiry missing or older, set to wrapped expiry.
3. Always create immutable `FusesSet` event.

Important SQL note: the official subgraph treats `expiryDate = 0` as truthy enough to enter this branch. In SQL, if using decimal text plus casts, cast parameters explicitly. If using `numeric`, this issue goes away.

### `ExpiryExtended(node, expiry)`

1. Load `WrappedDomain(node)`.
2. If found:
   - set `wrappedDomain.expiryDate = expiry`;
   - save;
   - if PCC is burned:
     - create/load domain;
     - if domain expiry missing or older, set domain expiry.
3. Always create immutable `ExpiryExtended` event.

### `TransferSingle` and `TransferBatch`

Both call `makeWrappedTransfer`.

`makeWrappedTransfer(blockNumber, transactionID, eventID, node, to)`:

1. Create/load account `to`.
2. Convert ERC-1155 token ID to namehash:
   - `namehash = "0x" + token_id_hex_without_0x.left_pad_to_64()`.
3. Create/load domain.
4. Load `WrappedDomain(namehash)`.
5. If missing, create placeholder:
   - `domain = domain.id`;
   - `expiryDate = 0`;
   - `fuses = 0`.
6. Set `wrappedDomain.owner = to`.
7. Save wrapped domain.
8. Set `domain.wrappedOwner = to`.
9. Save domain.
10. Create immutable `WrappedTransfer` event.

## Resolver Projection

Source: `src/resolver.ts`.

Resolver ID:

```text
{resolver_contract_address}-{node}
```

### `AddrChanged(node, a)`

1. Create/save `Account(a)`.
2. Create `Resolver(id)` with:
   - `domain = node`;
   - `address = event.address`;
   - `addr = a`.
3. If `Domain(node)` exists and `domain.resolver == resolver.id`:
   - set `domain.resolvedAddress = a`.
4. Create immutable `AddrChanged` event.

Note: this handler uses `new Resolver(id)` rather than load/update. In practice it overwrites resolver state for this ID.

### `AddressChanged(node, coinType, newAddress)`

1. Get or create resolver without saving on new.
2. Add `coinType` to `resolver.coinTypes` if not present.
3. Save resolver when modified.
4. Create immutable `MulticoinAddrChanged` event with raw address bytes.

### `NameChanged(node, name)`

1. Ignore names containing null byte.
2. Get or create resolver with `saveOnNew = true`.
3. Create immutable `NameChanged` event.

### `ABIChanged(node, contentType)`

Get/create resolver with `saveOnNew = true`; create `AbiChanged` event.

### `PubkeyChanged(node, x, y)`

Get/create resolver with `saveOnNew = true`; create `PubkeyChanged` event.

### `TextChanged(node, indexedKey, key)` and overloaded `TextChanged(..., value)`

1. Get/create resolver with `saveOnNew = false`.
2. Add key to `resolver.texts` if not present.
3. Save resolver when modified.
4. Create immutable `TextChanged` event.
5. The overload with value also sets `value`.

### `ContenthashChanged(node, hash)`

1. Get/create resolver with `saveOnNew = false`.
2. Set `resolver.contentHash = hash`.
3. Save resolver.
4. Create immutable `ContenthashChanged` event.

### `InterfaceChanged(node, interfaceID, implementer)`

Get/create resolver with `saveOnNew = true`; create `InterfaceChanged` event.

### `AuthorisationChanged(node, owner, target, isAuthorised)`

Get/create resolver with `saveOnNew = true`; create `AuthorisationChanged` event.

### `VersionChanged(node, newVersion)`

1. Create immutable `VersionChanged` event.
2. If `Domain(node)` exists and `domain.resolver == resolverID`:
   - set `domain.resolvedAddress = null`.
3. Get/create resolver with `saveOnNew = false`.
4. Reset resolver mutable state:
   - `addr = null`;
   - `contentHash = null`;
   - `texts = null`;
   - `coinTypes = null`.
5. Save resolver.

## Ordering and Idempotency Concerns

The official subgraph is event-order sensitive but Graph Node processes logs deterministically by block and log index. A custom Rust indexer must preserve ordering where entities depend on earlier events.

Critical ordering dependencies:

- `NewOwner` creates domains that registrar events expect to exist.
- `BaseRegistrar.NameRenewed` expects `Registration` to exist.
- `BaseRegistrar.Transfer` returns early if registration does not exist.
- `NameWrapper.TransferSingle` can precede `NameWrapped`, so placeholder wrapped domains are required.
- `Resolver.AddrChanged` only updates `Domain.resolvedAddress` if the domain currently points to that resolver ID.

Recommended implementation rule:

- Process logs in `(block_number, transaction_index, log_index)` order per chain.
- Within the same global log stream, dispatch to projections in order.
- Use idempotent upserts for mutable entities.
- Use `insert on conflict do nothing` for immutable events by event ID.

