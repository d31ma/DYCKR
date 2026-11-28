pub const NAMES: [&str; 3] = ["Check", "PageDefaults", "CacheKey"];

pub fn is_parameter_marker(name: &str) -> bool {
    let normalized = name.replace('_', "").to_ascii_lowercase();
    crate::parameter_markers::NAMES
        .iter()
        .any(|marker| marker.to_ascii_lowercase() == normalized)
}
