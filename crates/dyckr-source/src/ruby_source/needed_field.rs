use std::ops::Range;

pub struct NeededField {
    pub name: String,
    pub type_name: String,
    pub span: Range<usize>,
}
