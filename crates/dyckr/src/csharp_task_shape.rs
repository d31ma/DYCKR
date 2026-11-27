use crate::sorted_json::SortedJson;
use serde_json::Value;

pub struct CsharpTaskShape;

impl CsharpTaskShape {
    pub const SCALARS: [&'static str; 4] = ["string", "integer", "number", "boolean"];
    pub const REFUSAL: &'static str = "C# browser Task model fields require scalars, one registered record of scalars, or one-level lists of registered scalar records; deeper nesting, recursive graphs, nested collections and arbitrary fields remain unsupported.";

    pub fn validate(root: &str, models: &Value) -> Result<(), String> {
        Self::root_fields(root, models)
    }

    pub fn root_fields(root: &str, models: &Value) -> Result<(), String> {
        let fields = Self::record_fields(&models[root])?;
        SortedJson::values(fields)
            .into_iter()
            .try_for_each(|field| Self::root_field(field, models))
    }

    pub fn root_field(field: &Value, models: &Value) -> Result<(), String> {
        let kind = Self::kind(field)?;
        if Self::SCALARS.contains(&kind) {
            return Ok(());
        }
        Self::nested(kind, models)
    }

    pub fn nested(kind: &str, models: &Value) -> Result<(), String> {
        let item = kind.strip_prefix("[]").unwrap_or(kind);
        let item = item.strip_prefix('?').unwrap_or(item);
        let fields = Self::record_fields(&models[item])?;
        SortedJson::values(fields)
            .into_iter()
            .try_for_each(Self::scalar_field)
    }

    pub fn record_fields(model: &Value) -> Result<&serde_json::Map<String, Value>, String> {
        if model["open"] != false {
            return Err(Self::REFUSAL.to_owned());
        }
        model["fields"]
            .as_object()
            .ok_or_else(|| Self::REFUSAL.to_owned())
    }

    pub fn scalar_field(field: &Value) -> Result<(), String> {
        Self::SCALARS
            .contains(&Self::kind(field)?)
            .then_some(())
            .ok_or_else(|| Self::REFUSAL.to_owned())
    }

    pub fn kind(field: &Value) -> Result<&str, String> {
        let kind = field.as_str().ok_or_else(|| Self::REFUSAL.to_owned())?;
        let kind = kind.strip_prefix('!').unwrap_or(kind);
        Ok(kind.strip_prefix('?').unwrap_or(kind))
    }
}
