use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{error::*, filters::*, models::*, query::*};

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

        push_text_filter(&mut separated, &mut has_where, "id", filter.id);
        push_text_not_filter(&mut separated, &mut has_where, "id", filter.id_not);
        push_text_comparison_filters(
            &mut separated,
            &mut has_where,
            "id",
            filter.id_gt,
            filter.id_lt,
            filter.id_gte,
            filter.id_lte,
        );
        push_text_array_filter(&mut separated, &mut has_where, "id", filter.id_in);
        push_text_not_array_filter(&mut separated, &mut has_where, "id", filter.id_not_in);
        push_text_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            filter.domain_id,
        );
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            filter.domain_filter,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "registrant_id",
            filter.registrant_id,
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "registrant_id",
            filter.registrant_filter,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name,
        );
        push_text_not_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not,
        );
        push_text_comparison_filters(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_gt,
            filter.label_name_lt,
            filter.label_name_gte,
            filter.label_name_lte,
        );
        push_text_array_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_in,
        );
        push_text_not_array_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_in,
        );
        push_text_contains_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_contains,
            false,
        );
        push_text_contains_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_contains_nocase,
            true,
        );
        push_text_not_contains_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_contains,
            false,
        );
        push_text_not_contains_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_contains_nocase,
            true,
        );
        push_text_prefix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_starts_with,
        );
        push_text_prefix_nocase_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_starts_with_nocase,
        );
        push_text_not_prefix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_starts_with,
            false,
        );
        push_text_not_prefix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_starts_with_nocase,
            true,
        );
        push_text_suffix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_ends_with,
        );
        push_text_suffix_nocase_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_ends_with_nocase,
        );
        push_text_not_suffix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_ends_with,
            false,
        );
        push_text_not_suffix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_not_ends_with_nocase,
            true,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "registration_date",
            "=",
            filter.registration_date,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "registration_date",
            ">",
            filter.registration_date_gt,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "registration_date",
            "<",
            filter.registration_date_lt,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "registration_date",
            ">=",
            filter.registration_date_gte,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "registration_date",
            "<=",
            filter.registration_date_lte,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "=",
            filter.expiry_date,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            ">",
            filter.expiry_date_gt,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "<",
            filter.expiry_date_lt,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            ">=",
            filter.expiry_date_gte,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "<=",
            filter.expiry_date_lte,
        );
        push_numeric_text_filter(&mut separated, &mut has_where, "cost", "=", filter.cost);
        push_numeric_text_filter(&mut separated, &mut has_where, "cost", ">", filter.cost_gt);
        push_numeric_text_filter(&mut separated, &mut has_where, "cost", "<", filter.cost_lt);
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "cost",
            ">=",
            filter.cost_gte,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "cost",
            "<=",
            filter.cost_lte,
        );

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
