use crate::cpp_attribute::CppAttribute;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CppParameter {
    pub name: String,
    pub type_text: String,
    pub markers: Vec<CppAttribute>,
    pub has_default: bool,
}
