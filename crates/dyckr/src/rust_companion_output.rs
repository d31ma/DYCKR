use crate::rust_native_libraries::RustNativeLibraries;
use std::path::PathBuf;

pub struct RustCompanionOutput {
    pub library: PathBuf,
    pub executables: Vec<PathBuf>,
    pub native_libraries: RustNativeLibraries,
}
