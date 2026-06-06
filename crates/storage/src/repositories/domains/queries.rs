use sqlx::{Postgres, QueryBuilder};

use super::DomainsRepo;
use crate::{error::*, filters::*, models::*, query::*};

impl DomainsRepo<'_> {
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
        push_text_field_filters(
            &mut separated,
            &mut has_where,
            "label_name",
            TextFieldFilter {
                exact: filter.label_name,
                not: filter.label_name_not,
                gt: filter.label_name_gt,
                lt: filter.label_name_lt,
                gte: filter.label_name_gte,
                lte: filter.label_name_lte,
                in_values: filter.label_name_in,
                not_in: filter.label_name_not_in,
                contains: filter.label_name_contains,
                contains_nocase: filter.label_name_contains_nocase,
                not_contains: filter.label_name_not_contains,
                not_contains_nocase: filter.label_name_not_contains_nocase,
                starts_with: filter.label_name_starts_with,
                starts_with_nocase: filter.label_name_starts_with_nocase,
                not_starts_with: filter.label_name_not_starts_with,
                not_starts_with_nocase: filter.label_name_not_starts_with_nocase,
                ends_with: filter.label_name_ends_with,
                ends_with_nocase: filter.label_name_ends_with_nocase,
                not_ends_with: filter.label_name_not_ends_with,
                not_ends_with_nocase: filter.label_name_not_ends_with_nocase,
            },
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
