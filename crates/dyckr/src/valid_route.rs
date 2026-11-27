pub fn valid_route(value: &str) -> bool {
    if !value.starts_with('/') || value.len() > 2_048 {
        return false;
    }
    if value.contains('\\') || value.contains("//") {
        return false;
    }
    !value
        .split('/')
        .any(|segment| matches!(segment, "." | ".."))
}
