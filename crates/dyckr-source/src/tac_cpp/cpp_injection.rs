#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CppInjection {
    pub field: String,
    pub provider: String,
    pub terminator: usize,
}
