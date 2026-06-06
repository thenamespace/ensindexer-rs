use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

use self::filtering::push_registration_filters;
use crate::{error::*, filters::*, models::*, query::registration_order_column};

mod filtering;

pub struct RegistrationsRepo<'a> {
    pub(crate) pool: &'a PgPool,
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
        let mut query = QueryBuilder::<Postgres>::new(
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
        sqlx::query("update registrations set expiry_date = $2 where id = $1")
            .bind(id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant(&self, id: &str, registrant_id: &str) -> StorageResult<()> {
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
        sqlx::query("update registrations set label_name = $2, cost = $3 where id = $1")
            .bind(id)
            .bind(label_name)
            .bind(cost)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_label_name(&self, id: &str, label_name: &str) -> StorageResult<()> {
        sqlx::query("update registrations set label_name = $2 where id = $1")
            .bind(id)
            .bind(label_name)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
