use async_graphql::{Context, Object, Result};
use storage::Storage;

use crate::meta::{BlockHeight, Meta, MetaBlock};

#[derive(Default)]
pub(crate) struct MetaQueries;

// Keep the object impl consistent with the other generated-style GraphQL roots;
// async-graphql expands these methods in a way clippy counts broadly.
#[allow(clippy::too_many_arguments)]
#[Object]
impl MetaQueries {
    #[graphql(name = "_meta")]
    async fn meta(&self, ctx: &Context<'_>, block: Option<BlockHeight>) -> Result<Meta> {
        let storage = ctx.data::<Storage>()?;
        let block_row = match block.unwrap_or_default() {
            BlockHeight {
                hash: Some(hash), ..
            } => storage.blocks().find_by_hash(&hash).await?,
            BlockHeight {
                number: Some(number),
                ..
            } => storage.blocks().find_by_number(number.into()).await?,
            BlockHeight {
                number_gte: Some(number),
                ..
            } => {
                storage
                    .blocks()
                    .find_latest_at_or_after(number.into())
                    .await?
            }
            BlockHeight { .. } => storage.blocks().find_latest().await?,
        };

        let block = match block_row {
            Some(block) => MetaBlock::try_from(block)?,
            None => MetaBlock {
                hash: None,
                number: 0,
                timestamp: None,
            },
        };

        Ok(Meta {
            block,
            deployment: "local-rust-ens-indexer".to_owned(),
            has_indexing_errors: false,
        })
    }
}
