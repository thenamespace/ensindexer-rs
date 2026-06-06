use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct NameFieldFilter {
    #[graphql(name = "name")]
    pub name: Option<String>,
    #[graphql(name = "name_not")]
    pub name_not: Option<String>,
    #[graphql(name = "name_gt")]
    pub name_gt: Option<String>,
    #[graphql(name = "name_lt")]
    pub name_lt: Option<String>,
    #[graphql(name = "name_gte")]
    pub name_gte: Option<String>,
    #[graphql(name = "name_lte")]
    pub name_lte: Option<String>,
    #[graphql(name = "name_in")]
    pub name_in: Option<Vec<String>>,
    #[graphql(name = "name_not_in")]
    pub name_not_in: Option<Vec<String>>,
    #[graphql(name = "name_contains")]
    pub name_contains: Option<String>,
    #[graphql(name = "name_contains_nocase")]
    pub name_contains_nocase: Option<String>,
    #[graphql(name = "name_not_contains")]
    pub name_not_contains: Option<String>,
    #[graphql(name = "name_not_contains_nocase")]
    pub name_not_contains_nocase: Option<String>,
    #[graphql(name = "name_starts_with")]
    pub name_starts_with: Option<String>,
    #[graphql(name = "name_starts_with_nocase")]
    pub name_starts_with_nocase: Option<String>,
    #[graphql(name = "name_not_starts_with")]
    pub name_not_starts_with: Option<String>,
    #[graphql(name = "name_not_starts_with_nocase")]
    pub name_not_starts_with_nocase: Option<String>,
    #[graphql(name = "name_ends_with")]
    pub name_ends_with: Option<String>,
    #[graphql(name = "name_ends_with_nocase")]
    pub name_ends_with_nocase: Option<String>,
    #[graphql(name = "name_not_ends_with")]
    pub name_not_ends_with: Option<String>,
    #[graphql(name = "name_not_ends_with_nocase")]
    pub name_not_ends_with_nocase: Option<String>,
}

impl ApplyEventFilter for NameFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.name = self.name;
        filter.name_contains = self.name_contains;
        filter.name_contains_nocase = self.name_contains_nocase;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct KeyFieldFilter {
    #[graphql(name = "key")]
    pub key: Option<String>,
    #[graphql(name = "key_not")]
    pub key_not: Option<String>,
    #[graphql(name = "key_gt")]
    pub key_gt: Option<String>,
    #[graphql(name = "key_lt")]
    pub key_lt: Option<String>,
    #[graphql(name = "key_gte")]
    pub key_gte: Option<String>,
    #[graphql(name = "key_lte")]
    pub key_lte: Option<String>,
    #[graphql(name = "key_in")]
    pub key_in: Option<Vec<String>>,
    #[graphql(name = "key_not_in")]
    pub key_not_in: Option<Vec<String>>,
    #[graphql(name = "key_contains")]
    pub key_contains: Option<String>,
    #[graphql(name = "key_contains_nocase")]
    pub key_contains_nocase: Option<String>,
    #[graphql(name = "key_not_contains")]
    pub key_not_contains: Option<String>,
    #[graphql(name = "key_not_contains_nocase")]
    pub key_not_contains_nocase: Option<String>,
    #[graphql(name = "key_starts_with")]
    pub key_starts_with: Option<String>,
    #[graphql(name = "key_starts_with_nocase")]
    pub key_starts_with_nocase: Option<String>,
    #[graphql(name = "key_not_starts_with")]
    pub key_not_starts_with: Option<String>,
    #[graphql(name = "key_not_starts_with_nocase")]
    pub key_not_starts_with_nocase: Option<String>,
    #[graphql(name = "key_ends_with")]
    pub key_ends_with: Option<String>,
    #[graphql(name = "key_ends_with_nocase")]
    pub key_ends_with_nocase: Option<String>,
    #[graphql(name = "key_not_ends_with")]
    pub key_not_ends_with: Option<String>,
    #[graphql(name = "key_not_ends_with_nocase")]
    pub key_not_ends_with_nocase: Option<String>,
}

