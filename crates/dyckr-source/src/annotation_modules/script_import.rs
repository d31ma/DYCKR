use std::ops::Range;

pub struct ScriptImport {
    pub range: Range<usize>,
    pub specifier: String,
}
