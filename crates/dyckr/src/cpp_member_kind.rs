#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CppMemberKind {
    Field,
    Method,
    Constructor,
    Destructor,
    Operator,
    NestedType,
    Other,
}
