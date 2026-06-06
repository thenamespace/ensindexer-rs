use async_graphql::{Enum, InputObject};

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "BlockChangedFilter")]
pub struct BlockChangedFilter {
    #[graphql(name = "number_gte")]
    pub number_gte: Option<i32>,
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq)]
#[graphql(name = "Aggregation_interval")]
pub enum AggregationInterval {
    #[graphql(name = "hour")]
    Hour,
    #[graphql(name = "day")]
    Day,
}
