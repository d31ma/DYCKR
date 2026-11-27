use crate::ascii_name::AsciiName;
pub struct Yard;

impl Yard {
    pub fn comments_before(source: &str, declaration: usize) -> String {
        let line_start = source[..declaration].rfind('\n').map_or(0, |at| at + 1);
        let mut lines = Vec::new();
        for line in source[..line_start].lines().rev() {
            let text = line.trim();
            let Some(comment) = text.strip_prefix('#') else {
                break;
            };
            if !comment.starts_with('[') {
                lines.push(comment);
            }
        }
        lines.reverse();
        lines.join("\n")
    }

    pub fn parameter_type(comments: &str, name: &str) -> String {
        for line in comments.lines() {
            let Some(rest) = line.trim().strip_prefix("@param") else {
                continue;
            };
            let rest = rest.trim_start();
            if let Some((types, after)) = Self::bracketed(rest)
                && Self::first_word(after) == name
            {
                return types.to_owned();
            }
            let written = Self::first_word(rest);
            if written != name {
                continue;
            }
            if let Some((types, _)) = Self::bracketed(rest[written.len()..].trim_start()) {
                return types.to_owned();
            }
        }
        String::new()
    }

    pub fn return_type(comments: &str) -> String {
        for line in comments.lines() {
            let Some(rest) = line.trim().strip_prefix("@return") else {
                continue;
            };
            if let Some((types, _)) = Self::bracketed(rest.trim_start()) {
                return types.to_owned();
            }
        }
        String::new()
    }

    pub fn attribute_types(comments: &str) -> Vec<(String, String)> {
        let mut attributes = Vec::new();
        let mut pending: Option<String> = None;
        for line in comments.lines() {
            let text = line.trim();
            if let Some(rest) = text.strip_prefix("@!attribute") {
                let rest = rest.trim_start();
                let rest = Self::bracketed(rest).map_or(rest, |(_, after)| after);
                pending = Some(Self::first_word(rest).to_owned());
                continue;
            }
            let Some(name) = pending.take() else {
                continue;
            };
            let Some(rest) = text.strip_prefix("@return") else {
                pending = Some(name);
                continue;
            };
            if let Some((types, _)) = Self::bracketed(rest.trim_start()) {
                attributes.push((name, types.to_owned()));
            }
        }
        attributes
    }

    fn bracketed(text: &str) -> Option<(&str, &str)> {
        let inside = text.strip_prefix('[')?;
        let mut depth = 0_usize;
        for (at, character) in inside.char_indices() {
            match character {
                '[' => depth += 1,
                ']' if depth == 0 => return Some((inside[..at].trim(), &inside[at + 1..])),
                ']' => depth -= 1,
                _ => {}
            }
        }
        None
    }

    fn first_word(text: &str) -> &str {
        AsciiName::leading(text.trim_start())
    }
}

