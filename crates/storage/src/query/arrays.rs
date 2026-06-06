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
