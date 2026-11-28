use serde_json::{Value, json};
use crate::prepared::Prepared;
use crate::failure::Failure;
use std::path::Path;

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

    pub fn read_source(path: &Path, source: &str, class: &str, prepare: &crate::model_schema::Prepare) -> Result<Value, Failure> {
        let prepared = prepare(path, source, "tac")?;
        Ok(Self::read(prepared.as_ref(), class))
    }
}
