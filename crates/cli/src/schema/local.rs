use std::collections::{BTreeMap, BTreeSet};

use api::build_schema_sdl;

use super::model::SchemaSummary;

pub(super) fn local_schema_summary() -> SchemaSummary {
    let sdl = build_schema_sdl();
    let (query_fields, query_arg_types) = parse_query_fields(&sdl);
    SchemaSummary {
        query_fields,
        query_arg_types,
        input_types: parse_named_blocks(&sdl, "input "),
        enum_types: parse_named_blocks(&sdl, "enum "),
    }
}

fn parse_query_fields(sdl: &str) -> (BTreeMap<String, BTreeSet<String>>, BTreeMap<String, String>) {
    let mut fields = BTreeMap::new();
    let mut arg_types = BTreeMap::new();
    let mut in_query = false;
    for line in sdl.lines() {
        let trimmed = line.trim();
        if trimmed == "type QueryRoot {" {
            in_query = true;
            continue;
        }
        if in_query && trimmed == "}" {
            break;
        }
        if !in_query || trimmed.is_empty() {
            continue;
        }
        let Some(name) = trimmed.split(['(', ':']).next() else {
            continue;
        };
        let (args, types) = parse_args(trimmed);
        for (arg, ty) in types {
            arg_types.insert(format!("{name}.{arg}"), ty);
        }
        fields.insert(name.to_owned(), args);
    }
    (fields, arg_types)
}

fn parse_named_blocks(sdl: &str, prefix: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut types = BTreeMap::new();
    let mut current_type = None::<String>;
    let mut current_fields = BTreeSet::new();

    for line in sdl.lines() {
        let trimmed = line.trim();
        if current_type.is_none() {
            if let Some(name) = trimmed.strip_prefix(prefix).and_then(first_token) {
                current_type = Some(name.to_owned());
                current_fields.clear();
            }
            continue;
        }

        if trimmed == "}" {
            if let Some(name) = current_type.take() {
                types.insert(name, std::mem::take(&mut current_fields));
            }
            continue;
        }

        if let Some(name) = trimmed.split([':', '(', ' ']).next()
            && !name.is_empty()
        {
            current_fields.insert(name.to_owned());
        }
    }

    types
}

fn parse_args(line: &str) -> (BTreeSet<String>, BTreeMap<String, String>) {
    let Some(args_start) = line.find('(') else {
        return (BTreeSet::new(), BTreeMap::new());
    };
    let Some(args_end) = line[args_start + 1..].find(')') else {
        return (BTreeSet::new(), BTreeMap::new());
    };
    let mut args = BTreeSet::new();
    let mut arg_types = BTreeMap::new();
    for arg in line[args_start + 1..args_start + 1 + args_end].split(',') {
        let mut parts = arg.trim().splitn(2, ':');
        let Some(name) = parts.next().filter(|name| !name.is_empty()) else {
            continue;
        };
        args.insert(name.to_owned());
        if let Some(ty) = parts.next() {
            let ty = ty.split('=').next().unwrap_or(ty);
            arg_types.insert(name.to_owned(), ty.trim().replace(' ', ""));
        }
    }
    (args, arg_types)
}

fn first_token(value: &str) -> Option<&str> {
    value
        .split([' ', '{'])
        .next()
        .filter(|name| !name.trim().is_empty())
}
