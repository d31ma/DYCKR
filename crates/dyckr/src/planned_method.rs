use serde_json::{Map, Value};

#[derive(Clone, Debug)]
pub struct PlannedMethod {
    pub owner: String,
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub metadata: Map<String, Value>,
    pub declared: bool,
    pub asynchronous: bool,
    pub mapped_argument: Option<usize>,
}

