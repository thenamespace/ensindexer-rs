use async_graphql::Enum;
use storage::OrderDirection as StorageOrderDirection;

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "OrderDirection")]
pub enum OrderDirection {
    #[default]
    #[graphql(name = "asc")]
    Asc,
    #[graphql(name = "desc")]
    Desc,
}

impl From<OrderDirection> for StorageOrderDirection {
    fn from(value: OrderDirection) -> Self {
        match value {
            OrderDirection::Asc => Self::Asc,
            OrderDirection::Desc => Self::Desc,
        }
    }
}
