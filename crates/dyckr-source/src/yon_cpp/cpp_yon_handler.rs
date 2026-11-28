use dyckr::cpp_file::CppFile;
use std::path::PathBuf;

pub struct CppYonHandler {
    pub path: PathBuf,
    pub file: CppFile,
    pub source: String,
}
