use serde_json::{Map, Value};

pub struct Unit {
    pub name: String,
    pub configuration: String,
    pub partition: String,
    pub profiles: Value,
    pub properties: Value,
    pub cases: Vec<String>,
    pub inputs: Map<String, Value>,
    pub mocks: Vec<String>,
}

impl Unit {
    pub fn flat_settings(&self) -> Vec<(String, String)> {
        Self::flat_entries(&self.properties)
    }

    pub fn flat_properties(&self) -> Value {
        Self::flat_object(&self.properties)
    }

    pub fn flat_object(properties: &Value) -> Value {
        let entries = Self::flat_entries(properties)
            .into_iter()
            .map(|(name, text)| (name, Value::String(text)))
            .collect();
        Value::Object(entries)
    }

    pub fn flat_entries(properties: &Value) -> Vec<(String, String)> {
        properties
            .as_object()
            .into_iter()
            .flat_map(crate::sorted_json::SortedJson::entries)
            .map(|(path, value)| {
                let text = value
                    .as_str()
                    .map_or_else(|| crate::sorted_json::SortedJson::text(value), str::to_owned);
                (path.replace('.', "_").to_uppercase(), text)
            })
            .collect()
    }
}

