use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct CppFill {
    pub path: Vec<String>,
    pub constant: Option<Value>,
}

impl CppFill {
    pub fn from_path(path: Vec<String>) -> Self {
        Self {
            path,
            constant: None,
        }
    }

    pub fn from_constant(constant: Value) -> Self {
        Self {
            path: Vec::new(),
            constant: Some(constant),
        }
    }
}
