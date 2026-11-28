use std::ops::Range;

pub struct UntypedField {
    pub name: String,
    pub span: Range<usize>,
}
