use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{error::*, filters::*, models::*, query::*};

pub struct WrappedDomainsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl WrappedDomainsRepo<'_> {
    pub async fn upsert_full(
        &self,
        id: &str,
        domain_id: &str,
        expiry_date: BigDecimal,
        fuses: i32,
        owner_id: &str,
        name: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into wrapped_domains (id, domain_id, expiry_date, fuses, owner_id, name)
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do update
            set domain_id = excluded.domain_id,
                expiry_date = excluded.expiry_date,
                fuses = excluded.fuses,
                owner_id = excluded.owner_id,
                name = excluded.name
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(expiry_date)
        .bind(fuses)
        .bind(owner_id)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_transfer_placeholder(
        &self,
        id: &str,
        domain_id: &str,
        owner_id: &str,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into wrapped_domains (id, domain_id, expiry_date, fuses, owner_id)
            values ($1, $2, 0, 0, $3)
            on conflict (id) do update
            set owner_id = excluded.owner_id
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(owner_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<WrappedDomainRow>> {
        Ok(sqlx::query_as::<_, WrappedDomainRow>(
            r#"
            select id, domain_id, expiry_date, fuses, owner_id, name
            from wrapped_domains
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn find_by_domain_id(
        &self,
        domain_id: &str,
    ) -> StorageResult<Option<WrappedDomainRow>> {
        Ok(sqlx::query_as::<_, WrappedDomainRow>(
            r#"
            select id, domain_id, expiry_date, fuses, owner_id, name
            from wrapped_domains
            where domain_id = $1
            "#,
        )
        .bind(domain_id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn list(
        &self,
        first: i64,
        skip: i64,
        filter: WrappedDomainFilter,
        order_by: WrappedDomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<WrappedDomainRow>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "select id, domain_id, expiry_date, fuses, owner_id, name from wrapped_domains",
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
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "domain_id",
            TextFieldFilter {
                exact: filter.domain_id,
                not: filter.domain_id_not,
                gt: filter.domain_id_gt,
                lt: filter.domain_id_lt,
                gte: filter.domain_id_gte,
                lte: filter.domain_id_lte,
                in_values: filter.domain_id_in,
                not_in: filter.domain_id_not_in,
                contains: filter.domain_id_contains,
                contains_nocase: filter.domain_id_contains_nocase,
                not_contains: filter.domain_id_not_contains,
                not_contains_nocase: filter.domain_id_not_contains_nocase,
                starts_with: filter.domain_id_starts_with,
                starts_with_nocase: filter.domain_id_starts_with_nocase,
                not_starts_with: filter.domain_id_not_starts_with,
                not_starts_with_nocase: filter.domain_id_not_starts_with_nocase,
                ends_with: filter.domain_id_ends_with,
                ends_with_nocase: filter.domain_id_ends_with_nocase,
                not_ends_with: filter.domain_id_not_ends_with,
                not_ends_with_nocase: filter.domain_id_not_ends_with_nocase,
            },
        );
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            filter.domain_filter,
        );
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "owner_id",
            TextFieldFilter {
                exact: filter.owner_id,
                not: filter.owner_id_not,
                gt: filter.owner_id_gt,
                lt: filter.owner_id_lt,
                gte: filter.owner_id_gte,
                lte: filter.owner_id_lte,
                in_values: filter.owner_id_in,
                not_in: filter.owner_id_not_in,
                contains: filter.owner_id_contains,
                contains_nocase: filter.owner_id_contains_nocase,
                not_contains: filter.owner_id_not_contains,
                not_contains_nocase: filter.owner_id_not_contains_nocase,
                starts_with: filter.owner_id_starts_with,
                starts_with_nocase: filter.owner_id_starts_with_nocase,
                not_starts_with: filter.owner_id_not_starts_with,
                not_starts_with_nocase: filter.owner_id_not_starts_with_nocase,
                ends_with: filter.owner_id_ends_with,
                ends_with_nocase: filter.owner_id_ends_with_nocase,
                not_ends_with: filter.owner_id_not_ends_with,
                not_ends_with_nocase: filter.owner_id_not_ends_with_nocase,
            },
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "owner_id",
            filter.owner_filter,
        );
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "name",
            TextFieldFilter {
                exact: filter.name,
                not: filter.name_not,
                gt: filter.name_gt,
                lt: filter.name_lt,
                gte: filter.name_gte,
                lte: filter.name_lte,
                in_values: filter.name_in,
                not_in: filter.name_not_in,
                contains: filter.name_contains,
                contains_nocase: filter.name_contains_nocase,
                not_contains: filter.name_not_contains,
                not_contains_nocase: filter.name_not_contains_nocase,
                starts_with: filter.name_starts_with,
                starts_with_nocase: filter.name_starts_with_nocase,
                not_starts_with: filter.name_not_starts_with,
                not_starts_with_nocase: filter.name_not_starts_with_nocase,
                ends_with: filter.name_ends_with,
                ends_with_nocase: filter.name_ends_with_nocase,
                not_ends_with: filter.name_not_ends_with,
                not_ends_with_nocase: filter.name_not_ends_with_nocase,
            },
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
            "!=",
            filter.expiry_date_not,
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
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            filter.expiry_date_in,
            false,
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            filter.expiry_date_not_in,
            true,
        );
        push_i32_filter(&mut separated, &mut has_where, "fuses", "=", filter.fuses);
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            "!=",
            filter.fuses_not,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            ">",
            filter.fuses_gt,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            "<",
            filter.fuses_lt,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            ">=",
            filter.fuses_gte,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            "<=",
            filter.fuses_lte,
        );
        push_i32_array_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            filter.fuses_in,
            false,
        );
        push_i32_array_filter(
            &mut separated,
            &mut has_where,
            "fuses",
            filter.fuses_not_in,
            true,
        );

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(wrapped_domain_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id asc limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub async fn set_fuses(&self, id: &str, fuses: i32) -> StorageResult<()> {
        sqlx::query("update wrapped_domains set fuses = $2 where id = $1")
            .bind(id)
            .bind(fuses)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_expiry(&self, id: &str, expiry_date: BigDecimal) -> StorageResult<()> {
        sqlx::query("update wrapped_domains set expiry_date = $2 where id = $1")
            .bind(id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> StorageResult<()> {
        sqlx::query("delete from wrapped_domains where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
