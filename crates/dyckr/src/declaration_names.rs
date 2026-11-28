use std::path::Path;
use crate::failure::Failure;


pub fn declaration_names_source(path: &Path, source: &str) -> Result<Vec<String>, Failure> {
    let code = crate::source_mask::masked_source(path, source)?;
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    let python = matches!(extension, "py" | "rb");
    let mut names = Vec::new();
    let mut depth = 0_usize;
    for line in code.lines() {
        if depth == 0
            && let Some(name) = top_level_declaration_name(extension, line, python)
        {
            names.push(name.to_owned());
        }
        if !python {
            depth = brace_depth_after(line, depth);
        }
    }
    Ok(names)
}

pub fn top_level_declaration_name<'a>(extension: &str, line: &'a str, python: bool) -> Option<&'a str> {
    let trimmed = line.trim_start();
    if extension == "go" {
        return crate::declaration::Declaration::go_name(trimmed);
    }
    let indented = line.len() != trimmed.len();
    if python && indented {
        return None;
    }
    if crate::declaration::Declaration::is_implementation(trimmed) {
        return None;
    }
    crate::declaration::Declaration::name(trimmed)
}

pub fn brace_depth_after(text: &str, mut depth: usize) -> usize {
    for byte in text.bytes() {
        match byte {
            b'{' => depth += 1,
            b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    depth
}
