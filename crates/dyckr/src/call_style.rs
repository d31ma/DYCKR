use crate::sorted_json::SortedJson;
use serde_json::Value;

pub struct CallStyle {
    receiver: &'static str,
    pub literal: fn(&Value) -> String,
}

impl CallStyle {
    pub const RUST: Self = Self {
        receiver: "self.",
        literal: Self::rust_literal,
    };
    pub const JVM: Self = Self {
        receiver: "",
        literal: Self::jvm_literal,
    };
    pub const JSON: Self = Self {
        receiver: "",
        literal: Self::json_literal,
    };
    pub const DART: Self = Self {
        receiver: "",
        literal: Self::dart_literal,
    };
    pub const SWIFT: Self = Self {
        receiver: "",
        literal: Self::rust_literal,
    };
    pub const DOLLAR: char = '$';
    pub const ESCAPED_DOLLAR: &'static str = "\\u0024";
    pub const DART_DOLLAR: &'static str = "\\$";
    pub const LONG_MARK: &'static str = "L";

    pub fn call(&self, method: &str, argument: &str) -> String {
        format!("{}{method}({argument})", self.receiver)
    }

    pub fn rust_literal(value: &Value) -> String {
        match value {
            Value::String(text) => format!("{text:?}"),
            other => SortedJson::text(other),
        }
    }

    pub fn json_literal(value: &Value) -> String {
        SortedJson::text(value)
    }

    pub fn dart_literal(value: &Value) -> String {
        SortedJson::text(value).replace(Self::DOLLAR, Self::DART_DOLLAR)
    }

    pub fn jvm_literal(value: &Value) -> String {
        let text = SortedJson::text(value).replace(Self::DOLLAR, Self::ESCAPED_DOLLAR);
        format!("{text}{}", Self::long_mark(value))
    }

    pub fn long_mark(value: &Value) -> &'static str {
        let beyond_int = value
            .as_i64()
            .is_some_and(|whole| i32::try_from(whole).is_err());
        if beyond_int {
            return Self::LONG_MARK;
        }
        ""
    }
}
