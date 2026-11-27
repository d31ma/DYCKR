pub struct TypeName<'a> {
    pub access: &'static str,
    pub name: &'a str,
    pub generics: String,
    pub rest: &'a str,
}
