use std::path::Path;
use crate::failure::{Failure, diagnostic, source_span};

const MAX_CAPTURED_SOURCE: usize = 16 * 1024 * 1024;

pub fn masked_source(path: &Path, source: &str) -> Result<String, Failure> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if source.len() > MAX_CAPTURED_SOURCE {
        return Err(Failure::one(diagnostic(
            1310,
            "Captured source exceeds 16 MiB",
            Some(String::from("Split source into bounded modules.")),
            source_span(&path.to_string_lossy(), 0, 0),
        )));
    }
    let mut out = source.as_bytes().to_vec();
    if out.starts_with(&[0xef, 0xbb, 0xbf]) {
        out[..3].fill(b' ');
    }
    if matches!(extension, "js" | "ts") {
        mask_regex_literals(source, &mut out);
    }
    let source = String::from_utf8(out).unwrap_or_default();
    let mask = crate::lexical::code_mask(extension, &source);
    let mut code = source.into_bytes();
    for (byte, visible) in code.iter_mut().zip(mask.code) {
        if !visible && !matches!(*byte, b'\n' | b'\r') {
            *byte = b' ';
        }
    }
    Ok(String::from_utf8(code).unwrap_or_default())
}

fn can_start_regex(before: &str) -> bool {
    let before = before.trim_end();
    if before.is_empty()
        || before.ends_with(['=', '(', '[', '{', ',', ':', ';', '!', '?', '&', '|'])
    {
        return true;
    }
    for keyword in ["return", "throw", "yield", "case"] {
        if ends_with_keyword(before, keyword) {
            return true;
        }
    }
    false
}

fn ends_with_keyword(text: &str, keyword: &str) -> bool {
    let Some(before_keyword) = text.strip_suffix(keyword) else {
        return false;
    };
    before_keyword
        .bytes()
        .next_back()
        .is_none_or(|byte| !crate::declaration::is_identifier_byte(byte))
}

fn regex_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut at = start + 1;
    let mut in_bracket = false;
    while at < bytes.len() {
        match bytes[at] {
            b'\n' | b'\r' => return None,
            b'\\' => at += 1,
            b'[' => in_bracket = true,
            b']' => in_bracket = false,
            b'/' if !in_bracket => return Some(at + 1),
            _ => {}
        }
        at += 1;
    }
    None
}

pub fn mask_regex_literals(source: &str, output: &mut [u8]) {
    let mut at = 0;
    while at < output.len() {
        if matches!(output[at], b'\'' | b'"' | b'`') {
            at = quoted_end(output, at);
            continue;
        }
        if output.get(at..at + 2) == Some(b"//") {
            at += source[at..].find('\n').unwrap_or(source.len() - at);
            continue;
        }
        if output.get(at..at + 2) == Some(b"/*") {
            at = source[at + 2..]
                .find("*/")
                .map_or(source.len(), |offset| at + offset + 4);
            continue;
        }
        if output[at] == b'/'
            && can_start_regex(&source[..at])
            && let Some(end) = regex_end(source.as_bytes(), at)
        {
            output[at..end].fill(b' ');
            at = end;
            continue;
        }
        at += 1;
    }
}

fn quoted_end(bytes: &[u8], start: usize) -> usize {
    let quote = bytes[start];
    let mut at = start + 1;
    while at < bytes.len() {
        if bytes[at] == quote {
            return at + 1;
        }
        at += if bytes[at] == b'\\' { 2 } else { 1 };
    }
    bytes.len()
}
