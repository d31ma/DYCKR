use std::path::{Path, PathBuf};

#[must_use]
pub fn units(project: &Path, realm: &str) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![PathBuf::from(realm).join("tests")];
    while let Some(relative) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(project.join(&relative)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let Some(name) = name.to_str() else { continue };
            let child = relative.join(name);
            if entry.path().is_dir() {
                pending.push(child);
                continue;
            }
            if is_test_unit_file(name) {
                found.push(child);
            }
        }
    }
    found.sort();
    found
}

pub fn is_test_unit_file(name: &str) -> bool {
    let stem = Path::new(name).file_stem().and_then(|stem| stem.to_str());
    stem.is_some_and(|stem| stem.ends_with("Test"))
}
