pub const EXTENSION: &str = "fs";

pub fn syntax(extension: &str) -> &str {
    if extension == "fs" {
        "cs"
    } else {
        extension
    }
}
