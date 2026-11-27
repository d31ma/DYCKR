#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretMatch {
    pub line: usize,
    pub kind: &'static str,
    pub value: String,
}
