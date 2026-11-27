pub fn without_bom(source: &str) -> &str {
    source.strip_prefix('\u{feff}').unwrap_or(source)
}
