pub fn equality_predicate(d: &str, value: &str) -> bool {
    d == value
}

pub fn starts_with_predicate(d: &str, value: &str) -> bool {
    d.starts_with(value)
}
