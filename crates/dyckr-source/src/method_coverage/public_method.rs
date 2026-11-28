#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicMethod {
    pub name: String,
    pub line: usize,
}

impl PublicMethod {
    pub fn new(name: String, line: usize) -> Self {
        Self { name, line }
    }
}
