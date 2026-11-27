use serde_json::{Value, json};
use crate::prepared::Prepared;

pub struct ProviderQualifiers;

impl ProviderQualifiers {
    pub fn read(prepared: Option<&Prepared>, class: &str) -> Value {
        let Some(Prepared::Annotated(module)) = prepared else {
            return json!({});
        };
        module
            .types
            .iter()
            .find(|ty| ty.name == class)
            .and_then(|ty| ty.policies.get("qualifiers"))
            .cloned()
            .unwrap_or_else(|| json!({}))
    }
}
