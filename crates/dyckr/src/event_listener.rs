use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct EventListener {
    pub method: String,
    pub events: Vec<String>,
    pub replay: bool,
    pub condition: Option<Value>,
    pub id: Option<String>,
    pub order: i64,
    pub asynchronous: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
}

impl EventListener {
    pub const MAX_ID_BYTES: usize = 64;
    pub const MAX_ORDER: i64 = 1_000_000;
    pub const OPTION_KEYS: [&str; 3] = ["replay", "condition", "id"];
    pub const NAMED_EVENT_REFUSAL: &str = "OnEvent takes no event names: the model type of the payload parameter is the event (ADR 0055). Write @OnEvent and give the parameter a model type, for example placed(OrderPlaced order). `ty migrate fix --write` converts a listener whose event name matches a model";
}

