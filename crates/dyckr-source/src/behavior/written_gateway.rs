use super::gateway::policy;
use crate::Failure;
use dyckr::behavior_input::Input;
use dyckr::behavior_marker::Marker;
use dyckr::gateway::{GatewayDeclaration, GatewayMethod};
use std::path::Path;

pub struct WrittenGateway<'a> {
    pub owner: &'a str,
    pub name: &'a str,
    pub arguments: &'a str,
    pub at: usize,
    pub parameter_types: Vec<&'a str>,
    pub result_type: &'a str,
    pub has_body: bool,
}

impl WrittenGateway<'_> {
    const CPP: &'static str = "hpp";
    const UNREAD: &'static str = "Gateway takes either class with an optional method, or command";

    pub fn declared(&self, path: &Path, source: &str) -> Result<GatewayDeclaration, Failure> {
        let input = Input {
            path,
            source,
            code: source.to_owned(),
        };
        let marker = Marker {
            name: String::from("Gateway"),
            arguments: self.arguments.to_owned(),
            span: self.at..self.at + 1,
            target: self.at,
            parameter: false,
        };
        let method = GatewayMethod {
            owner: self.owner,
            name: self.name,
            language: Self::CPP,
            parameter_types: self.parameter_types.clone(),
            result_type: self.result_type,
            has_body: self.has_body,
        };
        let written = policy(&input, &marker, &method)?;
        GatewayDeclaration::from_policy(&written)
            .ok_or_else(|| input.failure(self.at, Self::UNREAD))
    }
}
