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
