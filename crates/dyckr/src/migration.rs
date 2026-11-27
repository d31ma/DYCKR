use crate::failure::{diagnostic, Failure};
use serde::Serialize;
use std::fmt::Write as _;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationStatus {
    Supported,
    Changed,
    Unsupported,
}

impl MigrationStatus {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Changed => "changed",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MigrationFinding {
    pub source: String,
    pub feature: String,
    pub status: MigrationStatus,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl MigrationFinding {
    pub fn unreadable(relative: &str, failure: &Failure) -> Self {
        let detail = failure.diagnostics().first().map_or_else(
            || failure.to_string(),
            |diagnostic| diagnostic.message.clone(),
        );
        Self::new(
            relative,
            "source.unreadable",
            MigrationStatus::Unsupported,
            &detail,
            Some(
                "Save the file as UTF-8 text, or move it out of the project folders, then run ty migrate check again.",
            ),
        )
    }

    pub fn new(
        source: &str,
        feature: &str,
        status: MigrationStatus,
        detail: &str,
        action: Option<&str>,
    ) -> Self {
        Self {
            source: String::from(source),
            feature: String::from(feature),
            status,
            detail: String::from(detail),
            action: action.map(String::from),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MigrationReport {
    pub contract_version: u8,
    pub supported: usize,
    pub changed: usize,
    pub unsupported: usize,
    pub findings: Vec<MigrationFinding>,
}

impl MigrationReport {
    #[must_use]
    pub const fn is_clean(&self) -> bool {
        self.unsupported == 0 && self.changed == 0
    }

    #[must_use]
    pub fn unsupported_failure(&self) -> Option<Failure> {
        if self.unsupported == 0 {
            return None;
        }
        Some(Failure::one(diagnostic(
            1702,
            format!(
                "{} construct(s) have no equivalent in this implementation.",
                self.unsupported
            ),
            Some(String::from(
                "Resolve each unsupported finding, or pass --allow-unsupported \
                 to report without failing.",
            )),
            None,
        )))
    }

    #[must_use]
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        for finding in &self.findings {
            let _ = writeln!(
                text,
                "{:<11} {:<28} {}",
                finding.status.label(),
                finding.feature,
                finding.source
            );
            let _ = writeln!(text, "            {}", finding.detail);
            if let Some(action) = &finding.action {
                let _ = writeln!(text, "            action: {action}");
            }
        }
        let _ = writeln!(
            text,
            "\n{} supported, {} changed, {} unsupported",
            self.supported, self.changed, self.unsupported
        );
        text
    }
}
