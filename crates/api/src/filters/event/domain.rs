use async_graphql::InputObject;

use super::{
    common::{ApplyEventFilter, BaseEventFilter, EventFilter},
    fields::{ExpiryDateFieldFilter, FusesFieldFilter, NameFieldFilter, TtlFieldFilter},
    relations::{
        DomainRelationFilter, NewResolverRelationFilter, OwnerRelationFilter,
        ParentDomainRelationFilter,
    },
};

macro_rules! domain_filter {
    ($name:ident, $graphql_name:literal, [$($field:ident : $ty:ty),* $(,)?]) => {
        #[derive(Debug, Clone, InputObject, Default)]
        #[graphql(name = $graphql_name)]
        pub struct $name {
            #[graphql(flatten)]
            base: BaseEventFilter,
            $(
                #[graphql(flatten)]
                $field: $ty,
            )*
            and: Option<Vec<$name>>,
            or: Option<Vec<$name>>,
        }

        impl From<$name> for EventFilter {
            fn from(value: $name) -> Self {
                let mut filter = EventFilter::default();
                value.base.apply(&mut filter);
                $(value.$field.apply(&mut filter);)*
                filter.and = value
                    .and
                    .map(|filters| filters.into_iter().map(EventFilter::from).collect());
                filter.or = value
                    .or
                    .map(|filters| filters.into_iter().map(EventFilter::from).collect());
                filter
            }
        }
    };
}

domain_filter!(DomainEventFilter, "DomainEvent_filter", [domain: DomainRelationFilter]);
domain_filter!(TransferFilter, "Transfer_filter", [
    domain: DomainRelationFilter,
    owner: OwnerRelationFilter,
]);
domain_filter!(NewOwnerFilter, "NewOwner_filter", [
    parent_domain: ParentDomainRelationFilter,
    domain: DomainRelationFilter,
    owner: OwnerRelationFilter,
]);
domain_filter!(NewResolverFilter, "NewResolver_filter", [
    domain: DomainRelationFilter,
    resolver: NewResolverRelationFilter,
]);
domain_filter!(NewTtlFilter, "NewTTL_filter", [
    domain: DomainRelationFilter,
    ttl: TtlFieldFilter,
]);
domain_filter!(WrappedTransferFilter, "WrappedTransfer_filter", [
    domain: DomainRelationFilter,
    owner: OwnerRelationFilter,
]);
domain_filter!(NameWrappedFilter, "NameWrapped_filter", [
    domain: DomainRelationFilter,
    name: NameFieldFilter,
    fuses: FusesFieldFilter,
    owner: OwnerRelationFilter,
    expiry_date: ExpiryDateFieldFilter,
]);
domain_filter!(NameUnwrappedFilter, "NameUnwrapped_filter", [
    domain: DomainRelationFilter,
    owner: OwnerRelationFilter,
]);
domain_filter!(FusesSetFilter, "FusesSet_filter", [
    domain: DomainRelationFilter,
    fuses: FusesFieldFilter,
]);
domain_filter!(ExpiryExtendedFilter, "ExpiryExtended_filter", [
    domain: DomainRelationFilter,
    expiry_date: ExpiryDateFieldFilter,
]);
