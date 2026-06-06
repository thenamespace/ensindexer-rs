use sqlx::{FromRow, Postgres, QueryBuilder, postgres::PgRow};

use crate::{error::*, filters::*, models::*, query::*};

use super::{
    EventsRepo,
    event_filters::{push_event_filters, push_event_specific_filters},
};

impl EventsRepo<'_> {
    pub(super) async fn find_event<T>(
        &self,
        table: &'static str,
        columns: &'static str,
        id: &str,
    ) -> StorageResult<Option<T>>
    where
        for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
    {
        let mut query = QueryBuilder::<Postgres>::new("select ");
        query
            .push(columns)
            .push(" from ")
            .push(table)
            .push(" where id = ")
            .push_bind(id);
        Ok(query.build_query_as().fetch_optional(self.pool).await?)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn list_events<T>(
        &self,
        table: &'static str,
        columns: &'static str,
        parent_column: &'static str,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<T>>
    where
        for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
    {
        let mut query = QueryBuilder::<Postgres>::new("select ");
        query.push(columns).push(" from ").push(table);

        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, parent_column, &filter);
        push_event_specific_filters(&mut separated, &mut has_where, table, &filter);
        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(event_ref_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(" limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub(super) async fn list_event_refs(
        &self,
        union_sql: &'static str,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "select kind, id, block_number, transaction_id, parent_id from (",
        );
        query.push(union_sql).push(") event_refs");

        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "parent_id", &filter);
        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(event_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id ")
            .push(direction.sql())
            .push(" limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }
}

fn event_ref_order_column(order_by: EventOrderField) -> &'static str {
    match order_by {
        EventOrderField::Domain | EventOrderField::Registration | EventOrderField::Resolver => {
            "parent_id"
        }
        _ => event_order_column(order_by),
    }
}
