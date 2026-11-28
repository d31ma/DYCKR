use std::path::Path;

pub fn module_failure(path: &Path, source: &str, message: String) -> crate::failure::Failure {
    let input = crate::behavior_input::Input {
        path,
        source,
        code: String::new(),
    };
    input.failure(0, message)
}
