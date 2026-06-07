use sqlx::{Postgres, query_builder::Separated};

use super::composition::push_registration_filter_group;
use crate::{
    filters::RegistrationFilter, query::*, repositories::events::push_registration_events_filter,
};

pub(crate) fn push_registration_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: RegistrationFilter,
) {
    let remaining_filter = filter.clone();
    let group_filter = filter.clone();
    push_text_filter(separated, has_where, "id", filter.id);
    push_text_not_filter(separated, has_where, "id", filter.id_not);
    push_text_comparison_filters(
        separated,
        has_where,
        "id",
        filter.id_gt,
        filter.id_lt,
        filter.id_gte,
        filter.id_lte,
    );
    push_text_array_filter(separated, has_where, "id", filter.id_in);
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in);
    push_change_block_filter(
        separated,
        has_where,
        "Registration",
        "registrations.id",
        filter.change_block_number_gte,
    );
    push_registration_events_filter(separated, has_where, filter.events_filter.clone());
    push_registration_text_fields(separated, has_where, remaining_filter);
    push_registration_filter_group(separated, has_where, " and ", group_filter.and);
    push_registration_filter_group(separated, has_where, " or ", group_filter.or);
}

fn push_registration_text_fields<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: RegistrationFilter,
) {
    let numeric_filter = filter.clone();
    push_text_field_filters(
        separated,
        has_where,
        "domain_id",
        TextFieldFilter {
            exact: filter.domain_id,
            not: filter.domain_id_not,
            gt: filter.domain_id_gt,
            lt: filter.domain_id_lt,
            gte: filter.domain_id_gte,
            lte: filter.domain_id_lte,
            in_values: filter.domain_id_in,
            not_in: filter.domain_id_not_in,
            contains: filter.domain_id_contains,
            contains_nocase: filter.domain_id_contains_nocase,
            not_contains: filter.domain_id_not_contains,
            not_contains_nocase: filter.domain_id_not_contains_nocase,
            starts_with: filter.domain_id_starts_with,
            starts_with_nocase: filter.domain_id_starts_with_nocase,
            not_starts_with: filter.domain_id_not_starts_with,
            not_starts_with_nocase: filter.domain_id_not_starts_with_nocase,
            ends_with: filter.domain_id_ends_with,
            ends_with_nocase: filter.domain_id_ends_with_nocase,
            not_ends_with: filter.domain_id_not_ends_with,
            not_ends_with_nocase: filter.domain_id_not_ends_with_nocase,
        },
    );
    push_domain_relation_filter(separated, has_where, "domain_id", filter.domain_filter);
    push_text_field_filters(
        separated,
        has_where,
        "registrant_id",
        TextFieldFilter {
            exact: filter.registrant_id,
            not: filter.registrant_id_not,
            gt: filter.registrant_id_gt,
            lt: filter.registrant_id_lt,
            gte: filter.registrant_id_gte,
            lte: filter.registrant_id_lte,
            in_values: filter.registrant_id_in,
            not_in: filter.registrant_id_not_in,
            contains: filter.registrant_id_contains,
            contains_nocase: filter.registrant_id_contains_nocase,
            not_contains: filter.registrant_id_not_contains,
            not_contains_nocase: filter.registrant_id_not_contains_nocase,
            starts_with: filter.registrant_id_starts_with,
            starts_with_nocase: filter.registrant_id_starts_with_nocase,
            not_starts_with: filter.registrant_id_not_starts_with,
            not_starts_with_nocase: filter.registrant_id_not_starts_with_nocase,
            ends_with: filter.registrant_id_ends_with,
            ends_with_nocase: filter.registrant_id_ends_with_nocase,
            not_ends_with: filter.registrant_id_not_ends_with,
            not_ends_with_nocase: filter.registrant_id_not_ends_with_nocase,
        },
    );
    push_account_relation_filter(
        separated,
        has_where,
        "registrant_id",
        filter.registrant_filter,
    );
    push_text_field_filters(
        separated,
        has_where,
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
    push_registration_numeric_fields(separated, has_where, numeric_filter);
}

fn push_registration_numeric_fields<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: RegistrationFilter,
) {
    push_numeric_field(
        separated,
        has_where,
        "registration_date",
        NumericTextFilter {
            exact: filter.registration_date,
            not: filter.registration_date_not,
            gt: filter.registration_date_gt,
            lt: filter.registration_date_lt,
            gte: filter.registration_date_gte,
            lte: filter.registration_date_lte,
            in_values: filter.registration_date_in,
            not_in: filter.registration_date_not_in,
        },
    );
    push_numeric_field(
        separated,
        has_where,
        "expiry_date",
        NumericTextFilter {
            exact: filter.expiry_date,
            not: filter.expiry_date_not,
            gt: filter.expiry_date_gt,
            lt: filter.expiry_date_lt,
            gte: filter.expiry_date_gte,
            lte: filter.expiry_date_lte,
            in_values: filter.expiry_date_in,
            not_in: filter.expiry_date_not_in,
        },
    );
    push_numeric_field(
        separated,
        has_where,
        "cost",
        NumericTextFilter {
            exact: filter.cost,
            not: filter.cost_not,
            gt: filter.cost_gt,
            lt: filter.cost_lt,
            gte: filter.cost_gte,
            lte: filter.cost_lte,
            in_values: filter.cost_in,
            not_in: filter.cost_not_in,
        },
    );
}

