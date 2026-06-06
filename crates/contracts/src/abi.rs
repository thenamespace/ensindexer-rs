use alloy::sol;

pub mod registry {
    use super::sol;

    sol! {
        event Transfer(bytes32 indexed node, address owner);
        event NewOwner(bytes32 indexed node, bytes32 indexed label, address owner);
        event NewResolver(bytes32 indexed node, address resolver);
        event NewTTL(bytes32 indexed node, uint64 ttl);
    }
}

pub mod base_registrar {
    use super::sol;

    sol! {
        event NameRegistered(uint256 indexed id, address indexed owner, uint256 expires);
        event NameRenewed(uint256 indexed id, uint256 expires);
        event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    }
}

pub mod legacy_eth_registrar_controller {
    use super::sol;

    sol! {
        event NameRegistered(string name, bytes32 indexed label, address indexed owner, uint256 cost, uint256 expires);
        event NameRenewed(string name, bytes32 indexed label, uint256 cost, uint256 expires);
    }
}

pub mod wrapped_eth_registrar_controller {
    use super::sol;

    sol! {
        event NameRegistered(string name, bytes32 indexed label, address indexed owner, uint256 baseCost, uint256 premium, uint256 expires);
        event NameRenewed(string name, bytes32 indexed label, uint256 cost, uint256 expires);
    }
}

pub mod unwrapped_eth_registrar_controller {
    use super::sol;

    sol! {
        event NameRegistered(string label, bytes32 indexed labelhash, address indexed owner, uint256 baseCost, uint256 premium, uint256 expires, bytes32 referrer);
        event NameRenewed(string label, bytes32 indexed labelhash, uint256 cost, uint256 expires, bytes32 referrer);
    }
}

pub mod name_wrapper {
    use super::sol;

    sol! {
        event NameWrapped(bytes32 indexed node, bytes name, address owner, uint32 fuses, uint64 expiry);
        event NameUnwrapped(bytes32 indexed node, address owner);
        event FusesSet(bytes32 indexed node, uint32 fuses);
        event ExpiryExtended(bytes32 indexed node, uint64 expiry);
        event TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value);
        event TransferBatch(address indexed operator, address indexed from, address indexed to, uint256[] ids, uint256[] values);
    }
}

pub mod resolver {
    use super::sol;

    sol! {
        event AddrChanged(bytes32 indexed node, address a);
        event AddressChanged(bytes32 indexed node, uint256 coinType, bytes newAddress);
        event NameChanged(bytes32 indexed node, string name);
        event ABIChanged(bytes32 indexed node, uint256 indexed contentType);
        event PubkeyChanged(bytes32 indexed node, bytes32 x, bytes32 y);
        event TextChanged(bytes32 indexed node, string indexed indexedKey, string key);
        event TextChangedWithValue(bytes32 indexed node, string indexed indexedKey, string key, string value);
        event ContenthashChanged(bytes32 indexed node, bytes hash);
        event InterfaceChanged(bytes32 indexed node, bytes4 indexed interfaceID, address implementer);
        event AuthorisationChanged(bytes32 indexed node, address indexed owner, address indexed target, bool isAuthorized);
        event VersionChanged(bytes32 indexed node, uint64 newVersion);
    }
}
