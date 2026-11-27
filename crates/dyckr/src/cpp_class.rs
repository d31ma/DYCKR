use crate::cpp_attribute::CppAttribute;
use crate::cpp_member::CppMember;
use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CppClass {
    pub name: String,
    pub name_at: usize,
    pub markers: Vec<CppAttribute>,
    pub members: Vec<CppMember>,
    pub enumerators: Vec<String>,
    pub is_enum: bool,
    pub body: Range<usize>,
}

impl CppClass {
    pub fn every_marker(&self) -> Vec<&CppAttribute> {
        let members = self.members.iter().flat_map(|member| member.markers.iter());
        let parameters = self
            .members
            .iter()
            .flat_map(|member| member.parameters.iter())
            .flat_map(|parameter| parameter.markers.iter());
        self.markers
            .iter()
            .chain(members)
            .chain(parameters)
            .collect()
    }
}
