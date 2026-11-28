use crate::handler_declaration::DeclaredHandler;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledWorker {
    pub(super) relative: String,
    pub(super) every_seconds: u64,
    pub(super) initial_seconds: u64,
    pub(super) source: Arc<DeclaredHandler>,
}

impl ScheduledWorker {
    pub fn relative(&self) -> &str {
        &self.relative
    }

    pub const fn initial_seconds(&self) -> u64 {
        self.initial_seconds
    }

    pub const fn every_seconds(&self) -> u64 {
        self.every_seconds
    }

    pub const fn source(&self) -> &Arc<DeclaredHandler> {
        &self.source
    }
}
