use sqlx::PgPool;

mod columns;
mod common;
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

pub struct EventsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}
