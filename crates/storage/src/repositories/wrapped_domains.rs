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
        push_text_filter(&mut separated, &mut has_where, "owner_id", filter.owner_id);
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "owner_id",
            filter.owner_filter,
        );
        push_text_filter(&mut separated, &mut has_where, "name", filter.name);
        push_text_not_filter(&mut separated, &mut has_where, "name", filter.name_not);
        push_text_comparison_filters(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_gt,
            filter.name_lt,
            filter.name_gte,
            filter.name_lte,
        );
        push_text_array_filter(&mut separated, &mut has_where, "name", filter.name_in);
        push_text_not_array_filter(&mut separated, &mut has_where, "name", filter.name_not_in);
        push_text_contains_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_contains,
            false,
        );
        push_text_contains_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_contains_nocase,
            true,
        );
        push_text_not_contains_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_not_contains,
            false,
        );
        push_text_not_contains_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_not_contains_nocase,
            true,
        );
        push_text_prefix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_starts_with,
        );
        push_text_prefix_nocase_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_starts_with_nocase,
        );
        push_text_not_prefix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_not_starts_with,
            false,
        );
        push_text_not_prefix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_not_starts_with_nocase,
            true,
        );
        push_text_suffix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_ends_with,
        );
        push_text_suffix_nocase_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_ends_with_nocase,
        );
        push_text_not_suffix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_not_ends_with,
            false,
        );
        push_text_not_suffix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_not_ends_with_nocase,
            true,
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
        push_i32_filter(&mut separated, &mut has_where, "fuses", "=", filter.fuses);
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
