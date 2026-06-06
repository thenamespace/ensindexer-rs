use sqlx::PgPool;

mod columns;
mod common;
mod composition;
mod domain;
mod event_filters;
mod event_sql;
mod insert_domain;
mod insert_registration;
mod insert_resolver;
mod refs;
mod registration;
mod resolver;
mod specific_filters;
mod text_fields;

pub struct EventsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}
