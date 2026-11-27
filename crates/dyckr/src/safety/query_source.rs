use std::path::Path;

pub struct QuerySource<'a> {
    pub path: &'a Path,
    pub extension: &'a str,
    pub text: &'a str,
    pub code: String,
}
