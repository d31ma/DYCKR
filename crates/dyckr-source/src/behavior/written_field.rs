pub struct WrittenField<'f> {
    pub name: &'f str,
    pub type_source: &'f str,
    pub initializer: Option<&'f str>,
}
