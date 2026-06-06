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

pub(crate) fn push_text_prefix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_where_prefix(separated, has_where);
        separated
            .push(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("{value}%"));
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
        separated
            .push(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("%{value}"));
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
