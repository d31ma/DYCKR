
pub fn string_list(policies: &serde_json::Map<String, serde_json::Value>, key: &str) -> Vec<String> {
    let Some(values) = policies.get(key).and_then(serde_json::Value::as_array) else {
        return Vec::new();
    };
    values
        .iter()
        .filter_map(serde_json::Value::as_str)
        .map(str::to_owned)
        .collect()
}
