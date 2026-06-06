use std::collections::{BTreeMap, BTreeSet};

pub(super) struct SchemaSummary {
    pub(super) query_fields: BTreeMap<String, BTreeSet<String>>,
    pub(super) query_arg_types: BTreeMap<String, String>,
    pub(super) input_types: BTreeMap<String, BTreeSet<String>>,
    pub(super) enum_types: BTreeMap<String, BTreeSet<String>>,
}

impl SchemaSummary {
    pub(super) fn diff(&self, official: &Self) -> SchemaDiff {
        SchemaDiff {
            missing_query_fields: missing_keys(&official.query_fields, &self.query_fields),
            extra_query_fields: missing_keys(&self.query_fields, &official.query_fields),
            missing_query_args: missing_members(&official.query_fields, &self.query_fields),
            extra_query_args: missing_members(&self.query_fields, &official.query_fields),
            mismatched_query_arg_types: mismatched_values(
                &official.query_arg_types,
                &self.query_arg_types,
            ),
            missing_input_types: missing_keys(&official.input_types, &self.input_types),
            extra_input_types: missing_keys(&self.input_types, &official.input_types),
            missing_input_fields: missing_members(&official.input_types, &self.input_types),
            extra_input_fields: missing_members(&self.input_types, &official.input_types),
            missing_enum_types: missing_keys(&official.enum_types, &self.enum_types),
            extra_enum_types: missing_keys(&self.enum_types, &official.enum_types),
            missing_enum_values: missing_members(&official.enum_types, &self.enum_types),
            extra_enum_values: missing_members(&self.enum_types, &official.enum_types),
        }
    }
}

pub(super) struct SchemaDiff {
    missing_query_fields: Vec<String>,
    extra_query_fields: Vec<String>,
    missing_query_args: Vec<String>,
    extra_query_args: Vec<String>,
    mismatched_query_arg_types: Vec<String>,
    missing_input_types: Vec<String>,
    extra_input_types: Vec<String>,
    missing_input_fields: Vec<String>,
    extra_input_fields: Vec<String>,
    missing_enum_types: Vec<String>,
    extra_enum_types: Vec<String>,
    missing_enum_values: Vec<String>,
    extra_enum_values: Vec<String>,
}

impl SchemaDiff {
    pub(super) fn has_missing(&self) -> bool {
        !self.missing_query_fields.is_empty()
            || !self.missing_query_args.is_empty()
            || !self.mismatched_query_arg_types.is_empty()
            || !self.missing_input_types.is_empty()
            || !self.missing_input_fields.is_empty()
            || !self.missing_enum_types.is_empty()
            || !self.missing_enum_values.is_empty()
    }

    pub(super) fn print(&self) {
        print_section("missing query fields", &self.missing_query_fields);
        print_section("extra query fields", &self.extra_query_fields);
        print_section("missing query args", &self.missing_query_args);
        print_section("extra query args", &self.extra_query_args);
        print_section(
            "mismatched query arg types",
            &self.mismatched_query_arg_types,
        );
        print_section("missing input types", &self.missing_input_types);
        print_section("extra input types", &self.extra_input_types);
        print_section("missing input fields", &self.missing_input_fields);
        print_section("extra input fields", &self.extra_input_fields);
        print_section("missing enum types", &self.missing_enum_types);
        print_section("extra enum types", &self.extra_enum_types);
        print_section("missing enum values", &self.missing_enum_values);
        print_section("extra enum values", &self.extra_enum_values);
    }
}

fn missing_keys(
    left: &BTreeMap<String, BTreeSet<String>>,
    right: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<String> {
    left.keys()
        .filter(|key| !right.contains_key(*key))
        .cloned()
        .collect()
}

fn missing_members(
    left: &BTreeMap<String, BTreeSet<String>>,
    right: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<String> {
    let mut missing = Vec::new();
    for (type_name, left_members) in left {
        let Some(right_members) = right.get(type_name) else {
            continue;
        };
        for member in left_members.difference(right_members) {
            missing.push(format!("{type_name}.{member}"));
        }
    }
    missing
}

fn mismatched_values(
    left: &BTreeMap<String, String>,
    right: &BTreeMap<String, String>,
) -> Vec<String> {
    left.iter()
        .filter_map(|(key, left_value)| {
            let right_value = right.get(key)?;
            (left_value != right_value)
                .then(|| format!("{key}: official={left_value} local={right_value}"))
        })
        .collect()
}

fn print_section(label: &str, values: &[String]) {
    println!("{label}: {}", values.len());
    for value in values.iter().take(40) {
        println!("  {value}");
    }
    if values.len() > 40 {
        println!("  ... {} more", values.len() - 40);
    }
}
