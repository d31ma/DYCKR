use serde_json::{Value, json};
use crate::prepared::Prepared;


    pub fn prepared(prepared: Option<&crate::prepared::Prepared>, class: &str) -> Value {
        let Some(crate::prepared::Prepared::Annotated(module)) = prepared else {
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
