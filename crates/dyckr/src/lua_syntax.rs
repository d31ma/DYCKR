pub const EXTENSION: &str = "lua";

pub fn syntax(extension: &str) -> &str {
    if extension == "lua" {
        "rb"
    } else {
        extension
    }
}
