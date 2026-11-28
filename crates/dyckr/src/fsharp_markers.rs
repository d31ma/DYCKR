use std::ops::Range;

pub const OPEN: &str = "[<";
pub const CLOSE: &str = ">]";

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

use crate::fsharp_lexer::FsharpLexer;
pub struct FsharpMarkers;

impl FsharpMarkers {
    pub fn strip(source: &str, find: &dyn Fn(&str) -> bool) -> String {
        let code = FsharpLexer::code_mask(source).code;
        Self::without_groups(source, groups(source, &code), find)
    }

    fn without_groups(
        source: &str,
        groups: Vec<Range<usize>>,
        find: &dyn Fn(&str) -> bool,
    ) -> String {
        let mut stripped = source.to_owned();
        for group in groups.into_iter().rev() {
            let replacement = Self::kept(&source[group.clone()], find);
            stripped.replace_range(group, &replacement);
        }
        stripped
    }

    fn kept(group: &str, find: &dyn Fn(&str) -> bool) -> String {
        let inside = &group[OPEN.len()..group.len() - CLOSE.len()];
        let kept: Vec<&str> = crate::fsharp_type::FsharpType::split_top_level(inside, ";")
            .into_iter()
            .filter(|item| !find(is_tachyon_name(item)))
            .collect();
        let written = written(&kept);
        format!("{written:<width$}", width = group.len())
    }
}
