use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

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
        filter.hash_not = self.hash_not;
        filter.hash_gt = self.hash_gt;
        filter.hash_lt = self.hash_lt;
        filter.hash_gte = self.hash_gte;
        filter.hash_lte = self.hash_lte;
        filter.hash_in = self.hash_in;
        filter.hash_not_in = self.hash_not_in;
        filter.hash_contains = self.hash_contains;
        filter.hash_not_contains = self.hash_not_contains;
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
        filter.x_not = self.x_not;
        filter.x_gt = self.x_gt;
        filter.x_lt = self.x_lt;
        filter.x_gte = self.x_gte;
        filter.x_lte = self.x_lte;
        filter.x_in = self.x_in;
        filter.x_not_in = self.x_not_in;
        filter.x_contains = self.x_contains;
        filter.x_not_contains = self.x_not_contains;
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
        filter.y_not = self.y_not;
        filter.y_gt = self.y_gt;
        filter.y_lt = self.y_lt;
        filter.y_gte = self.y_gte;
        filter.y_lte = self.y_lte;
        filter.y_in = self.y_in;
        filter.y_not_in = self.y_not_in;
        filter.y_contains = self.y_contains;
        filter.y_not_contains = self.y_not_contains;
    }
}
