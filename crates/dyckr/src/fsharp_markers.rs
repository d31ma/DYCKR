use std::ops::Range;

pub const OPEN: &'static str = "[<";
pub const CLOSE: &'static str = ">]";

pub fn is_tachyon_name(item: &str) -> &str {
    item.split(['(', ' ']).next().unwrap_or(item)
}

pub fn groups(source: &str, code: &[bool]) -> Vec<Range<usize>> {
        let mut groups = Vec::new();
        for (start, _) in source.match_indices(OPEN) {
            groups.extend(crate::fsharp_markers::group_at(source, code, start));
        }
        groups
    }

pub fn group_at(source: &str, code: &[bool], start: usize) -> Option<Range<usize>> {
        code.get(start).copied().filter(|visible| *visible)?;
        let length = source[start..].find(CLOSE)?;
        let end = start + length + CLOSE.len();
        let line_break = source[start..end].contains('\n');
        (!line_break).then_some(start..end)
    }

pub fn written(kept: &[&str]) -> String {
        if kept.is_empty() {
            return String::new();
        }
        format!("{}{}{}", OPEN, kept.join("; "), CLOSE)
    }
