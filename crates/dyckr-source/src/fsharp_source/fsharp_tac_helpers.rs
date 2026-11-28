use super::{FsharpHelperOwnership, FsharpView};
use crate::Failure;
use std::path::Path;

pub struct FsharpTacHelpers;

impl FsharpTacHelpers {
    pub fn source(path: &Path, source: &str) -> Result<(), Failure> {
        Self::wanted(path)
            .then(|| Self::authored(path, source))
            .transpose()
            .map(|_| ())
    }

    pub fn callers(path: &Path, source: &str) -> Result<(), Failure> {
        Self::wanted(path)
            .then(|| Self::constructed(path, source))
            .transpose()
            .map(|_| ())
    }

    fn wanted(path: &Path) -> bool {
        dyckr::fsharp_syntax::is_fsharp(path)
            && dyckr::realm::Realm::of(path) == Some(dyckr::realm::Realm::Tac)
    }

    fn authored(path: &Path, source: &str) -> Result<(), Failure> {
        crate::annotation_layout::FolderRoleCheck::check(path, source)?;
        FsharpHelperOwnership::read(path, source)?.require_source()?;
        let view = FsharpView::tac(path, source);
        crate::behavior::HelperRole::refuse_in_tac(view.path(), view.text())
            .map_err(|failure| view.failure(&failure))
    }

    fn constructed(path: &Path, source: &str) -> Result<(), Failure> {
        let ownership = FsharpHelperOwnership::read(path, source)?;
        ownership.require_source()?;
        ownership.require_callers()?;
        let view = FsharpView::tac(path, source);
        crate::behavior::HelperRole::require_callers(view.path(), view.text())
            .map_err(|failure| view.failure(&failure))
    }
}
