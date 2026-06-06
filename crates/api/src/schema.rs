use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Result, Schema};
use storage::Storage;

mod entities;
mod events_domain;
mod events_interface;
mod events_registration;
mod events_resolver;
mod meta;

use self::{
    entities::EntityQueries, events_domain::DomainEventQueries,
    events_interface::InterfaceEventQueries, events_registration::RegistrationEventQueries,
    events_resolver::ResolverEventQueries, meta::MetaQueries,
};
use crate::filters::{AggregationInterval, BlockChangedFilter};
use crate::meta::BlockHeight;

pub type EnsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(storage: Storage) -> EnsSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .register_input_type::<BlockChangedFilter>()
        .register_output_type::<AggregationInterval>()
        .data(storage)
        .finish()
}

pub fn build_schema_sdl() -> String {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .register_input_type::<BlockChangedFilter>()
        .register_output_type::<AggregationInterval>()
        .finish()
        .sdl()
}

#[derive(Default, MergedObject)]
pub struct QueryRoot(
    MetaQueries,
    EntityQueries,
    InterfaceEventQueries,
    DomainEventQueries,
    RegistrationEventQueries,
    ResolverEventQueries,
);

fn ensure_current_block(block: Option<BlockHeight>) -> Result<()> {
    if block.as_ref().is_none_or(BlockHeight::is_current) {
        Ok(())
    } else {
        Err(async_graphql::Error::new(
            "historical block queries are not implemented yet",
        ))
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};

    use super::QueryRoot;

    #[test]
    fn core_queries_expose_graph_node_compatibility_arguments() {
        let sdl = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
            .finish()
            .sdl();

        assert!(sdl.contains(
            "domain(id: ID!, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"
        ));
        assert!(sdl.contains("domains(first: Int, skip: Int, where: Domain_filter, orderBy: Domain_orderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"));
        assert!(sdl.contains(
            "transfer(id: ID!, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"
        ));
        assert!(sdl.contains("transfers(first: Int, skip: Int, where: Transfer_filter, orderBy: Transfer_orderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"));
        assert!(sdl.contains(
            "newTTL(id: ID!, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"
        ));
        assert!(sdl.contains("newTTLs(first: Int, skip: Int, where: NewTTL_filter, orderBy: NewTTL_orderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"));
        assert!(sdl.contains(
            "nameRegistered(id: ID!, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"
        ));
        assert!(sdl.contains("nameRegistereds(first: Int, skip: Int, where: NameRegistered_filter, orderBy: NameRegistered_orderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"));
        assert!(sdl.contains(
            "versionChanged(id: ID!, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"
        ));
        assert!(sdl.contains("versionChangeds(first: Int, skip: Int, where: VersionChanged_filter, orderBy: VersionChanged_orderBy, orderDirection: OrderDirection, block: Block_height, subgraphError: _SubgraphErrorPolicy_! = deny)"));
        assert!(sdl.contains("domain: String"));
        assert!(sdl.contains("registration: String"));
        assert!(sdl.contains("resolver: String"));
        assert!(sdl.contains("owner: String"));
        assert!(sdl.contains("enum _SubgraphErrorPolicy_"));
    }
}
