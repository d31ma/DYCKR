pub fn valid_icon_source(source: &str) -> bool {
    let relative = source.trim_start_matches('/');
    if relative.is_empty() || source.starts_with("//") {
        return false;
    }
    if relative.contains(['\\', ':', '?', '#', '%']) {
        return false;
    }
    for segment in relative.split('/') {
        if matches!(segment, "" | "." | "..") {
            return false;
        }
    }
    true
}
