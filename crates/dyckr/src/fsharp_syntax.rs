pub const EXTENSION: &str = "fs";

pub fn syntax(extension: &str) -> &str {
    if extension == "fs" {
        "cs"
    } else {
        extension
    }
}

use std::path::Path;

pub fn is_fsharp(source: &Path) -> bool {
    source.extension().is_some_and(|extension| extension.eq_ignore_ascii_case(EXTENSION))
}
