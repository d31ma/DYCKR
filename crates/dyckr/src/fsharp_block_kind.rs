#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Type,
    Member,
    Value,
    Skip,
}
