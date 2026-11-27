#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceAction {
    Css,
    Island { boundaries: Vec<String> },
    Reload,
}
