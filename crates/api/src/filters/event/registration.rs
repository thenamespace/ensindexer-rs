use async_graphql::InputObject;

use super::{
    common::{ApplyEventFilter, BaseEventFilter, EventFilter},
    fields::ExpiryDateFieldFilter,
    relations::{NewOwnerRelationFilter, RegistrantRelationFilter, RegistrationRelationFilter},
};

macro_rules! registration_filter {
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

registration_filter!(RegistrationEventFilter, "RegistrationEvent_filter", [
    registration: RegistrationRelationFilter,
]);
registration_filter!(NameRegisteredFilter, "NameRegistered_filter", [
    registration: RegistrationRelationFilter,
    registrant: RegistrantRelationFilter,
    expiry_date: ExpiryDateFieldFilter,
]);
registration_filter!(NameRenewedFilter, "NameRenewed_filter", [
    registration: RegistrationRelationFilter,
    expiry_date: ExpiryDateFieldFilter,
]);
registration_filter!(NameTransferredFilter, "NameTransferred_filter", [
    registration: RegistrationRelationFilter,
    new_owner: NewOwnerRelationFilter,
]);
