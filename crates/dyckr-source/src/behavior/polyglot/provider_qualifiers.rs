use crate::Failure;
use serde_json::Value;
use std::path::Path;

pub struct ProviderQualifiers;

impl ProviderQualifiers {
    pub fn read(path: &Path, source: &str, class: &str) -> Result<Value, Failure> {
        dyckr::provider_qualifiers::ProviderQualifiers::read_source(
            path,
            source,
            class,
            &|p, s, e| super::prepare(p, s, e),
        )
    }
}
