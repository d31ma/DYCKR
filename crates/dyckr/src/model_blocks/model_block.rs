#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelBlock {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

impl ModelBlock {
    pub fn new(name: &str, start: usize, end: usize) -> Self {
        Self {
            name: name.to_owned(),
            start,
            end,
        }
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}
