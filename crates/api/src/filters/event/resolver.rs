use async_graphql::InputObject;

use super::{
    common::{ApplyEventFilter, BaseEventFilter, EventFilter},
    fields::{
        AddrBytesFieldFilter, AuthOwnerFieldFilter, CoinTypeFieldFilter, ContentTypeFieldFilter,
        HashFieldFilter, ImplementerFieldFilter, InterfaceIdFieldFilter, IsAuthorizedFieldFilter,
        KeyFieldFilter, NameFieldFilter, TargetFieldFilter, ValueFieldFilter, VersionFieldFilter,
        XFieldFilter, YFieldFilter,
    },
    relations::{AddrAccountRelationFilter, ResolverRelationFilter},
};

macro_rules! resolver_filter {
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

resolver_filter!(ResolverEventFilter, "ResolverEvent_filter", [resolver: ResolverRelationFilter]);
resolver_filter!(AddrChangedFilter, "AddrChanged_filter", [
    resolver: ResolverRelationFilter,
    addr: AddrAccountRelationFilter,
]);
resolver_filter!(MulticoinAddrChangedFilter, "MulticoinAddrChanged_filter", [
    resolver: ResolverRelationFilter,
    coin_type: CoinTypeFieldFilter,
    addr: AddrBytesFieldFilter,
]);
resolver_filter!(NameChangedFilter, "NameChanged_filter", [
    resolver: ResolverRelationFilter,
    name: NameFieldFilter,
]);
resolver_filter!(AbiChangedFilter, "AbiChanged_filter", [
    resolver: ResolverRelationFilter,
    content_type: ContentTypeFieldFilter,
]);
resolver_filter!(PubkeyChangedFilter, "PubkeyChanged_filter", [
    resolver: ResolverRelationFilter,
    x: XFieldFilter,
    y: YFieldFilter,
]);
resolver_filter!(TextChangedFilter, "TextChanged_filter", [
    resolver: ResolverRelationFilter,
    key: KeyFieldFilter,
    value: ValueFieldFilter,
]);
resolver_filter!(ContenthashChangedFilter, "ContenthashChanged_filter", [
    resolver: ResolverRelationFilter,
    hash: HashFieldFilter,
]);
resolver_filter!(InterfaceChangedFilter, "InterfaceChanged_filter", [
    resolver: ResolverRelationFilter,
    interface_id: InterfaceIdFieldFilter,
    implementer: ImplementerFieldFilter,
]);
resolver_filter!(AuthorisationChangedFilter, "AuthorisationChanged_filter", [
    resolver: ResolverRelationFilter,
    owner: AuthOwnerFieldFilter,
    target: TargetFieldFilter,
    is_authorized: IsAuthorizedFieldFilter,
]);
resolver_filter!(VersionChangedFilter, "VersionChanged_filter", [
    resolver: ResolverRelationFilter,
    version: VersionFieldFilter,
]);
