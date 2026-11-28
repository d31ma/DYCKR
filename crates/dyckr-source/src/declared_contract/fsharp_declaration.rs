use super::contract_role::ContractRole;
use dyckr::http_client_source::HttpClientSource;
use serde_json::Value;

pub struct FsharpDeclaration {
    pub role: ContractRole,
    pub marker: usize,
    pub header: usize,
    pub name: String,
    pub options: Value,
    pub body: Vec<usize>,
    pub repository: Option<(usize, String)>,
}

impl FsharpDeclaration {
    const SETUP_SUFFIX: &'static str = "Setup";
    const SETUP_KEY: &'static str = "setup";
    const ADAPTER_KEY: &'static str = "adapter";

    pub fn options_of(marker: &str) -> Value {
        let open = marker.find('{');
        let close = marker.rfind('}');
        open.zip(close)
            .filter(|(open, close)| close > open)
            .and_then(|(open, close)| serde_json::from_str(&marker[open..=close]).ok())
            .unwrap_or(Value::Null)
    }

    pub fn setup(&self) -> String {
        let stem = self
            .name
            .strip_suffix(self.role.suffix)
            .unwrap_or(&self.name);
        self.option(Self::SETUP_KEY)
            .unwrap_or_else(|| format!("{stem}{}", Self::SETUP_SUFFIX))
    }

    pub fn partition(&self) -> String {
        self.option(Self::ADAPTER_KEY)
            .unwrap_or_else(|| HttpClientSource::adapter_name(&self.name))
    }

    fn option(&self, key: &str) -> Option<String> {
        self.options
            .get(key)
            .and_then(Value::as_str)
            .map(str::to_owned)
    }
}
