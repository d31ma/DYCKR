use super::CppFill;

pub struct CppConversion {
    pub typed: bool,
    pub fills: Vec<(String, CppFill)>,
}
