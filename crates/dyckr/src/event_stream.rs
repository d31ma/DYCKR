use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct EventStream {
    pub method: String,
    pub path: String,
    pub event: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub socket: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Value>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub topic: bool,
}
