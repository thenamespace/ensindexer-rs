use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{error::*, filters::*, inserts::*, models::*, query::*};

pub struct DomainsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl DomainsRepo<'_> {
    pub async fn create_if_missing(&self, input: DomainUpsert) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into domains (id, created_at, owner_id, is_migrated)
            values ($1, $2, $3, $4)
            on conflict (id) do nothing
            "#,
        )
        .bind(input.id)
        .bind(input.created_at)
        .bind(input.owner_id)
        .bind(input.is_migrated)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<DomainRow>> {
        Ok(sqlx::query_as::<_, DomainRow>(
            r#"
            select id, name, label_name, labelhash, parent_id, subdomain_count,
                   resolved_address_id, resolver_id, ttl, is_migrated, created_at,
                   owner_id, registrant_id, wrapped_owner_id, expiry_date
            from domains
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn set_owner(&self, id: &str, owner_id: &str) -> StorageResult<()> {
        sqlx::query("update domains set owner_id = $2 where id = $1")
            .bind(id)
            .bind(owner_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_parent_and_label(
        &self,
        id: &str,
        parent_id: &str,
        labelhash: &str,
        is_migrated: bool,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            update domains
            set parent_id = $2,
                labelhash = $3,
                is_migrated = $4
            where id = $1
            "#,
        )
        .bind(id)
        .bind(parent_id)
        .bind(labelhash)
        .bind(is_migrated)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_name_if_unknown(
        &self,
        id: &str,
        label_name: Option<&str>,
        name: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            update domains
            set label_name = coalesce(label_name, $2),
                name = coalesce(name, $3)
            where id = $1
            "#,
        )
        .bind(id)
        .bind(label_name)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_name(
        &self,
        id: &str,
        label_name: Option<&str>,
        name: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            update domains
            set label_name = $2,
                name = $3
            where id = $1
            "#,
        )
        .bind(id)
        .bind(label_name)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn increment_subdomain_count(&self, id: &str) -> StorageResult<()> {
        sqlx::query("update domains set subdomain_count = subdomain_count + 1 where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_resolver(
        &self,
        id: &str,
        resolver_id: Option<&str>,
        resolved_address_id: Option<&str>,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            update domains
            set resolver_id = $2,
                resolved_address_id = $3
            where id = $1
            "#,
        )
        .bind(id)
        .bind(resolver_id)
        .bind(resolved_address_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_ttl(&self, id: &str, ttl: BigDecimal) -> StorageResult<()> {
        sqlx::query("update domains set ttl = $2 where id = $1")
            .bind(id)
            .bind(ttl)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant_and_expiry(
        &self,
        id: &str,
        registrant_id: &str,
        expiry_date: BigDecimal,
    ) -> StorageResult<()> {
        sqlx::query("update domains set registrant_id = $2, expiry_date = $3 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .bind(expiry_date)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_registrant(&self, id: &str, registrant_id: &str) -> StorageResult<()> {
        sqlx::query("update domains set registrant_id = $2 where id = $1")
            .bind(id)
            .bind(registrant_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_wrapped_owner(&self, id: &str, wrapped_owner_id: &str) -> StorageResult<()> {
        sqlx::query("update domains set wrapped_owner_id = $2 where id = $1")
            .bind(id)
            .bind(wrapped_owner_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn clear_wrapped_owner(&self, id: &str) -> StorageResult<()> {
        sqlx::query("update domains set wrapped_owner_id = null where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn clear_expiry(&self, id: &str) -> StorageResult<()> {
        sqlx::query("update domains set expiry_date = null where id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_expiry_if_newer(
        &self,
        id: &str,
        expiry_date: BigDecimal,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            update domains
            set expiry_date = $2
            where id = $1 and (expiry_date is null or expiry_date < $2)
            "#,
        )
        .bind(id)
        .bind(expiry_date)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn list(&self, first: i64, skip: i64) -> StorageResult<Vec<DomainRow>> {
        self.list_filtered(
            first,
            skip,
            DomainFilter::default(),
            DomainOrderField::Id,
            OrderDirection::Asc,
        )
        .await
    }

    pub async fn list_filtered(
        &self,
        first: i64,
        skip: i64,
        filter: DomainFilter,
        order_by: DomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<DomainRow>> {
        let mut query = QueryBuilder::<Postgres>::new(domain_select_sql());
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_filter(&mut separated, &mut has_where, "id", filter.id);
        push_text_not_filter(&mut separated, &mut has_where, "id", filter.id_not);
        push_text_array_filter(&mut separated, &mut has_where, "id", filter.id_in);
        push_text_not_array_filter(&mut separated, &mut has_where, "id", filter.id_not_in);
        push_text_filter(&mut separated, &mut has_where, "name", filter.name);
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
        push_text_prefix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_starts_with,
        );
        push_text_suffix_filter(
            &mut separated,
            &mut has_where,
            "name",
            filter.name_ends_with,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name,
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
        push_text_prefix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_starts_with,
        );
        push_text_suffix_filter(
            &mut separated,
            &mut has_where,
            "label_name",
            filter.label_name_ends_with,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "labelhash",
            filter.labelhash,
        );
        push_text_not_filter(
            &mut separated,
            &mut has_where,
            "labelhash",
            filter.labelhash_not,
        );
        push_text_array_filter(
            &mut separated,
            &mut has_where,
            "labelhash",
            filter.labelhash_in,
        );
        push_text_not_array_filter(
            &mut separated,
            &mut has_where,
            "labelhash",
            filter.labelhash_not_in,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "parent_id",
            filter.parent_id,
        );
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "parent_id",
            filter.parent_filter,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            "=",
            filter.subdomain_count,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            ">",
            filter.subdomain_count_gt,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            "<",
            filter.subdomain_count_lt,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            ">=",
            filter.subdomain_count_gte,
        );
        push_i32_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            "<=",
            filter.subdomain_count_lte,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "resolved_address_id",
            filter.resolved_address_id,
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "resolved_address_id",
            filter.resolved_address_filter,
        );
        push_text_filter(&mut separated, &mut has_where, "owner_id", filter.owner_id);
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "owner_id",
            filter.owner_filter,
        );
        push_text_filter(
            &mut separated,
            &mut has_where,
            "resolver_id",
            filter.resolver_id,
        );
        push_resolver_relation_filter(
            &mut separated,
            &mut has_where,
            "resolver_id",
            filter.resolver_filter,
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
            "wrapped_owner_id",
            filter.wrapped_owner_id,
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "wrapped_owner_id",
            filter.wrapped_owner_filter,
        );
        push_bool_filter(
            &mut separated,
            &mut has_where,
            "is_migrated",
            "=",
            filter.is_migrated,
        );
        push_bool_filter(
            &mut separated,
            &mut has_where,
            "is_migrated",
            "!=",
            filter.is_migrated_not,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "=",
            filter.created_at,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            ">",
            filter.created_at_gt,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "<",
            filter.created_at_lt,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            ">=",
            filter.created_at_gte,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "<=",
            filter.created_at_lte,
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
        push_numeric_text_filter(&mut separated, &mut has_where, "ttl", "=", filter.ttl);
        push_numeric_text_filter(&mut separated, &mut has_where, "ttl", ">", filter.ttl_gt);
        push_numeric_text_filter(&mut separated, &mut has_where, "ttl", "<", filter.ttl_lt);
        push_numeric_text_filter(&mut separated, &mut has_where, "ttl", ">=", filter.ttl_gte);
        push_numeric_text_filter(&mut separated, &mut has_where, "ttl", "<=", filter.ttl_lte);

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(domain_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id asc limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub async fn list_by_parent(
        &self,
        parent_id: &str,
        first: i64,
        skip: i64,
    ) -> StorageResult<Vec<DomainRow>> {
        self.list_filtered(
            first,
            skip,
            DomainFilter {
                parent_id: Some(parent_id.to_owned()),
                ..DomainFilter::default()
            },
            DomainOrderField::Id,
            OrderDirection::Asc,
        )
        .await
    }
}
