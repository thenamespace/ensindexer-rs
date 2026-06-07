use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{error::*, filters::*, models::*, query::*};

pub struct AccountsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl AccountsRepo<'_> {
    pub async fn create_if_missing(&self, id: &str) -> StorageResult<bool> {
        let inserted = sqlx::query_scalar::<_, String>(
            "insert into accounts (id) values ($1) on conflict (id) do nothing returning id",
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(inserted.is_some())
    }

    pub async fn find_by_id(&self, id: &str) -> StorageResult<Option<AccountRow>> {
        Ok(
            sqlx::query_as::<_, AccountRow>("select id from accounts where id = $1")
                .bind(id)
                .fetch_optional(self.pool)
                .await?,
        )
    }

    pub async fn list(
        &self,
        first: i64,
        skip: i64,
        filter: AccountFilter,
        order_by: AccountOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AccountRow>> {
        let mut query = QueryBuilder::<Postgres>::new("select id from accounts");
        let mut separated = query.separated(" and ");
        let mut has_where = false;

        push_account_filters(&mut separated, &mut has_where, filter);

        if has_where {
            separated.push_unseparated(" ");
        }

        query
            .push(" order by ")
            .push(account_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(" limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }
}
