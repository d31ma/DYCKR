use serde_json::Value;

pub struct WrittenQuery<'q> {
    pub at: usize,
    pub method: &'q str,
    pub arity: usize,
    pub query: Option<&'q Value>,
    pub page: Option<&'q Value>,
}