impl ApplyEventFilter for KeyFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.key = self.key;
        filter.key_contains = self.key_contains;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ValueFieldFilter {
    #[graphql(name = "value")]
    pub value: Option<String>,
    #[graphql(name = "value_not")]
    pub value_not: Option<String>,
    #[graphql(name = "value_gt")]
    pub value_gt: Option<String>,
    #[graphql(name = "value_lt")]
    pub value_lt: Option<String>,
    #[graphql(name = "value_gte")]
    pub value_gte: Option<String>,
    #[graphql(name = "value_lte")]
    pub value_lte: Option<String>,
    #[graphql(name = "value_in")]
    pub value_in: Option<Vec<String>>,
    #[graphql(name = "value_not_in")]
    pub value_not_in: Option<Vec<String>>,
    #[graphql(name = "value_contains")]
    pub value_contains: Option<String>,
    #[graphql(name = "value_contains_nocase")]
    pub value_contains_nocase: Option<String>,
    #[graphql(name = "value_not_contains")]
    pub value_not_contains: Option<String>,
    #[graphql(name = "value_not_contains_nocase")]
    pub value_not_contains_nocase: Option<String>,
    #[graphql(name = "value_starts_with")]
    pub value_starts_with: Option<String>,
    #[graphql(name = "value_starts_with_nocase")]
    pub value_starts_with_nocase: Option<String>,
    #[graphql(name = "value_not_starts_with")]
    pub value_not_starts_with: Option<String>,
    #[graphql(name = "value_not_starts_with_nocase")]
    pub value_not_starts_with_nocase: Option<String>,
    #[graphql(name = "value_ends_with")]
    pub value_ends_with: Option<String>,
    #[graphql(name = "value_ends_with_nocase")]
    pub value_ends_with_nocase: Option<String>,
    #[graphql(name = "value_not_ends_with")]
    pub value_not_ends_with: Option<String>,
    #[graphql(name = "value_not_ends_with_nocase")]
    pub value_not_ends_with_nocase: Option<String>,
}

impl ApplyEventFilter for ValueFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.value = self.value;
        filter.value_contains = self.value_contains;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct HashFieldFilter {
    #[graphql(name = "hash")]
    pub hash: Option<String>,
    #[graphql(name = "hash_not")]
    pub hash_not: Option<String>,
    #[graphql(name = "hash_gt")]
    pub hash_gt: Option<String>,
    #[graphql(name = "hash_lt")]
    pub hash_lt: Option<String>,
    #[graphql(name = "hash_gte")]
    pub hash_gte: Option<String>,
    #[graphql(name = "hash_lte")]
    pub hash_lte: Option<String>,
    #[graphql(name = "hash_in")]
    pub hash_in: Option<Vec<String>>,
    #[graphql(name = "hash_not_in")]
    pub hash_not_in: Option<Vec<String>>,
    #[graphql(name = "hash_contains")]
    pub hash_contains: Option<String>,
    #[graphql(name = "hash_not_contains")]
    pub hash_not_contains: Option<String>,
}

impl ApplyEventFilter for HashFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.hash = self.hash;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct XFieldFilter {
    #[graphql(name = "x")]
    pub x: Option<String>,
    #[graphql(name = "x_not")]
    pub x_not: Option<String>,
    #[graphql(name = "x_gt")]
    pub x_gt: Option<String>,
    #[graphql(name = "x_lt")]
    pub x_lt: Option<String>,
    #[graphql(name = "x_gte")]
    pub x_gte: Option<String>,
    #[graphql(name = "x_lte")]
    pub x_lte: Option<String>,
    #[graphql(name = "x_in")]
    pub x_in: Option<Vec<String>>,
    #[graphql(name = "x_not_in")]
    pub x_not_in: Option<Vec<String>>,
    #[graphql(name = "x_contains")]
    pub x_contains: Option<String>,
    #[graphql(name = "x_not_contains")]
    pub x_not_contains: Option<String>,
}

impl ApplyEventFilter for XFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.x = self.x;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct YFieldFilter {
    #[graphql(name = "y")]
    pub y: Option<String>,
    #[graphql(name = "y_not")]
    pub y_not: Option<String>,
    #[graphql(name = "y_gt")]
    pub y_gt: Option<String>,
    #[graphql(name = "y_lt")]
    pub y_lt: Option<String>,
    #[graphql(name = "y_gte")]
    pub y_gte: Option<String>,
    #[graphql(name = "y_lte")]
    pub y_lte: Option<String>,
    #[graphql(name = "y_in")]
    pub y_in: Option<Vec<String>>,
    #[graphql(name = "y_not_in")]
    pub y_not_in: Option<Vec<String>>,
    #[graphql(name = "y_contains")]
    pub y_contains: Option<String>,
    #[graphql(name = "y_not_contains")]
    pub y_not_contains: Option<String>,
}

