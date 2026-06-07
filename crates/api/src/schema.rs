use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Result, Schema};
use storage::{
    AbiChangedEventRow, AddrChangedEventRow, AuthorisationChangedEventRow,
    ContenthashChangedEventRow, ExpiryExtendedEventRow, FusesSetEventRow, InterfaceChangedEventRow,
    MulticoinAddrChangedEventRow, NameChangedEventRow, NameRegisteredEventRow, NameRenewedEventRow,
    NameTransferredEventRow, NameUnwrappedEventRow, NameWrappedEventRow, NewOwnerEventRow,
    NewResolverEventRow, NewTtlEventRow, PubkeyChangedEventRow, Storage, TextChangedEventRow,
    TransferEventRow, VersionChangedEventRow, WrappedTransferEventRow,
};

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
use crate::filters::{AggregationCurrent, AggregationInterval, BlockChangedFilter};
use crate::meta::BlockHeight;

pub type EnsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(storage: Storage) -> EnsSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .register_input_type::<BlockChangedFilter>()
        .register_output_type::<AggregationCurrent>()
        .register_output_type::<AggregationInterval>()
        .data(storage)
        .finish()
}

pub fn build_schema_sdl() -> String {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .register_input_type::<BlockChangedFilter>()
        .register_output_type::<AggregationCurrent>()
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

async fn resolve_historical_block(
    storage: &Storage,
    block: Option<BlockHeight>,
) -> Result<Option<i32>> {
    let Some(block) = block.filter(|block| !block.is_current()) else {
        return Ok(None);
    };

    if let Some(number) = block.number {
        return Ok(Some(number));
    }
    if let Some(hash) = block.hash {
        let Some(row) = storage.blocks().find_by_hash(&hash).await? else {
            return Err(async_graphql::Error::new(format!(
                "unknown block hash: {hash}"
            )));
        };
        return Ok(Some(row.number.try_into()?));
    }
    if let Some(number_gte) = block.number_gte {
        let Some(row) = storage
            .blocks()
            .find_latest_at_or_after(i64::from(number_gte))
            .await?
        else {
            return Err(async_graphql::Error::new(format!(
                "no indexed block at or after {number_gte}"
            )));
        };
        return Ok(Some(row.number.try_into()?));
    }

    Ok(None)
}

fn with_event_block(
    mut filter: storage::EventFilter,
    block_number: Option<i32>,
) -> storage::EventFilter {
    if let Some(block_number) = block_number {
        filter.block_number_lte = Some(
            filter
                .block_number_lte
                .map_or(block_number, |existing| existing.min(block_number)),
        );
    }
    filter
}

trait HasBlockNumber {
    fn block_number(&self) -> i32;
}

macro_rules! impl_has_block_number {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl HasBlockNumber for $ty {
                fn block_number(&self) -> i32 {
                    self.block_number
                }
            }
        )+
    };
}

impl_has_block_number!(
    TransferEventRow,
    NewOwnerEventRow,
    NewResolverEventRow,
    NewTtlEventRow,
    WrappedTransferEventRow,
    NameWrappedEventRow,
    NameUnwrappedEventRow,
    FusesSetEventRow,
    ExpiryExtendedEventRow,
    NameRegisteredEventRow,
    NameRenewedEventRow,
    NameTransferredEventRow,
    AddrChangedEventRow,
    MulticoinAddrChangedEventRow,
    NameChangedEventRow,
    AbiChangedEventRow,
    PubkeyChangedEventRow,
    TextChangedEventRow,
    ContenthashChangedEventRow,
    InterfaceChangedEventRow,
    AuthorisationChangedEventRow,
    VersionChangedEventRow,
);

fn visible_at_block<T: HasBlockNumber>(row: Option<T>, block_number: Option<i32>) -> Option<T> {
    row.filter(|row| block_number.is_none_or(|block_number| row.block_number() <= block_number))
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use storage::{EventFilter, TransferEventRow};

    use super::{QueryRoot, visible_at_block, with_event_block};

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

    #[test]
    fn event_block_filter_clamps_existing_upper_bound() {
        let filter = with_event_block(
            EventFilter {
                block_number_lte: Some(50),
                ..Default::default()
            },
            Some(40),
        );

        assert_eq!(filter.block_number_lte, Some(40));

        let filter = with_event_block(
            EventFilter {
                block_number_lte: Some(30),
                ..Default::default()
            },
            Some(40),
        );

        assert_eq!(filter.block_number_lte, Some(30));
    }

    #[test]
    fn singular_event_rows_are_hidden_after_requested_block() {
        let row = TransferEventRow {
            id: "event-id".to_string(),
            domain_id: "domain-id".to_string(),
            block_number: 20,
            transaction_id: "tx".to_string(),
            owner_id: "owner".to_string(),
        };

        assert!(visible_at_block(Some(row.clone()), Some(19)).is_none());
        assert!(visible_at_block(Some(row.clone()), Some(20)).is_some());
        assert!(visible_at_block(Some(row), None).is_some());
    }
}
