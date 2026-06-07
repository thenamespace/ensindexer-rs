mod derived_filters;
mod filter_fields;
mod mutations;
mod queries;

use sqlx::PgPool;

use crate::{error::*, models::DomainRow};

pub struct DomainsRepo<'a> {
    pub(crate) pool: &'a PgPool,
}

impl DomainsRepo<'_> {
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
}
