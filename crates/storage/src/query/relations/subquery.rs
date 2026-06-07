use sqlx::{Postgres, query_builder::Separated};

pub(super) fn push_sub_where_prefix<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
) {
    if *has_where {
        separated.push_unseparated(" and ");
    } else {
        separated.push_unseparated(" where ");
        *has_where = true;
    }
}

pub(super) fn push_sub_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(crate) fn push_sub_change_block_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    entity_type: &'static str,
    id_column: &'static str,
    number_gte: Option<i32>,
) {
    let Some(number_gte) = number_gte else {
        return;
    };

    push_sub_where_prefix(separated, has_where);
    separated
        .push_unseparated("exists (select 1 from entity_changes where entity_type = ")
        .push_bind_unseparated(entity_type)
        .push_unseparated(" and entity_id = ")
        .push_unseparated(id_column)
        .push_unseparated(" and block_number >= ")
        .push_bind_unseparated(number_gte)
        .push_unseparated(")");
}

pub(super) fn push_sub_text_array_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<Vec<String>>,
    negate: bool,
) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        push_sub_where_prefix(separated, has_where);
        if negate {
            separated.push_unseparated("not (");
        }
        separated
            .push_unseparated(column)
            .push_unseparated(" = any(")
            .push_bind_unseparated(value)
            .push_unseparated(")");
        if negate {
            separated.push_unseparated(")");
        }
    }
}

pub(super) fn push_sub_text_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
    nocase: bool,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        if nocase {
            separated
                .push_unseparated("lower(")
                .push_unseparated(column)
                .push_unseparated(") like lower(")
                .push_bind_unseparated(format!("%{value}%"))
                .push_unseparated(")");
        } else {
            separated
                .push_unseparated(column)
                .push_unseparated(" like ")
                .push_bind_unseparated(format!("%{value}%"));
        }
    }
}

pub(super) fn push_sub_text_prefix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("{value}%"));
    }
}

pub(super) fn push_sub_text_suffix_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" like ")
            .push_bind_unseparated(format!("%{value}"));
    }
}

pub(super) fn push_sub_numeric_text_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric");
    }
}

pub(super) fn push_sub_i32_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<i32>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(super) fn push_sub_bool_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    op: &'static str,
    value: Option<bool>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" ")
            .push_unseparated(op)
            .push_unseparated(" ")
            .push_bind_unseparated(value);
    }
}

pub(super) fn push_sub_text_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("]::text[]");
    }
}

pub(super) fn push_sub_numeric_array_contains_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        push_sub_where_prefix(separated, has_where);
        separated
            .push_unseparated(column)
            .push_unseparated(" @> array[")
            .push_bind_unseparated(value)
            .push_unseparated("::numeric]");
    }
}
