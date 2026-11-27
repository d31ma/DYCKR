pub struct HttpClientSource;
use std::ops::Range;
use std::path::Path;

impl HttpClientSource {

    pub fn extension(path: &Path) -> &str {
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("")
    }

    pub fn code(path: &Path, source: &str) -> String {
        let mask = crate::lexical::code_mask(Self::extension(path), source);
        Self::masked(source, &mask.code)
    }

    pub fn apply(source: &str, mut edits: Vec<(Range<usize>, String)>) -> String {
        edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
        let mut output = source.to_owned();
        for (range, replacement) in edits {
            output.replace_range(range, &replacement);
        }
        output
    }

    pub fn masked(source: &str, visible: &[bool]) -> String {
        let mut bytes = source.as_bytes().to_vec();
        for (byte, shown) in bytes.iter_mut().zip(visible) {
            if !shown && !matches!(*byte, b'\n' | b'\r') {
                *byte = b' ';
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
    }

    pub fn normalized(name: &str) -> String {
        name.rsplit("::")
            .next()
            .unwrap_or(name)
            .replace('_', "")
            .to_ascii_lowercase()
    }
}
