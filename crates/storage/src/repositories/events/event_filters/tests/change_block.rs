use super::*;

#[test]
fn concrete_event_filters_apply_change_block_number_gte() {
    let filter = EventFilter {
        change_block_number_gte: Some(100),
        ..EventFilter::default()
    };
    let mut query = QueryBuilder::<Postgres>::new("select id from transfer_events");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "domain_id", &filter);
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from transfer_events where block_number >= $1 "
    );
}

#[test]
fn event_interface_filters_apply_change_block_number_gte() {
    let filter = EventFilter {
        transaction_id: Some("0xtx".into()),
        change_block_number_gte: Some(200),
        ..EventFilter::default()
    };
    let mut query = QueryBuilder::<Postgres>::new("select id from domain_event_refs");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "parent_id", &filter);
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from domain_event_refs where transaction_id = $1 and block_number >= $2 "
    );
}
