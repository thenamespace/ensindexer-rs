mod metadata;
mod records;

use async_graphql::MergedObject;

use self::{metadata::ResolverMetadataEventQueries, records::ResolverRecordEventQueries};

#[derive(Default, MergedObject)]
pub(crate) struct ResolverEventQueries(ResolverRecordEventQueries, ResolverMetadataEventQueries);
