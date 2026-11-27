use serde_json::Value;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub type_source: String,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub type_source: String,
    pub uninitialized: bool,
    pub initializer: Option<String>,
    pub is_static: bool,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub signature: Range<usize>,
    pub body: Range<usize>,
    pub return_type: String,
    pub asynchronous: bool,
    pub is_static: bool,
    pub declared: bool,
    pub metadata: serde_json::Map<String, Value>,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub name: String,
    pub declaration: usize,
    pub keyword: Range<usize>,
    pub declared: bool,
    pub body: Range<usize>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub configuration: bool,
    pub cache: Option<Value>,
    pub entity: Option<Value>,
    pub table: Option<String>,
    pub constructor: Option<Vec<String>>,
    pub repository: Option<Value>,
    pub policies: serde_json::Map<String, Value>,
}

#[derive(Clone, Debug)]
pub struct Edit {
    pub range: Range<usize>,
    pub replacement: String,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub source: String,
    pub language: String,
    pub environment: String,
    #[allow(clippy::struct_field_names)]
    pub module_id: String,
    pub types: Vec<Type>,
    pub edits: Vec<Edit>,
}

