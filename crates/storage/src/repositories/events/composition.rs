use sqlx::{Postgres, query_builder::Separated};

use super::{event_filters::push_event_filters, specific_filters::push_event_specific_filters};
use crate::{filters::EventFilter, query::push_where_prefix};

pub(super) fn push_concrete_event_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    parent_column: &'static str,
    joiner: &'static str,
    filters: Option<Vec<EventFilter>>,
) {
    push_event_filter_group(
        separated,
        has_where,
        EventFilterSource::Concrete {
            table,
            parent_column,
        },
        joiner,
        filters,
    );
}

pub(super) fn push_event_ref_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    union_sql: &'static str,
    joiner: &'static str,
    filters: Option<Vec<EventFilter>>,
) {
    push_event_filter_group(
        separated,
        has_where,
        EventFilterSource::Union { union_sql },
        joiner,
        filters,
    );
}

fn push_event_filter_group<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    source: EventFilterSource,
    joiner: &'static str,
    filters: Option<Vec<EventFilter>>,
) {
    let Some(filters) = filters else {
        return;
    };
    if filters.is_empty() {
        return;
    }

    push_where_prefix(separated, has_where);
    separated.push("(");
    for (index, filter) in filters.into_iter().enumerate() {
        if index > 0 {
            separated.push_unseparated(joiner);
        }
        push_event_filter_subquery(separated, source, filter);
    }
    separated.push_unseparated(")");
}

fn push_event_filter_subquery<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    source: EventFilterSource,
    mut filter: EventFilter,
) {
    let and_filters = filter.and.take();
    let or_filters = filter.or.take();
    separated.push_unseparated("id in (select id from ");
    match source {
        EventFilterSource::Concrete { table, .. } => separated.push_unseparated(table),
        EventFilterSource::Union { union_sql } => separated
            .push_unseparated("(")
            .push_unseparated(union_sql)
            .push_unseparated(") event_refs"),
    };

    separated.push_unseparated(" where true");
    let mut sub_has_where = true;
    match source {
        EventFilterSource::Concrete {
            table,
            parent_column,
        } => {
            push_event_filters(separated, &mut sub_has_where, parent_column, &filter);
            push_event_specific_filters(separated, &mut sub_has_where, table, &filter);
        }
        EventFilterSource::Union { .. } => {
            push_event_filters(separated, &mut sub_has_where, "parent_id", &filter);
        }
    }
    push_event_filter_group(separated, &mut sub_has_where, source, " and ", and_filters);
    push_event_filter_group(separated, &mut sub_has_where, source, " or ", or_filters);
    separated.push_unseparated(")");
}

#[derive(Clone, Copy)]
enum EventFilterSource {
    Concrete {
        table: &'static str,
        parent_column: &'static str,
    },
    Union {
        union_sql: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::*;

    #[test]
    fn concrete_event_filters_support_boolean_composition() {
        let filter = EventFilter {
            block_number_gte: Some(10),
            or: Some(vec![
                EventFilter {
                    name_contains: Some("foo".into()),
                    ..Default::default()
                },
                EventFilter {
                    fuses_gt: Some(32),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let mut query = QueryBuilder::<Postgres>::new("select id from name_wrapped_events");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "domain_id", &filter);
        push_event_specific_filters(
            &mut separated,
            &mut has_where,
            "name_wrapped_events",
            &filter,
        );
        push_concrete_event_filter_group(
            &mut separated,
            &mut has_where,
            "name_wrapped_events",
            "domain_id",
            " or ",
            filter.or,
        );

        assert_eq!(
            query.build().sql(),
            "select id from name_wrapped_events where block_number >= $1 and (id in (select id from name_wrapped_events where true and name like $2) or id in (select id from name_wrapped_events where true and fuses > $3))"
        );
    }

    #[test]
    fn event_ref_filters_support_boolean_composition() {
        const UNION_SQL: &str =
            "select id, block_number, transaction_id, domain_id as parent_id from transfer_events";
        let filter = EventFilter {
            transaction_id: Some("0xabc".into()),
            and: Some(vec![
                EventFilter {
                    parent_id: Some("0xdomain".into()),
                    ..Default::default()
                },
                EventFilter {
                    block_number_gt: Some(20),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let mut query = QueryBuilder::<Postgres>::new("select id from (");
        query.push(UNION_SQL).push(") event_refs");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "parent_id", &filter);
        push_event_ref_filter_group(
            &mut separated,
            &mut has_where,
            UNION_SQL,
            " and ",
            filter.and,
        );

        assert_eq!(
            query.build().sql(),
            "select id from (select id, block_number, transaction_id, domain_id as parent_id from transfer_events) event_refs where transaction_id = $1 and (id in (select id from (select id, block_number, transaction_id, domain_id as parent_id from transfer_events) event_refs where true and parent_id = $2) and id in (select id from (select id, block_number, transaction_id, domain_id as parent_id from transfer_events) event_refs where true and block_number > $3))"
        );
    }
}