struct NumericTextFilter {
    exact: Option<String>,
    not: Option<String>,
    gt: Option<String>,
    lt: Option<String>,
    gte: Option<String>,
    lte: Option<String>,
    in_values: Option<Vec<String>>,
    not_in: Option<Vec<String>>,
}

fn push_numeric_field<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: NumericTextFilter,
) {
    push_numeric_text_filter(separated, has_where, column, "=", filter.exact);
    push_numeric_text_filter(separated, has_where, column, "!=", filter.not);
    push_numeric_text_filter(separated, has_where, column, ">", filter.gt);
    push_numeric_text_filter(separated, has_where, column, "<", filter.lt);
    push_numeric_text_filter(separated, has_where, column, ">=", filter.gte);
    push_numeric_text_filter(separated, has_where, column, "<=", filter.lte);
    push_numeric_text_array_filter(separated, has_where, column, filter.in_values, false);
    push_numeric_text_array_filter(separated, has_where, column, filter.not_in, true);
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use crate::filters::{AccountFilter, DomainFilter};

    use super::*;

    #[test]
    fn registration_filters_support_boolean_composition() {
        let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_registration_filters(
                &mut separated,
                &mut has_where,
                RegistrationFilter {
                    cost_gte: Some("1".into()),
                    or: Some(vec![
                        RegistrationFilter {
                            label_name_contains: Some("foo".into()),
                            ..RegistrationFilter::default()
                        },
                        RegistrationFilter {
                            expiry_date_gt: Some("100".into()),
                            ..RegistrationFilter::default()
                        },
                    ]),
                    ..RegistrationFilter::default()
                },
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from registrations where cost >= $1::numeric and (id in (select id from registrations where label_name like $2) or id in (select id from registrations where expiry_date > $3::numeric)) "
        );
    }

    #[test]
    fn registration_composition_supports_nested_relationship_filters() {
        let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_registration_filters(
                &mut separated,
                &mut has_where,
                RegistrationFilter {
                    and: Some(vec![
                        RegistrationFilter {
                            domain_filter: Some(Box::new(DomainFilter {
                                parent_filter: Some(Box::new(DomainFilter {
                                    name: Some("eth".into()),
                                    ..DomainFilter::default()
                                })),
                                ..DomainFilter::default()
                            })),
                            ..RegistrationFilter::default()
                        },
                        RegistrationFilter {
                            registrant_filter: Some(Box::new(AccountFilter {
                                id: Some("0xregistrant".into()),
                                ..AccountFilter::default()
                            })),
                            ..RegistrationFilter::default()
                        },
                    ]),
                    ..RegistrationFilter::default()
                },
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from registrations where (id in (select id from registrations where domain_id in (select id from domains where parent_id in (select id from domains where name = $1))) and id in (select id from registrations where registrant_id in (select id from accounts where id = $2))) "
        );
    }

    #[test]
    fn registration_filters_support_change_block_predicate() {
        let mut query = QueryBuilder::<Postgres>::new("select id from registrations");
        {
            let mut separated = query.separated(" and ");
            let mut has_where = false;
            push_registration_filters(
                &mut separated,
                &mut has_where,
                RegistrationFilter {
                    change_block_number_gte: Some(100),
                    ..RegistrationFilter::default()
                },
            );
            separated.push_unseparated(" ");
        }

        let built = query.build();
        assert_eq!(
            built.sql(),
            "select id from registrations where exists (select 1 from entity_changes where entity_type = $1 and entity_id = registrations.id and block_number >= $2) "
        );
    }
}
