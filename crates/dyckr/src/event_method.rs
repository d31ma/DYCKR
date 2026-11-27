use std::ops::Range;

#[derive(Clone, Debug)]
pub struct EventParameter {
    pub name: String,
    pub type_source: String,
}
#[derive(Clone, Debug)]
pub struct EventMethod {
    pub owner: String,
    pub name: String,
    pub parameters: Vec<EventParameter>,
    pub signature: Range<usize>,
    pub body: Range<usize>,
    pub return_type: String,
    pub asynchronous: bool,
}

