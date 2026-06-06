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
        filter.owner_id_not = self.owner_not;
        filter.owner_id_gt = self.owner_gt;
        filter.owner_id_lt = self.owner_lt;
        filter.owner_id_gte = self.owner_gte;
        filter.owner_id_lte = self.owner_lte;
        filter.owner_id_in = self.owner_in;
        filter.owner_id_not_in = self.owner_not_in;
        filter.owner_id_contains = self.owner_contains;
        filter.owner_id_not_contains = self.owner_not_contains;
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
        filter.addr_id_not = self.addr_not;
        filter.addr_id_gt = self.addr_gt;
        filter.addr_id_lt = self.addr_lt;
        filter.addr_id_gte = self.addr_gte;
        filter.addr_id_lte = self.addr_lte;
        filter.addr_id_in = self.addr_in;
        filter.addr_id_not_in = self.addr_not_in;
        filter.addr_id_contains = self.addr_contains;
        filter.addr_id_not_contains = self.addr_not_contains;
    }
}
