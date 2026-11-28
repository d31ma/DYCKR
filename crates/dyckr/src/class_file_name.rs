use std::path::Path;

pub fn class_file_matches(path: &Path, name: &str) -> bool {
    let stem = path.file_stem().and_then(|stem| stem.to_str());
    [
        stem == Some(name),
        stem == Some(preferred_class_file(path, name).as_str()),
    ]
    .contains(&true)
}

pub fn preferred_class_file(path: &Path, name: &str) -> String {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("rs") => snake_case(name),
        _ => name.to_owned(),
    }
}

fn snake_case(name: &str) -> String {
    let bytes = name.as_bytes();
    (0..bytes.len()).map(|at| piece(bytes, at)).collect()
}

fn piece(bytes: &[u8], at: usize) -> String {
    let lower = char::from(bytes[at]).to_ascii_lowercase();
    if boundary(bytes, at) {
        format!("_{lower}")
    } else {
        lower.to_string()
    }
}

fn boundary(bytes: &[u8], at: usize) -> bool {
    let before = at
        .checked_sub(1)
        .and_then(|at| bytes.get(at))
        .copied()
        .unwrap_or(b' ');
    let after = bytes.get(at + 1).copied().unwrap_or(b' ');
    let after_word = [before.is_ascii_lowercase(), before.is_ascii_digit()].contains(&true);
    let acronym_end = [before.is_ascii_uppercase(), after.is_ascii_lowercase()] == [true, true];
    [
        bytes[at].is_ascii_uppercase(),
        [after_word, acronym_end].contains(&true),
    ] == [true, true]
}
