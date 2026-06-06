use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{error::*, filters::*, models::*, query::*};

pub struct ResolversRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl ResolversRepo<'_> {
    pub async fn create_if_missing(
        &self,
        id: &str,
        domain_id: &str,
        address: &str,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into resolvers (id, domain_id, address)
            values ($1, $2, $3)
            on conflict (id) do nothing
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(address)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_addr(&self, id: &str, addr_id: &str) -> StorageResult<()> {
        sqlx::query("update resolvers set addr_id = $2 where id = $1")
            .bind(id)
            .bind(addr_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_coin_type(&self, id: &str, coin_type: BigDecimal) -> StorageResult<()> {
        sqlx::query(
            r#"
            update resolvers
            set coin_types = case
                when $2 = any(coin_types) then coin_types
                else array_append(coin_types, $2)
            end
            where id = $1
            "#,
        )
        .bind(id)
        .bind(coin_type)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_text(&self, id: &str, key: &str) -> StorageResult<()> {
        sqlx::query(
            r#"
            update resolvers
            set texts = case
                when $2 = any(texts) then texts
                else array_append(texts, $2)
            end
            where id = $1
            "#,
        )
        .bind(id)
        .bind(key)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_content_hash(&self, id: &str, content_hash: &str) -> StorageResult<()> {
        sqlx::query("update resolvers set content_hash = $2 where id = $1")
            .bind(id)
            .bind(content_hash)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn reset_records(&self, id: &str) -> StorageResult<()> {
        sqlx::query(
            r#"
            update resolvers
            set addr_id = null,
                content_hash = null,
                texts = '{}',
                coin_types = '{}'
            where id = $1
            "#,
        )
        .bind(id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<ResolverRow>> {
        Ok(sqlx::query_as::<_, ResolverRow>(
            r#"
            select id, domain_id, address, addr_id, content_hash, texts, coin_types
            from resolvers
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?)
    }

    pub async fn list(&self, first: i64, skip: i64) -> StorageResult<Vec<ResolverRow>> {
        self.list_filtered(
            first,
            skip,
            ResolverFilter::default(),
            ResolverOrderField::Id,
            OrderDirection::Asc,
        )
        .await
    }

    pub async fn list_filtered(
        &self,
        first: i64,
        skip: i64,
        mut filter: ResolverFilter,
        order_by: ResolverOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<ResolverRow>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "select id, domain_id, address, addr_id, content_hash, texts, coin_types from resolvers",
        );
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_text_filter(&mut separated, &mut has_where, "id", filter.id.take());
        push_text_not_filter(&mut separated, &mut has_where, "id", filter.id_not.take());
        push_text_comparison_filters(
            &mut separated,
            &mut has_where,
            "id",
            filter.id_gt.take(),
            filter.id_lt.take(),
            filter.id_gte.take(),
            filter.id_lte.take(),
        );
        push_text_array_filter(&mut separated, &mut has_where, "id", filter.id_in.take());
        push_text_not_array_filter(
            &mut separated,
            &mut has_where,
            "id",
            filter.id_not_in.take(),
        );
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "domain_id",
            TextFieldFilter {
                exact: filter.domain_id.take(),
                not: filter.domain_id_not.take(),
                gt: filter.domain_id_gt.take(),
                lt: filter.domain_id_lt.take(),
                gte: filter.domain_id_gte.take(),
                lte: filter.domain_id_lte.take(),
                in_values: filter.domain_id_in.take(),
                not_in: filter.domain_id_not_in.take(),
                contains: filter.domain_id_contains.take(),
                contains_nocase: filter.domain_id_contains_nocase.take(),
                not_contains: filter.domain_id_not_contains.take(),
                not_contains_nocase: filter.domain_id_not_contains_nocase.take(),
                starts_with: filter.domain_id_starts_with.take(),
                starts_with_nocase: filter.domain_id_starts_with_nocase.take(),
                not_starts_with: filter.domain_id_not_starts_with.take(),
                not_starts_with_nocase: filter.domain_id_not_starts_with_nocase.take(),
                ends_with: filter.domain_id_ends_with.take(),
                ends_with_nocase: filter.domain_id_ends_with_nocase.take(),
                not_ends_with: filter.domain_id_not_ends_with.take(),
                not_ends_with_nocase: filter.domain_id_not_ends_with_nocase.take(),
            },
        );
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "domain_id",
            filter.domain_filter.take(),
        );
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "address",
            TextFieldFilter {
                exact: filter.address.take(),
                not: filter.address_not.take(),
                gt: filter.address_gt.take(),
                lt: filter.address_lt.take(),
                gte: filter.address_gte.take(),
                lte: filter.address_lte.take(),
                in_values: filter.address_in.take(),
                not_in: filter.address_not_in.take(),
                contains: filter.address_contains.take(),
                not_contains: filter.address_not_contains.take(),
                ..TextFieldFilter::default()
            },
        );
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "addr_id",
            TextFieldFilter {
                exact: filter.addr_id.take(),
                not: filter.addr_id_not.take(),
                gt: filter.addr_id_gt.take(),
                lt: filter.addr_id_lt.take(),
                gte: filter.addr_id_gte.take(),
                lte: filter.addr_id_lte.take(),
                in_values: filter.addr_id_in.take(),
                not_in: filter.addr_id_not_in.take(),
                contains: filter.addr_id_contains.take(),
                contains_nocase: filter.addr_id_contains_nocase.take(),
                not_contains: filter.addr_id_not_contains.take(),
                not_contains_nocase: filter.addr_id_not_contains_nocase.take(),
                starts_with: filter.addr_id_starts_with.take(),
                starts_with_nocase: filter.addr_id_starts_with_nocase.take(),
                not_starts_with: filter.addr_id_not_starts_with.take(),
                not_starts_with_nocase: filter.addr_id_not_starts_with_nocase.take(),
                ends_with: filter.addr_id_ends_with.take(),
                ends_with_nocase: filter.addr_id_ends_with_nocase.take(),
                not_ends_with: filter.addr_id_not_ends_with.take(),
                not_ends_with_nocase: filter.addr_id_not_ends_with_nocase.take(),
            },
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "addr_id",
            filter.addr_filter.take(),
        );
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "content_hash",
            TextFieldFilter {
                exact: filter.content_hash.take(),
                not: filter.content_hash_not.take(),
                gt: filter.content_hash_gt.take(),
                lt: filter.content_hash_lt.take(),
                gte: filter.content_hash_gte.take(),
                lte: filter.content_hash_lte.take(),
                in_values: filter.content_hash_in.take(),
                not_in: filter.content_hash_not_in.take(),
                contains: filter.content_hash_contains.take(),
                not_contains: filter.content_hash_not_contains.take(),
                ..TextFieldFilter::default()
            },
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            filter.texts.take(),
            false,
            false,
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            filter.texts_not.take(),
            false,
            true,
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            filter.texts_contains.take(),
            false,
            false,
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            filter.texts_contains_nocase.take(),
            true,
            false,
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            filter.texts_not_contains.take(),
            false,
            true,
        );
        push_text_element_filter(
            &mut separated,
            &mut has_where,
            "texts",
            filter.texts_not_contains_nocase.take(),
            true,
            true,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            filter.coin_types.take(),
            false,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            filter.coin_types_not.take(),
            true,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            filter.coin_types_contains.take(),
            false,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            filter.coin_types_contains_nocase.take(),
            false,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            filter.coin_types_not_contains.take(),
            true,
        );
        push_numeric_element_filter(
            &mut separated,
            &mut has_where,
            "coin_types",
            filter.coin_types_not_contains_nocase.take(),
            true,
        );

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(resolver_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id asc limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub async fn list_distinct_addresses(&self) -> StorageResult<Vec<String>> {
        Ok(sqlx::query_scalar::<_, String>(
            "select distinct address from resolvers order by address",
        )
        .fetch_all(self.pool)
        .await?)
    }
}
