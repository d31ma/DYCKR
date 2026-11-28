#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    Type,
    Method,
    Field,
    Parameter,
}

impl Target {
    pub const fn word(self) -> &'static str {
        match self {
            Self::Type => "class",
            Self::Method => "method",
            Self::Field => "field",
            Self::Parameter => "parameter",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arguments {
    None,
    Optional,
    Required,
}
