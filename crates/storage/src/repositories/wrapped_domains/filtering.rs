use sqlx::{Postgres, query_builder::Separated};

use super::composition::push_wrapped_domain_filter_group;
use crate::{filters::WrappedDomainFilter, query::*};

pub(super) fn push_wrapped_domain_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    mut filter: WrappedDomainFilter,
) {
    push_text_filter(separated, has_where, "id", filter.id.take());
    push_text_not_filter(separated, has_where, "id", filter.id_not.take());
    push_text_comparison_filters(
        separated,
        has_where,
        "id",
        filter.id_gt.take(),
        filter.id_lt.take(),
        filter.id_gte.take(),
        filter.id_lte.take(),
    );
    push_text_array_filter(separated, has_where, "id", filter.id_in.take());
    push_text_not_array_filter(separated, has_where, "id", filter.id_not_in.take());
    let and_filters = filter.and.take();
    let or_filters = filter.or.take();
    push_text_field_filters(separated, has_where, "domain_id", domain_field(&mut filter));
    push_domain_relation_filter(
        separated,
        has_where,
        "domain_id",
        filter.domain_filter.take(),
    );
    push_text_field_filters(separated, has_where, "owner_id", owner_field(&mut filter));
    push_account_relation_filter(separated, has_where, "owner_id", filter.owner_filter.take());
    push_text_field_filters(separated, has_where, "name", name_field(&mut filter));
    push_numeric_filters(separated, has_where, filter);
    push_wrapped_domain_filter_group(separated, has_where, " and ", and_filters);
    push_wrapped_domain_filter_group(separated, has_where, " or ", or_filters);
}

fn domain_field(filter: &mut WrappedDomainFilter) -> TextFieldFilter {
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
    }
}

fn owner_field(filter: &mut WrappedDomainFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.owner_id.take(),
        not: filter.owner_id_not.take(),
        gt: filter.owner_id_gt.take(),
        lt: filter.owner_id_lt.take(),
        gte: filter.owner_id_gte.take(),
        lte: filter.owner_id_lte.take(),
        in_values: filter.owner_id_in.take(),
        not_in: filter.owner_id_not_in.take(),
        contains: filter.owner_id_contains.take(),
        contains_nocase: filter.owner_id_contains_nocase.take(),
        not_contains: filter.owner_id_not_contains.take(),
        not_contains_nocase: filter.owner_id_not_contains_nocase.take(),
        starts_with: filter.owner_id_starts_with.take(),
        starts_with_nocase: filter.owner_id_starts_with_nocase.take(),
        not_starts_with: filter.owner_id_not_starts_with.take(),
        not_starts_with_nocase: filter.owner_id_not_starts_with_nocase.take(),
        ends_with: filter.owner_id_ends_with.take(),
        ends_with_nocase: filter.owner_id_ends_with_nocase.take(),
        not_ends_with: filter.owner_id_not_ends_with.take(),
        not_ends_with_nocase: filter.owner_id_not_ends_with_nocase.take(),
    }
}

fn name_field(filter: &mut WrappedDomainFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.name.take(),
        not: filter.name_not.take(),
        gt: filter.name_gt.take(),
        lt: filter.name_lt.take(),
        gte: filter.name_gte.take(),
        lte: filter.name_lte.take(),
        in_values: filter.name_in.take(),
        not_in: filter.name_not_in.take(),
        contains: filter.name_contains.take(),
        contains_nocase: filter.name_contains_nocase.take(),
        not_contains: filter.name_not_contains.take(),
        not_contains_nocase: filter.name_not_contains_nocase.take(),
        starts_with: filter.name_starts_with.take(),
        starts_with_nocase: filter.name_starts_with_nocase.take(),
        not_starts_with: filter.name_not_starts_with.take(),
        not_starts_with_nocase: filter.name_not_starts_with_nocase.take(),
        ends_with: filter.name_ends_with.take(),
        ends_with_nocase: filter.name_ends_with_nocase.take(),
        not_ends_with: filter.name_not_ends_with.take(),
        not_ends_with_nocase: filter.name_not_ends_with_nocase.take(),
    }
}

fn push_numeric_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    filter: WrappedDomainFilter,
) {
    push_numeric_text_filter(separated, has_where, "expiry_date", "=", filter.expiry_date);
    push_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "!=",
        filter.expiry_date_not,
    );
    push_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        ">",
        filter.expiry_date_gt,
    );
    push_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "<",
        filter.expiry_date_lt,
    );
    push_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        ">=",
        filter.expiry_date_gte,
    );
    push_numeric_text_filter(
        separated,
        has_where,
        "expiry_date",
        "<=",
        filter.expiry_date_lte,
    );
    push_numeric_text_array_filter(
        separated,
        has_where,
        "expiry_date",
        filter.expiry_date_in,
        false,
    );
    push_numeric_text_array_filter(
        separated,
        has_where,
        "expiry_date",
        filter.expiry_date_not_in,
        true,
    );
    push_i32_filter(separated, has_where, "fuses", "=", filter.fuses);
    push_i32_filter(separated, has_where, "fuses", "!=", filter.fuses_not);
    push_i32_filter(separated, has_where, "fuses", ">", filter.fuses_gt);
    push_i32_filter(separated, has_where, "fuses", "<", filter.fuses_lt);
    push_i32_filter(separated, has_where, "fuses", ">=", filter.fuses_gte);
    push_i32_filter(separated, has_where, "fuses", "<=", filter.fuses_lte);
    push_i32_array_filter(separated, has_where, "fuses", filter.fuses_in, false);
    push_i32_array_filter(separated, has_where, "fuses", filter.fuses_not_in, true);
}

#[cfg(test)]
mod tests {
    use sqlx::{Execute, Postgres, QueryBuilder};

    use super::*;

    #[test]
    fn wrapped_domain_filters_support_boolean_composition() {
        let filter = WrappedDomainFilter {
            fuses_gte: Some(1),
            or: Some(vec![
                WrappedDomainFilter {
                    name_contains: Some("foo".to_string()),
                    ..Default::default()
                },
                WrappedDomainFilter {
                    expiry_date_gt: Some("100".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let mut query = QueryBuilder::<Postgres>::new("select id from wrapped_domains");
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_wrapped_domain_filters(&mut separated, &mut has_where, filter);

        assert_eq!(
            query.build().sql(),
            "select id from wrapped_domains where fuses >= $1 and (id in (select id from wrapped_domains where name like $2) or id in (select id from wrapped_domains where expiry_date > $3::numeric))"
        );
    }
}
