pub fn instance_name(class: &str) -> String {
    let stem = class.strip_suffix("Service").unwrap_or(class);
    let mut characters = stem.chars();
    let Some(first) = characters.next() else {
        return String::new();
    };
    first.to_ascii_lowercase().to_string() + characters.as_str()
}
