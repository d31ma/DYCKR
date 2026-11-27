use crate::shape::Shape;
use crate::type_text::TypeText;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelField {
    pub name: String,
    pub shape: Shape,
    pub required: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Record {
    pub fields: Vec<ModelField>,
    pub bases: Vec<String>,
    pub open: bool,
    pub unreadable: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Model {
    Record(Record),
    Alias(Shape),
}

impl Record {
    pub fn add(&mut self, extension: &str, name: &str, type_text: &str, required: bool) {
        let shape = TypeText::shape(extension, type_text).ok();
        self.open |= shape.is_none();
        self.fields
            .extend(shape.map(|shape| Self::field(name, shape, required)));
    }

    pub fn add_each(
        &mut self,
        extension: &str,
        fields: impl IntoIterator<Item = (String, String, bool)>,
    ) {
        for (name, type_text, required) in fields {
            self.add(extension, &name, &type_text, required);
        }
    }

    pub fn field(name: &str, shape: Shape, required: bool) -> ModelField {
        let required = required && !shape.is_optional();
        ModelField {
            name: name.to_owned(),
            shape,
            required,
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.fields.extend(other.fields);
        self.bases.extend(other.bases);
        self.open |= other.open;
        self.unreadable = self.unreadable.take().or(other.unreadable);
    }
}
