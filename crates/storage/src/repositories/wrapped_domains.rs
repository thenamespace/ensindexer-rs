use bigdecimal::BigDecimal;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub(crate) use self::composition::{
    push_wrapped_domain_subquery_filters, wrapped_domain_filter_has_conditions,
};
use self::filtering::push_wrapped_domain_filters;
use crate::{error::*, filters::*, models::*, query::wrapped_domain_order_column};

mod composition;
mod filtering;

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
        push_wrapped_domain_filters(&mut separated, &mut has_where, filter);

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
