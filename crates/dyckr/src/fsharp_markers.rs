pub fn is_tachyon_name(item: &str) -> &str {
    item.split(['(', ' ']).next().unwrap_or(item)
}
