#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnusedMember {
    pub path: String,
    pub name: String,
    pub start: usize,
    pub field: bool,
}
