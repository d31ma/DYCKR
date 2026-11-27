use crate::cpp_attribute::CppAttribute;
use crate::cpp_member_kind::CppMemberKind;
use crate::cpp_parameter::CppParameter;
use crate::cpp_type_text::CppTypeText;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CppMember {
    pub kind: CppMemberKind,
    pub name: String,
    pub name_at: usize,
    pub type_text: String,
    pub parameters: Vec<CppParameter>,
    pub public: bool,
    pub is_static: bool,
    pub has_initializer: bool,
    pub declarators: usize,
    pub markers: Vec<CppAttribute>,
    pub terminator: usize,
    pub body_end: usize,
    pub is_const: bool,
}

impl CppMember {
    pub fn marked(&self, name: &str) -> bool {
        self.markers.iter().any(|marker| marker.name == name)
    }

    pub fn plain_type(&self) -> String {
        CppTypeText::plain(&self.type_text)
    }

    pub fn is_reference(&self) -> bool {
        [self.type_text.contains('&'), self.type_text.contains('*')].contains(&true)
    }

    pub fn is_instance(&self, kind: CppMemberKind) -> bool {
        [self.kind == kind, !self.is_static] == [true, true]
    }

    pub fn is_task(&self) -> bool {
        let plain = self.plain_type();
        [
            plain.starts_with("Task<"),
            plain.starts_with("tachyon::Task<"),
        ]
        .contains(&true)
    }

    pub fn is_public_instance(&self, kind: CppMemberKind) -> bool {
        [self.is_instance(kind), self.public] == [true, true]
    }
}
