#![allow(clippy::collapsible_if)]

use std::sync::{Arc, Mutex};

use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub(crate) use self::composition::{
    push_registration_subquery_filters, registration_filter_has_conditions,
};
pub(crate) use self::filtering::push_registration_filters;
use crate::{entity_cache::EntityCache, error::*, filters::*, models::*, query::*};

mod composition;
pub(super) mod filtering;

pub struct RegistrationsRepo<'a> {
    pub(crate) pool: &'a PgPool,
    pub(crate) entity_cache: Arc<Mutex<Option<EntityCache>>>,
}

impl RegistrationsRepo<'_> {
    pub async fn upsert_registered(
        &self,
        id: &str,
        domain_id: &str,
        registration_date: BigDecimal,
        expiry_date: BigDecimal,
        registrant_id: &str,
    ) -> StorageResult<()> {
        if self.cache_active()? {
            self.put_cached_registration(RegistrationRow {
                id: id.to_owned(),
                domain_id: domain_id.to_owned(),
                registration_date,
                expiry_date,
                cost: self.find_by_id(id).await?.and_then(|row| row.cost),
                registrant_id: registrant_id.to_owned(),
                label_name: self.find_by_id(id).await?.and_then(|row| row.label_name),
            })?;
            return Ok(());
        }
        sqlx::query(
            r#"
            insert into registrations (
                id, domain_id, registration_date, expiry_date, registrant_id
            )
            values ($1, $2, $3, $4, $5)
            on conflict (id) do update
            set domain_id = excluded.domain_id,
                registration_date = excluded.registration_date,
                expiry_date = excluded.expiry_date,
                registrant_id = excluded.registrant_id
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(registration_date)
        .bind(expiry_date)
        .bind(registrant_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<RegistrationRow>> {
        if let Some(row) = self.cached_registration(id)? {
            return Ok(row);
        }
        let row = self.find_by_id_uncached(id).await?;
        self.remember_registration(id, row.clone())?;
        Ok(row)
    }

    async fn find_by_id_uncached(&self, id: &str) -> StorageResult<Option<RegistrationRow>> {
        Ok(sqlx::query_as::<_, RegistrationRow>(
            r#"
            select id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name
            from registrations
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?)
    }

    fn cache_active(&self) -> StorageResult<bool> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache.is_some())
    }

    fn cached_registration(&self, id: &str) -> StorageResult<Option<Option<RegistrationRow>>> {
        let cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        Ok(cache
            .as_ref()
            .and_then(|active| active.get_registration(id)))
    }

    fn remember_registration(&self, id: &str, row: Option<RegistrationRow>) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.registrations.insert(id.to_owned(), row);
        }
        Ok(())
    }

    fn put_cached_registration(&self, row: RegistrationRow) -> StorageResult<()> {
        let mut cache = self
            .entity_cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        if let Some(active) = cache.as_mut() {
            active.put_registration(row);
        }
        Ok(())
    }

    pub async fn find_by_id_at_block(
        &self,
        id: &str,
        block_number: i32,
    ) -> StorageResult<Option<RegistrationRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        push_historical_entity_ctes(&mut query, block_number);
        query
            .push("select id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name from registrations")
            .push(" where id = ")
            .push_bind(id);
        Ok(query.build_query_as().fetch_optional(self.pool).await?)
    }

    pub async fn list(&self, first: i64, skip: i64) -> StorageResult<Vec<RegistrationRow>> {
        self.list_filtered(
            first,
            skip,
            RegistrationFilter::default(),
            RegistrationOrderField::Id,
            OrderDirection::Asc,
        )
        .await
    }

    pub async fn list_filtered(
        &self,
        first: i64,
        skip: i64,
        filter: RegistrationFilter,
        order_by: RegistrationOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<RegistrationRow>> {
        self.list_filtered_for_block(None, first, skip, filter, order_by, direction)
            .await
    }

    pub async fn list_filtered_at_block(
        &self,
        block_number: i32,
        first: i64,
        skip: i64,
        filter: RegistrationFilter,
        order_by: RegistrationOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<RegistrationRow>> {
        self.list_filtered_for_block(Some(block_number), first, skip, filter, order_by, direction)
            .await
    }

    async fn list_filtered_for_block(
        &self,
        block_number: Option<i32>,
        first: i64,
        skip: i64,
        filter: RegistrationFilter,
        order_by: RegistrationOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<RegistrationRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        if let Some(block_number) = block_number {
            push_historical_entity_ctes(&mut query, block_number);
        }
        query.push(
            "select id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name from registrations",
        );
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_registration_filters(&mut separated, &mut has_where, filter);

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(registration_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id asc limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub async fn find_by_domain_id(
        &self,
        domain_id: &str,
    ) -> StorageResult<Option<RegistrationRow>> {
        Ok(sqlx::query_as::<_, RegistrationRow>(
            r#"
            select id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name
            from registrations
            where domain_id = $1
            "#,
        )
        .bind(domain_id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn set_expiry(&self, id: &str, expiry_date: BigDecimal) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await? {
            if self.cache_active()? {
                row.expiry_date = expiry_date;
                return self.put_cached_registration(row);
            }
        }
        sqlx::query("update registrations set expiry_date = $2 where id = $1")
            .bind(id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant(&self, id: &str, registrant_id: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await? {
            if self.cache_active()? {
                row.registrant_id = registrant_id.to_owned();
                return self.put_cached_registration(row);
            }
        }
        sqlx::query("update registrations set registrant_id = $2 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_preimage(
        &self,
        id: &str,
        label_name: &str,
        cost: BigDecimal,
    ) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await? {
            if self.cache_active()? {
                row.label_name = Some(label_name.to_owned());
                row.cost = Some(cost);
                return self.put_cached_registration(row);
            }
        }
        sqlx::query("update registrations set label_name = $2, cost = $3 where id = $1")
            .bind(id)
            .bind(label_name)
            .bind(cost)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_label_name(&self, id: &str, label_name: &str) -> StorageResult<()> {
        if let Some(mut row) = self.find_by_id(id).await? {
            if self.cache_active()? {
                row.label_name = Some(label_name.to_owned());
                return self.put_cached_registration(row);
            }
        }
        sqlx::query("update registrations set label_name = $2 where id = $1")
            .bind(id)
            .bind(label_name)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
