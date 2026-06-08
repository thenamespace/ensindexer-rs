use sqlx::{Postgres, QueryBuilder};

use super::DomainsRepo;
use super::derived_filters::push_domain_derived_relation_filters;
use super::filter_fields::{push_primary_text_fields, push_relation_id_text_fields};
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
        self.list_filtered_for_block(None, first, skip, filter, order_by, direction)
            .await
    }

    pub async fn list_filtered_at_block(
        &self,
        block_number: i32,
        first: i64,
        skip: i64,
        filter: DomainFilter,
        order_by: DomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<DomainRow>> {
        self.list_filtered_for_block(Some(block_number), first, skip, filter, order_by, direction)
            .await
    }

    async fn list_filtered_for_block(
        &self,
        block_number: Option<i32>,
        first: i64,
        skip: i64,
        mut filter: DomainFilter,
        order_by: DomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<DomainRow>> {
        if block_number.is_none()
            && let Some(lookup) = super::fast_address::detect_address_lookup_filter(&filter)
        {
            return self
                .list_for_address_fast(first, skip, lookup, order_by, direction)
                .await;
        }

        let mut query = QueryBuilder::<Postgres>::new("");
        if let Some(block_number) = block_number {
            push_historical_entity_ctes(&mut query, block_number);
        }
        query.push(domain_select_sql());
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
        push_primary_text_fields(&mut separated, &mut has_where, &mut filter);
        push_relation_id_text_fields(&mut separated, &mut has_where, &mut filter);
        push_domain_relation_filter(
            &mut separated,
            &mut has_where,
            "parent_id",
            filter.parent_filter.take(),
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
            "!=",
            filter.subdomain_count_not,
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
        push_i32_array_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            filter.subdomain_count_in,
            false,
        );
        push_i32_array_filter(
            &mut separated,
            &mut has_where,
            "subdomain_count",
            filter.subdomain_count_not_in,
            true,
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "resolved_address_id",
            filter.resolved_address_filter.take(),
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "owner_id",
            filter.owner_filter.take(),
        );
        push_resolver_relation_filter(
            &mut separated,
            &mut has_where,
            "resolver_id",
            filter.resolver_filter.take(),
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "registrant_id",
            filter.registrant_filter.take(),
        );
        push_account_relation_filter(
            &mut separated,
            &mut has_where,
            "wrapped_owner_id",
            filter.wrapped_owner_filter.take(),
        );
        push_bool_filter(
            &mut separated,
            &mut has_where,
            "is_migrated",
            "=",
            filter.is_migrated.take(),
        );
        push_bool_filter(
            &mut separated,
            &mut has_where,
            "is_migrated",
            "!=",
            filter.is_migrated_not.take(),
        );
        push_bool_array_filter(
            &mut separated,
            &mut has_where,
            "is_migrated",
            filter.is_migrated_in.take(),
            false,
        );
        push_bool_array_filter(
            &mut separated,
            &mut has_where,
            "is_migrated",
            filter.is_migrated_not_in.take(),
            true,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "=",
            filter.created_at.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "!=",
            filter.created_at_not.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            ">",
            filter.created_at_gt.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "<",
            filter.created_at_lt.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            ">=",
            filter.created_at_gte.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            "<=",
            filter.created_at_lte.take(),
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            filter.created_at_in.take(),
            false,
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "created_at",
            filter.created_at_not_in.take(),
            true,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "=",
            filter.expiry_date.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "!=",
            filter.expiry_date_not.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            ">",
            filter.expiry_date_gt.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "<",
            filter.expiry_date_lt.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            ">=",
            filter.expiry_date_gte.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            "<=",
            filter.expiry_date_lte.take(),
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            filter.expiry_date_in.take(),
            false,
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "expiry_date",
            filter.expiry_date_not_in.take(),
            true,
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            "=",
            filter.ttl.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            "!=",
            filter.ttl_not.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            ">",
            filter.ttl_gt.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            "<",
            filter.ttl_lt.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            ">=",
            filter.ttl_gte.take(),
        );
        push_numeric_text_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            "<=",
            filter.ttl_lte.take(),
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            filter.ttl_in.take(),
            false,
        );
        push_numeric_text_array_filter(
            &mut separated,
            &mut has_where,
            "ttl",
            filter.ttl_not_in.take(),
            true,
        );
        push_change_block_filter(
            &mut separated,
            &mut has_where,
            "Domain",
            "domains.id",
            filter.change_block_number_gte.take(),
        );
        push_domain_derived_relation_filters(
            &mut separated,
            &mut has_where,
            filter.events_filter.take(),
            filter.subdomains_filter.take(),
            filter.registration_filter.take(),
            filter.wrapped_domain_filter.take(),
        );
        push_domain_filter_group(&mut separated, &mut has_where, " and ", filter.and.take());
        push_domain_filter_group(&mut separated, &mut has_where, " or ", filter.or.take());

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
