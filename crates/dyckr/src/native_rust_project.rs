use crate::rust_companion_crate::RustCompanionCrate;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NativeRustProject {
    pub declaration: RustCompanionCrate,
    pub client_root: PathBuf,
    pub vendor: PathBuf,
}

impl NativeRustProject {
    pub const VENDOR_FOLDER: &'static str = "vendor/rust";
    pub const CACHE_FOLDER: &'static str = "rust-companion";

    pub fn located(
        declaration: RustCompanionCrate,
        source_root: &Path,
        authored_root: &Path,
    ) -> Self {
        Self {
            declaration,
            client_root: source_root.join("client"),
            vendor: authored_root.join(Self::VENDOR_FOLDER),
        }
    }
}
