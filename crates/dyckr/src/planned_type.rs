#[derive(Clone, Debug)]
pub struct PlannedType {
    pub name: String,
    pub line: usize,
    pub declared: bool,
    pub takes_context: bool,
}
