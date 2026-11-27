use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustField {
    pub name: String,
    pub type_text: String,
    pub name_range: Range<usize>,
    pub type_range: Range<usize>,
}

impl RustField {
    pub const RAW_PREFIX: &'static str = "r#";

    pub fn key(&self) -> &str {
        self.name
            .strip_prefix(Self::RAW_PREFIX)
            .unwrap_or(&self.name)
    }
}
