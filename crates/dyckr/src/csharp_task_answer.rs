use serde_json::{Value, json};

pub struct CsharpTaskAnswer {
    pub reader: &'static str,
    pub item: String,
    shape: String,
    suffix: &'static str,
    models: Value,
}

impl CsharpTaskAnswer {
    pub const RECORD: &'static str = "ReadResult";
    pub const VALUE: &'static str = "ReadResultValue";
    pub const LIST: &'static str = "ReadResultList";
    pub const SUFFIXES: [&'static str; 2] = ["", ".ToArray()"];

    pub fn record(name: &str, models: Value) -> Self {
        Self {
            reader: Self::RECORD,
            item: format!("Models.{name}"),
            shape: name.to_owned(),
            suffix: Self::SUFFIXES[0],
            models,
        }
    }

    pub fn value(kind: &str, shape: &str) -> Self {
        Self {
            reader: Self::VALUE,
            item: kind.to_owned(),
            shape: shape.to_owned(),
            suffix: Self::SUFFIXES[0],
            models: json!({}),
        }
    }

    pub fn listed(self, array: bool) -> Self {
        Self {
            reader: Self::LIST,
            shape: format!("[]{}", self.shape),
            suffix: Self::SUFFIXES[usize::from(array)],
            ..self
        }
    }

    pub fn metadata(&self) -> Value {
        json!({
            "reader": format!("{}<{}>", self.reader, self.item),
            "shape": self.shape,
            "suffix": self.suffix,
            "models": self.models,
        })
    }
}
