use crate::{error::*, filters::*, models::*};

use super::{EventsRepo, common::EventRefSource, event_sql::*};

impl EventsRepo<'_> {
    pub async fn list_domain_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            EventRefSource {
                union_sql: domain_event_ref_union_sql(),
                interface_table: "domain_event_refs",
            },
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_registration_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            EventRefSource {
                union_sql: registration_event_ref_union_sql(),
                interface_table: "registration_event_refs",
            },
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_resolver_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            EventRefSource {
                union_sql: resolver_event_ref_union_sql(),
                interface_table: "resolver_event_refs",
            },
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }
}
