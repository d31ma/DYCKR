use super::CapturedFile;
use crate::Failure;
use crate::handler_declaration::{DeclaredHandler, OwnedSourceRoot};
use cap_std::fs::Dir;
use std::path::Path;
use std::sync::Arc;
use tachyon_diagnostics::Diagnostic;

pub(super) struct SnapshotContext<'a> {
    pub(super) project_root: &'a Path,
    pub(super) project: &'a Dir,
    pub(super) source_root: &'a Arc<OwnedSourceRoot>,
    pub(super) check: Option<&'a dyn super::DiscoveryChecks>,
}

impl SnapshotContext<'_> {
    pub(super) fn discover_handler(
        &self,
        relative: &Path,
        bytes: Vec<u8>,
    ) -> Result<Arc<DeclaredHandler>, Failure> {
        let declared = Arc::new(DeclaredHandler::declare(
            self.project_root.to_path_buf(),
            relative,
            bytes,
            Arc::clone(self.source_root),
        )?);
        if let Some(check) = self.check {
            check.check_handler(&declared, self.project)?;
        }
        Ok(declared)
    }

    pub(super) fn middleware(
        &self,
        file: Option<CapturedFile>,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Option<Arc<DeclaredHandler>> {
        let file = file?;
        self.discover_handler(&file.relative, file.bytes)
            .map_err(|failure| diagnostics.extend_from_slice(failure.diagnostics()))
            .ok()
    }
}
