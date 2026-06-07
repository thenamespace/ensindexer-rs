use sqlx::{Execute, Postgres, QueryBuilder};

use crate::{
    filters::{AccountFilter, DomainFilter},
    query::{
        push_account_filters, push_account_relation_filter, push_domain_filter_group,
        push_domain_relation_filter, push_i32_array_filter, push_numeric_element_filter,
        push_numeric_text_array_filter, push_numeric_text_filter, push_text_element_filter,
        push_text_filter, push_text_not_contains_filter, push_text_not_prefix_filter,
        push_text_prefix_nocase_filter,
    },
};

mod relations;
mod scalars;
