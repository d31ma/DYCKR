use std::path::Path;

#[cfg(test)]
mod tests;

pub struct ClassFileName;

impl ClassFileName {
    pub fn matches(path: &Path, name: &str) -> bool {
        dyckr::class_file_name::class_file_matches(path, name)
    }

    pub fn preferred(path: &Path, name: &str) -> String {
        dyckr::class_file_name::preferred_class_file(path, name)
    }

    pub fn declared(path: &Path, source: &str) -> Option<String> {
        Self::read_name(path, source)
    }

    fn read_name(path: &Path, source: &str) -> Option<String> {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("rs") => super::single_class::Parser::declared_name(path, source)
                .ok()
                .flatten()
                .or_else(|| {
                    super::declaration_names(path, source)
                        .ok()?
                        .into_iter()
                        .next()
                }),
            _ => path.file_stem()?.to_str().map(str::to_owned),
        }
    }
}
