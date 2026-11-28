pub fn declares_type(extension: &str, line: &str) -> bool {
    let line = line.trim_start();
    !Declaration::is_implementation(line) && Declaration::named(extension, line).is_some()
}

pub struct Declaration;

impl Declaration {
    const MAX_NAME: usize = 128;
    const MODIFIERS: [&'static str; 17] = [
        "public",
        "private",
        "protected",
        "internal",
        "final",
        "open",
        "abstract",
        "sealed",
        "static",
        "pub(crate)",
        "pub",
        "export",
        "default",
        "data",
        "partial",
        "readonly",
        "unsafe",
    ];
    const KINDS: [&'static str; 7] = [
        "class",
        "struct",
        "object",
        "record",
        "interface",
        "protocol",
        "trait",
    ];

    pub fn named<'a>(extension: &str, line: &'a str) -> Option<&'a str> {
        if extension == "go" {
            return Self::go_name(line);
        }
        Self::name(line)
    }

    pub fn go_name(line: &str) -> Option<&str> {
        let rest = Self::strip_word(line, "type")?;
        Self::take_name(rest)
    }

    pub fn is_implementation(line: &str) -> bool {
        line.split_whitespace()
            .any(|word| word == "impl" || word.starts_with("impl<"))
    }

    pub fn name(line: &str) -> Option<&str> {
        let line = Self::skip_modifiers(line);
        if let Some(rest) = Self::strip_impl(line) {
            return Self::implementation_target(rest.trim_start());
        }
        let rest = Self::KINDS
            .into_iter()
            .find_map(|word| Self::strip_word(line, word))?;
        Self::take_name(Self::skip_record_kind(rest))
    }

    pub fn skip_record_kind(rest: &str) -> &str {
        let kind = Self::strip_word(rest, "struct").or_else(|| Self::strip_word(rest, "class"));
        kind.unwrap_or(rest)
    }

    pub fn skip_modifiers(mut line: &str) -> &str {
        while let Some(rest) = Self::MODIFIERS
            .into_iter()
            .find_map(|word| Self::strip_word(line, word))
        {
            line = rest;
        }
        line
    }

    pub fn strip_impl(line: &str) -> Option<&str> {
        let rest = line.strip_prefix("impl")?;
        if rest.starts_with('<') || rest.starts_with(char::is_whitespace) {
            return Some(rest);
        }
        None
    }

    pub fn strip_word<'a>(line: &'a str, word: &str) -> Option<&'a str> {
        line.strip_prefix(word)
            .filter(|rest| rest.starts_with(char::is_whitespace))
            .map(str::trim_start)
    }

    pub fn take_name(source: &str) -> Option<&str> {
        let length = source
            .bytes()
            .take_while(|byte| is_identifier_byte(*byte))
            .count();
        let valid_end = source
            .as_bytes()
            .get(length)
            .is_none_or(|byte| byte.is_ascii() && *byte != b'$');
        if length == 0 || length > Self::MAX_NAME || !valid_end {
            return None;
        }
        Some(&source[..length])
    }

    pub fn implementation_target(mut source: &str) -> Option<&str> {
        if source.starts_with('<') {
            source = Self::skip_generics(source)?.trim_start();
        }
        let mut rest = source;
        let mut last_name = None;
        while !rest.is_empty() {
            if rest.starts_with(['{', ';']) || Self::strip_word(rest, "where").is_some() {
                break;
            }
            if rest.starts_with('<') {
                rest = Self::skip_generics(rest)?.trim_start();
                continue;
            }
            if let Some(after) = Self::strip_word(rest, "for") {
                last_name = None;
                rest = after;
                continue;
            }
            if let Some(after) = rest.strip_prefix("::") {
                rest = after.trim_start();
                continue;
            }
            let name = Self::take_name(rest)?;
            last_name = Some(name);
            rest = rest[name.len()..].trim_start();
        }
        last_name
    }

    pub fn skip_generics(source: &str) -> Option<&str> {
        let mut depth = 0_usize;
        for (at, byte) in source.bytes().enumerate() {
            match byte {
                b'<' => depth += 1,
                b'>' => {
                    depth = depth.checked_sub(1)?;
                    if depth == 0 {
                        return Some(&source[at + 1..]);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
pub fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
