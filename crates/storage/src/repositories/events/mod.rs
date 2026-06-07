use sqlx::PgPool;

mod columns;
mod common;
mod composition;
mod derived_filters;
mod domain;
mod event_filters;
mod event_sql;
mod insert_domain;
mod insert_registration;
mod insert_resolver;
mod interface_filters;
mod refs;
mod registration;
mod relation_filters;
mod resolver;
mod specific_filters;
mod text_fields;

pub struct EventsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

pub(crate) use derived_filters::{
    push_domain_events_filter, push_registration_events_filter, push_resolver_events_filter,
};
