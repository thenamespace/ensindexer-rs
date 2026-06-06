use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

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
