use std::path::Path;
use std::fmt::Write as _;
use sha2::{Digest, Sha256};


pub fn module_id(path: &Path) -> String {
    let portable = path.to_string_lossy().replace('\\', "/");
    let normalized = portable
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect::<Vec<_>>()
        .join("/");
    let mut encoded = String::with_capacity(64);
    for byte in Sha256::digest(normalized.as_bytes()) {
        let _ = write!(encoded, "{byte:02x}");
    }
    encoded
}

