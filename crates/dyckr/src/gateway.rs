use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GatewayTarget {
    Class { path: String, method: String },
    Command { arguments: Vec<String> },
}

impl GatewayTarget {
    pub fn names_a_path(entry: &str) -> bool {
        let bytes = entry.as_bytes();
        let has_drive_prefix =
            bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
        entry.contains('/') || has_drive_prefix
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayDeclaration {
    pub key: String,
    pub owner: String,
    pub method: String,
    pub arity: usize,
    pub target: GatewayTarget,
}

impl GatewayDeclaration {
    pub fn from_policy(policy: &Value) -> Option<Self> {
        let key = policy.get("key")?.as_str()?;
        let (owner, method) = key.split_once('.')?;
        let arity = usize::try_from(policy.get("arity")?.as_u64()?).ok()?;
        let target = if let Some(command) = policy.get("command") {
            let mut arguments = Vec::new();
            for entry in command.as_array()? {
                arguments.push(entry.as_str()?.to_owned());
            }
            GatewayTarget::Command { arguments }
        } else {
            GatewayTarget::Class {
                path: policy.get("class")?.as_str()?.to_owned(),
                method: policy.get("method")?.as_str()?.to_owned(),
            }
        };
        Some(Self {
            key: key.to_owned(),
            owner: owner.to_owned(),
            method: method.to_owned(),
            arity,
            target,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetSignature {
    pub parameter_types: Vec<String>,
    pub result_type: String,
    pub is_static: bool,
    pub has_body: bool,
    pub has_supported_receiver: bool,
}

impl TargetSignature {
    pub fn is_callable(&self, arity: usize, language: &str) -> bool {
        self.has_body
            && !self.is_static
            && self.parameter_types.len() == arity
            && self.uses_json_values(language)
            && self.has_supported_receiver
    }

    pub fn uses_json_values(&self, language: &str) -> bool {
        let Some((parameter, result, required)) = JsonValueType::for_language(language) else {
            return true;
        };
        let parameters_match = self
            .parameter_types
            .iter()
            .all(|written| JsonValueType::accepts(parameter, written, required));
        let result_matches = JsonValueType::accepts(result, &self.result_type, required)
            || JsonValueType::accepts(parameter, &self.result_type, required);
        parameters_match && result_matches
    }
}

pub struct GatewayMethod<'a> {
    pub owner: &'a str,
    pub name: &'a str,
    pub language: &'a str,
    pub parameter_types: Vec<&'a str>,
    pub result_type: &'a str,
    pub has_body: bool,
}

pub struct JsonValueType;

impl JsonValueType {
    pub const TYPESCRIPT_EMPTY_ANSWER: &'static str = "Promise<BehaviorJsonValue | void>";

    pub fn for_language(language: &str) -> Option<(&'static str, &'static str, bool)> {
        match language {
            "ts" => Some(("BehaviorJsonValue", "Promise<BehaviorJsonValue>", false)),
            "php" => Some(("mixed", "mixed", false)),
            "java" => Some(("Object", "Object", true)),
            "kt" => Some(("Any?", "Any?", true)),
            "cs" => Some(("BehaviorJSON", "BehaviorJSON", true)),
            "rs" => Some(("BehaviorValue", "BehaviorResult<BehaviorValue>", true)),
            "go" => Some(("BehaviorValue", "(BehaviorValue, error)", true)),
            "dart" => Some(("Object?", "Future<Object?>", true)),
            "hpp" => Some(("tachyon::json::JsonNode", "tachyon::json::JsonNode", true)),
            _ => None,
        }
    }

    pub fn accepts(expected: &str, written: &str, required: bool) -> bool {
        let written = written.trim();
        if written.is_empty() {
            return !required;
        }
        written.replace(' ', "") == expected.replace(' ', "")
    }

    pub fn accepts_result(language: &str, expected: &str, written: &str, required: bool) -> bool {
        Self::accepts(expected, written, required)
            || Self::is_typescript_empty_answer(language, written)
    }

    pub fn is_typescript_empty_answer(language: &str, written: &str) -> bool {
        language == "ts" && Self::accepts(Self::TYPESCRIPT_EMPTY_ANSWER, written, true)
    }
}
