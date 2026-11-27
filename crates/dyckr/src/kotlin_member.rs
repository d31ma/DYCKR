#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KotlinMember {
    pub name: String,
    pub value_type: String,
    pub parameters: Vec<String>,
    pub method: bool,
    pub mutable: bool,
}
