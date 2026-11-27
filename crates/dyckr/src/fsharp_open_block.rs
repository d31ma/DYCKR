use crate::fsharp_block_kind::BlockKind;
use crate::fsharp_type_form::TypeForm;
use crate::fsharp_type_header::TypeHeader;

pub struct OpenBlock {
    pub indent: usize,
    pub kind: BlockKind,
    pub closer: String,
    pub form: TypeForm,
    pub name: String,
    pub fields_open: bool,
    pub parameters: String,
}

impl OpenBlock {
    pub fn body(indent: usize, kind: BlockKind, closer: &str) -> Self {
        Self {
            indent,
            kind,
            closer: closer.to_owned(),
            form: TypeForm::Class,
            name: String::new(),
            fields_open: false,
            parameters: String::new(),
        }
    }

    pub fn declared_type(indent: usize, header: &TypeHeader) -> Self {
        Self {
            indent,
            kind: BlockKind::Type,
            closer: String::from(" }"),
            form: header.form,
            name: header.name.clone(),
            fields_open: false,
            parameters: header.parameters.clone(),
        }
    }
}
