use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CppAttribute {
    pub name: String,
    pub arguments: String,
    pub at: usize,
    pub specifier: Range<usize>,
}

impl CppAttribute {
    pub fn has_arguments(&self) -> bool {
        !self.arguments.trim().is_empty()
    }
}
