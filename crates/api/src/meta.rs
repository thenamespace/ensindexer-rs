use async_graphql::{Enum, InputObject, SimpleObject};
use storage::BlockRow;

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "Block_height")]
pub struct BlockHeight {
    pub hash: Option<String>,
    pub number: Option<i32>,
    #[graphql(name = "number_gte")]
    pub number_gte: Option<i32>,
}

impl BlockHeight {
    pub fn is_current(&self) -> bool {
        self.hash.is_none() && self.number.is_none() && self.number_gte.is_none()
    }
}

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "_SubgraphErrorPolicy_")]
pub enum SubgraphErrorPolicy {
    #[default]
    #[graphql(name = "deny")]
    Deny,
    #[graphql(name = "allow")]
    Allow,
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(name = "_Block_")]
pub struct MetaBlock {
    pub hash: Option<String>,
    pub number: i32,
    pub timestamp: Option<i32>,
}

impl TryFrom<BlockRow> for MetaBlock {
    type Error = std::num::TryFromIntError;

    fn try_from(value: BlockRow) -> Result<Self, Self::Error> {
        Ok(Self {
            hash: Some(value.hash),
            number: value.number.try_into()?,
            timestamp: Some(value.timestamp.try_into()?),
        })
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(name = "_Meta_")]
pub struct Meta {
    pub block: MetaBlock,
    pub deployment: String,
    #[graphql(name = "hasIndexingErrors")]
    pub has_indexing_errors: bool,
}
