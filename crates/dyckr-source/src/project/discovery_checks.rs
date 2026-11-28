use crate::Failure;
use crate::handler_declaration::DeclaredHandler;
use cap_std::fs::Dir;
use std::path::Path;
use std::sync::Arc;
use tachyon_diagnostics::Diagnostic;

pub trait DiscoveryChecks {
    fn check_handler(&self, handler: &Arc<DeclaredHandler>, project: &Dir) -> Result<(), Failure> {
        let _ = (handler, project);
        Ok(())
    }

    fn check_views<'a>(
        &self,
        project: &Dir,
        views: &mut dyn Iterator<Item = (&'a Path, &'a [u8])>,
    ) -> Vec<Diagnostic> {
        let _ = (project, views);
        Vec::new()
    }
}
