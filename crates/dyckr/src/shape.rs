use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Shape {
    #[default]
    Any,
    Text,
    Integer,
    Number,
    Boolean,
    List(Box<Shape>),
    Optional(Box<Shape>),
    Model(String),
}

impl Shape {
    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional(_))
    }

    pub fn inner(&self) -> &Self {
        match self {
            Self::Optional(inner) => inner,
            _ => self,
        }
    }

    pub fn plain_name(&self) -> String {
        match self {
            Self::Any => String::from("any JSON value"),
            Self::Text => String::from("text"),
            Self::Integer => String::from("a whole number"),
            Self::Number => String::from("a number"),
            Self::Boolean => String::from("true or false"),
            Self::List(item) => format!("a list of {}", item.plain_name()),
            Self::Optional(inner) => format!("{} or null", inner.plain_name()),
            Self::Model(name) => format!("a {name} object"),
        }
    }
}

