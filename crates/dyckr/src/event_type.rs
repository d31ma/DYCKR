use crate::failure::Failure;
use serde::Serialize;
use std::path::Path;

#[derive(Clone, Debug, Default, Serialize)]
pub struct EventType {
    pub name: String,
    pub post_construct: Option<String>,
    pub pre_destroy: Option<String>,
    pub listeners: Vec<crate::event_listener::EventListener>,
    pub resources: Vec<crate::event_resource::EventResource>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<crate::event_stream::EventStream>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timers: Vec<crate::event_timer::EventTimer>,
}

pub fn failure(path: &Path, at: usize, message: impl Into<String>) -> Failure {
    let help = "Use own-line OnEvent, OnStart and OnStop annotations on named instance members, and @Needs on a field named after the resource it takes.";
    let span = crate::failure::source_span(&path.to_string_lossy(), at, at.saturating_add(1));
    Failure::one(crate::failure::diagnostic(
        1308,
        message,
        Some(help.into()),
        span,
    ))
}
