use sqlx::{Postgres, query_builder::Separated};

use super::push_where_prefix;

pub(crate) fn push_bool_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    values: Option<Vec<bool>>,
    negate: bool,
) {
    if let Some(values) = values.filter(|values| !values.is_empty()) {
        push_where_prefix(separated, has_where);
        if negate {
            separated
                .push("not (")
                .push_unseparated(column)
                .push_unseparated(" = any(")
                .push_bind_unseparated(values)
                .push_unseparated("))");
        } else {
            separated
                .push(column)
                .push_unseparated(" = any(")
                .push_bind_unseparated(values)
                .push_unseparated(")");
        }
    }
}

pub(crate) fn push_text_element_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
    negate: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        if negate {
            separated.push("not (");
        }
        if nocase {
            if !negate {
                separated.push("exists (select 1 from unnest(");
            } else {
                separated.push_unseparated("exists (select 1 from unnest(");
            }
            separated
                .push_unseparated(column)
                .push_unseparated(") as value where lower(value) = lower(")
                .push_bind_unseparated(value)
                .push_unseparated("))");
        } else {
            if !negate {
                separated.push(column);
            } else {
                separated.push_unseparated(column);
            }
            separated
                .push_unseparated(" @> array[")
                .push_bind_unseparated(value)
                .push_unseparated("]::text[]");
        }
        if negate {
            separated.push_unseparated(")");
        }
    }
}

pub(crate) fn push_numeric_element_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    negate: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        if negate {
            separated.push("not (");
        }
        if !negate {
            separated.push(column);
        } else {
            separated.push_unseparated(column);
        }
        separated
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric]");
        if negate {
            separated.push_unseparated(")");
        }
    }
}
