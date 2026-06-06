use sqlx::{Postgres, query_builder::Separated};

pub(crate) fn push_where_prefix<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
) {
    if !*has_where {
        separated.push_unseparated(" where ");
        *has_where = true;
    }
}

pub(crate) fn push_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" = ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_not_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" != ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated(")");
    }
}

pub(crate) fn push_text_not_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        push_where_prefix(separated, has_where);
        separated
            .push("not (")
            .push_unseparated(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated("))");
    }
}

pub(crate) fn push_text_comparison_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    gt: Option<String>,
    lt: Option<String>,
    gte: Option<String>,
    lte: Option<String>,
) {
    push_text_order_filter(separated, has_where, column, ">", gt);
    push_text_order_filter(separated, has_where, column, "<", lt);
    push_text_order_filter(separated, has_where, column, ">=", gte);
    push_text_order_filter(separated, has_where, column, "<=", lte);
}

fn push_text_order_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        if nocase {
            separated
                .push("lower(")
                .push_unseparated(column)
                .push_unseparated(") like lower(")
                .push_bind_unseparated(format!("%{value}%"))
                .push_unseparated(")");
        } else {
            separated
                .push(column)
                .push_unseparated(" like ")
                .push_bind_unseparated(format!("%{value}%"));
        }
    }
}

pub(crate) fn push_text_not_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("%{value}%"), nocase, true);
    }
}

pub(crate) fn push_text_prefix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("{value}%"), false, false);
    }
}

pub(crate) fn push_text_not_prefix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("{value}%"), nocase, true);
    }
}

pub(crate) fn push_text_prefix_nocase_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("{value}%"), true, false);
    }
}

pub(crate) fn push_text_suffix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("%{value}"), false, false);
    }
}

pub(crate) fn push_text_not_suffix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("%{value}"), nocase, true);
    }
}

pub(crate) fn push_text_suffix_nocase_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        push_text_like_predicate(separated, column, format!("%{value}"), true, false);
    }
}

fn push_text_like_predicate<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    column: &'static str,
    pattern: String,
    nocase: bool,
    negate: bool,
) {
    if negate {
        separated.push("not (");
    }

    if nocase {
        if negate {
            separated.push_unseparated("lower(");
        } else {
            separated.push("lower(");
        }
        separated
            .push_unseparated(column)
            .push_unseparated(") like lower(")
            .push_bind_unseparated(pattern)
            .push_unseparated(")");
    } else {
        if negate {
            separated.push_unseparated(column);
        } else {
            separated.push(column);
        }
        separated
            .push_unseparated(" like ")
            .push_bind_unseparated(pattern);
    }

    if negate {
        separated.push_unseparated(")");
    }
}

pub(crate) fn push_numeric_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric");
    }
}

pub(crate) fn push_i32_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<i32>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_bool_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<bool>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_text_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("]::text[]");
    }
}

pub(crate) fn push_numeric_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric]");
    }
}