impl ApplyEventFilter for YFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.y = self.y;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct InterfaceIdFieldFilter {
    #[graphql(name = "interfaceID")]
    pub interface_id: Option<String>,
    #[graphql(name = "interfaceID_not")]
    pub interface_id_not: Option<String>,
    #[graphql(name = "interfaceID_gt")]
    pub interface_id_gt: Option<String>,
    #[graphql(name = "interfaceID_lt")]
    pub interface_id_lt: Option<String>,
    #[graphql(name = "interfaceID_gte")]
    pub interface_id_gte: Option<String>,
    #[graphql(name = "interfaceID_lte")]
    pub interface_id_lte: Option<String>,
    #[graphql(name = "interfaceID_in")]
    pub interface_id_in: Option<Vec<String>>,
    #[graphql(name = "interfaceID_not_in")]
    pub interface_id_not_in: Option<Vec<String>>,
    #[graphql(name = "interfaceID_contains")]
    pub interface_id_contains: Option<String>,
    #[graphql(name = "interfaceID_not_contains")]
    pub interface_id_not_contains: Option<String>,
}

impl ApplyEventFilter for InterfaceIdFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.interface_id = self.interface_id;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ImplementerFieldFilter {
    #[graphql(name = "implementer")]
    pub implementer: Option<String>,
    #[graphql(name = "implementer_not")]
    pub implementer_not: Option<String>,
    #[graphql(name = "implementer_gt")]
    pub implementer_gt: Option<String>,
    #[graphql(name = "implementer_lt")]
    pub implementer_lt: Option<String>,
    #[graphql(name = "implementer_gte")]
    pub implementer_gte: Option<String>,
    #[graphql(name = "implementer_lte")]
    pub implementer_lte: Option<String>,
    #[graphql(name = "implementer_in")]
    pub implementer_in: Option<Vec<String>>,
    #[graphql(name = "implementer_not_in")]
    pub implementer_not_in: Option<Vec<String>>,
    #[graphql(name = "implementer_contains")]
    pub implementer_contains: Option<String>,
    #[graphql(name = "implementer_not_contains")]
    pub implementer_not_contains: Option<String>,
}

impl ApplyEventFilter for ImplementerFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.implementer = self.implementer;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct TargetFieldFilter {
    #[graphql(name = "target")]
    pub target: Option<String>,
    #[graphql(name = "target_not")]
    pub target_not: Option<String>,
    #[graphql(name = "target_gt")]
    pub target_gt: Option<String>,
    #[graphql(name = "target_lt")]
    pub target_lt: Option<String>,
    #[graphql(name = "target_gte")]
    pub target_gte: Option<String>,
    #[graphql(name = "target_lte")]
    pub target_lte: Option<String>,
    #[graphql(name = "target_in")]
    pub target_in: Option<Vec<String>>,
    #[graphql(name = "target_not_in")]
    pub target_not_in: Option<Vec<String>>,
    #[graphql(name = "target_contains")]
    pub target_contains: Option<String>,
    #[graphql(name = "target_not_contains")]
    pub target_not_contains: Option<String>,
}

impl ApplyEventFilter for TargetFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.target = self.target;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct AuthOwnerFieldFilter {
    #[graphql(name = "owner")]
    pub owner: Option<String>,
    #[graphql(name = "owner_not")]
    pub owner_not: Option<String>,
    #[graphql(name = "owner_gt")]
    pub owner_gt: Option<String>,
    #[graphql(name = "owner_lt")]
    pub owner_lt: Option<String>,
    #[graphql(name = "owner_gte")]
    pub owner_gte: Option<String>,
    #[graphql(name = "owner_lte")]
    pub owner_lte: Option<String>,
    #[graphql(name = "owner_in")]
    pub owner_in: Option<Vec<String>>,
    #[graphql(name = "owner_not_in")]
    pub owner_not_in: Option<Vec<String>>,
    #[graphql(name = "owner_contains")]
    pub owner_contains: Option<String>,
    #[graphql(name = "owner_not_contains")]
    pub owner_not_contains: Option<String>,
}

impl ApplyEventFilter for AuthOwnerFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.owner_id = self.owner;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct AddrBytesFieldFilter {
    #[graphql(name = "addr")]
    pub addr: Option<String>,
    #[graphql(name = "addr_not")]
    pub addr_not: Option<String>,
    #[graphql(name = "addr_gt")]
    pub addr_gt: Option<String>,
    #[graphql(name = "addr_lt")]
    pub addr_lt: Option<String>,
    #[graphql(name = "addr_gte")]
    pub addr_gte: Option<String>,
    #[graphql(name = "addr_lte")]
    pub addr_lte: Option<String>,
    #[graphql(name = "addr_in")]
    pub addr_in: Option<Vec<String>>,
    #[graphql(name = "addr_not_in")]
    pub addr_not_in: Option<Vec<String>>,
    #[graphql(name = "addr_contains")]
    pub addr_contains: Option<String>,
    #[graphql(name = "addr_not_contains")]
    pub addr_not_contains: Option<String>,
}

impl ApplyEventFilter for AddrBytesFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.addr_id = self.addr;
    }
}
