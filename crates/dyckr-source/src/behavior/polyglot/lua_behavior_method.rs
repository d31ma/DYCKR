pub struct LuaBehaviorMethod {
    pub owner: String,
    pub name: String,
    pub parameters: Vec<String>,
    pub mapping: Option<serde_json::Value>,
    pub validated: Option<(usize, String)>,
    pub gateway: Option<String>,
}
