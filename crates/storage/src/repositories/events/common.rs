use sqlx::{FromRow, Postgres, QueryBuilder, postgres::PgRow};

use crate::{error::*, filters::*, models::*, query::*};

use super::{
    EventsRepo,
    composition::{push_concrete_event_filter_group, push_event_ref_filter_group},
    event_filters::push_event_filters,
    specific_filters::push_event_specific_filters,
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
        push_concrete_event_filter_group(
            &mut separated,
            &mut has_where,
            table,
            parent_column,
            " and ",
            filter.and.clone(),
        );
        push_concrete_event_filter_group(
            &mut separated,
            &mut has_where,
            table,
            parent_column,
            " or ",
            filter.or.clone(),
        );
        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(event_order_column(order_by))
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
        push_event_ref_filter_group(
            &mut separated,
            &mut has_where,
            union_sql,
            " and ",
            filter.and.clone(),
        );
        push_event_ref_filter_group(
            &mut separated,
            &mut has_where,
            union_sql,
            " or ",
            filter.or.clone(),
        );
        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(event_ref_order_column(order_by))
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
        EventOrderField::DomainId => "parent_id",
        EventOrderField::DomainName => "(select d.name from domains d where d.id = parent_id)",
        EventOrderField::DomainLabelName => {
            "(select d.label_name from domains d where d.id = parent_id)"
        }
        EventOrderField::DomainLabelhash => {
            "(select d.labelhash from domains d where d.id = parent_id)"
        }
        EventOrderField::DomainSubdomainCount => {
            "(select d.subdomain_count from domains d where d.id = parent_id)"
        }
        EventOrderField::DomainTtl => "(select d.ttl from domains d where d.id = parent_id)",
        EventOrderField::DomainIsMigrated => {
            "(select d.is_migrated from domains d where d.id = parent_id)"
        }
        EventOrderField::DomainCreatedAt => {
            "(select d.created_at from domains d where d.id = parent_id)"
        }
        EventOrderField::DomainExpiryDate => {
            "(select d.expiry_date from domains d where d.id = parent_id)"
        }
        EventOrderField::RegistrationId => "parent_id",
        EventOrderField::RegistrationRegistrationDate => {
            "(select r.registration_date from registrations r where r.id = parent_id)"
        }
        EventOrderField::RegistrationExpiryDate => {
            "(select r.expiry_date from registrations r where r.id = parent_id)"
        }
        EventOrderField::RegistrationCost => {
            "(select r.cost from registrations r where r.id = parent_id)"
        }
        EventOrderField::RegistrationLabelName => {
            "(select r.label_name from registrations r where r.id = parent_id)"
        }
        EventOrderField::ResolverId => "parent_id",
        EventOrderField::ResolverAddress => {
            "(select r.address from resolvers r where r.id = parent_id)"
        }
        EventOrderField::ResolverContentHash => {
            "(select r.content_hash from resolvers r where r.id = parent_id)"
        }
        _ => event_order_column(order_by),
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::*;

    #[test]
    fn concrete_events_use_table_specific_parent_order_columns() {
        let mut query = QueryBuilder::<Postgres>::new("select id from name_registered_events");
        query
            .push(" order by ")
            .push(event_order_column(EventOrderField::RegistrationExpiryDate))
            .push(" asc");

        assert_eq!(
            query.build().sql(),
            "select id from name_registered_events order by (select r.expiry_date from registrations r where r.id = registration_id) asc"
        );
    }

    #[test]
    fn event_refs_use_union_parent_order_columns() {
        let mut query = QueryBuilder::<Postgres>::new("select id from event_refs");
        query
            .push(" order by ")
            .push(event_ref_order_column(
                EventOrderField::RegistrationExpiryDate,
            ))
            .push(" asc");

        assert_eq!(
            query.build().sql(),
            "select id from event_refs order by (select r.expiry_date from registrations r where r.id = parent_id) asc"
        );
    }
}
