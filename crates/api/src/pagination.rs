pub fn normalize_first(first: Option<i32>) -> i64 {
    first.unwrap_or(100).clamp(0, 1_000) as i64
}

pub fn normalize_skip(skip: Option<i32>) -> i64 {
    skip.unwrap_or(0).max(0) as i64
}
