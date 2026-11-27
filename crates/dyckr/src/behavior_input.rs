use std::path::Path;
use crate::failure::{diagnostic, source_span, Failure};

pub struct Input<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub code: String,
}

impl Input<'_> {
    pub fn extension(&self) -> &str {
        crate::groovy_source::GroovySource::syntax(
            self.path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or(""),
        )
    }

    pub fn failure(&self, at: usize, message: impl Into<String>) -> Failure {
        self.coded_failure(1308, at, message)
    }

    pub fn coded_failure(&self, code: u16, at: usize, message: impl Into<String>) -> Failure {
        Failure::one(diagnostic(
            code,
            message,
            Some("Behavior annotations require named types, own-line markers and literal configuration; see docs/BEHAVIOR_ANNOTATIONS.md.".to_owned()),
            source_span(&self.path.to_string_lossy(), at, (at + 1).min(self.source.len())),
        ))
    }
}
