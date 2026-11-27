pub const EXTENSION: &str = "lua";

pub fn syntax(extension: &str) -> &str {
    if extension == "lua" {
        "rb"
    } else {
        extension
    }
}

use std::path::Path;

pub fn is_lua(source: &Path) -> bool {
    source.extension().is_some_and(|extension| extension.eq_ignore_ascii_case(EXTENSION))
}
