use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

use self::filtering::push_resolver_filters;
use crate::{error::*, filters::*, models::*, query::resolver_order_column};

mod composition;
mod filtering;

pub struct ResolversRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl ResolversRepo<'_> {
    pub async fn create_if_missing(
        &self,
        id: &str,
        domain_id: &str,
        address: &str,
    ) -> StorageResult<bool> {
        let inserted = sqlx::query_scalar::<_, String>(
            r#"
            insert into resolvers (id, domain_id, address)
            values ($1, $2, $3)
            on conflict (id) do nothing
            returning id
            "#,
        )
        .bind(id)
        .bind(domain_id)
        .bind(address)
        .fetch_optional(self.pool)
        .await?;
        Ok(inserted.is_some())
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
        filter: ResolverFilter,
        order_by: ResolverOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<ResolverRow>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "select id, domain_id, address, addr_id, content_hash, texts, coin_types from resolvers",
        );
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_resolver_filters(&mut separated, &mut has_where, filter);

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
