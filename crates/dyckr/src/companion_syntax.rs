pub fn class_body_named<'a>(source: &'a str, wanted: Option<&str>) -> Option<(String, &'a str)> {
    let Some(wanted) = wanted else {
        return class_body_at(source);
    };
    let mut rest = source;
    while let Some((name, body)) = class_body_at(rest) {
        if name == wanted {
            return Some((name, body));
        }
        let body_start = body.as_ptr() as usize - rest.as_ptr() as usize;
        rest = rest.get(body_start + body.len()..)?;
    }
    None
}

pub fn class_body_at(source: &str) -> Option<(String, &str)> {
    let mut offset = 0usize;
    for line in source.split_inclusive('\n') {
        let line_start = offset;
        offset += line.len();
        let Some(after) = after_declaration_keyword(line, "class") else {
            continue;
        };
        let name = crate::unicode_name::UnicodeName::leading(after);
        if name.is_empty() {
            continue;
        }
        let open = source[line_start..].find('{')? + line_start;
        let body = braced_body(source, open)?;
        return Some((name, body));
    }
    None
}

pub fn after_declaration_keyword<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let mut rest = line.trim_start();
    while let Some(after) = without_class_modifier(rest) {
        rest = after;
    }
    rest.strip_prefix(keyword).and_then(after_keyword)
}

pub fn without_class_modifier(text: &str) -> Option<&str> {
    for modifier in crate::modifiers::Modifiers::CLASS {
        let Some(after) = text.strip_prefix(modifier) else {
            continue;
        };
        if after.starts_with(char::is_whitespace) {
            return Some(after.trim_start());
        }
    }
    None
}

pub fn braced_body(source: &str, open: usize) -> Option<&str> {
    let mut depth = 0usize;
    for (position, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + position]);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn rust_field_declarations(line: &str) -> Vec<String> {
    let mut declarations = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for character in line.chars() {
        match character {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth <= 0 => {
                declarations.push(std::mem::take(&mut current));
                continue;
            }
            _ => {}
        }
        current.push(character);
    }
    declarations.push(current);
    declarations
}

pub fn closing_parenthesis(arguments: &str) -> Option<usize> {
    let mut depth = 0_usize;
    for (position, character) in arguments.char_indices() {
        match character {
            '(' | '<' | '[' => depth += 1,
            ')' | '>' | ']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 && character == ')' {
                    return Some(position);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn after_keyword(rest: &str) -> Option<&str> {
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    Some(rest.trim_start())
}

pub fn block_named<'a>(
    source: &'a str,
    keyword: &str,
    wanted: Option<&str>,
) -> Option<(String, &'a str)> {
    for (header, body) in blocks(source, keyword) {
        let name = crate::unicode_name::UnicodeName::leading(header.trim());
        let Some(wanted) = wanted else {
            return Some((name, body));
        };
        if name == wanted {
            return Some((name, body));
        }
    }
    None
}

pub fn blocks<'a>(source: &'a str, keyword: &str) -> Vec<(&'a str, &'a str)> {
    let mut found = Vec::new();
    let mut offset = 0usize;
    for line in source.split_inclusive('\n') {
        let line_start = offset;
        offset += line.len();
        let Some(after) = after_declaration_keyword(line, keyword) else {
            continue;
        };
        let header_end = after.find(['{', ';', '\n']).unwrap_or(after.len());
        let header = &after[..header_end];
        let tail = &source[line_start..];
        let Some(brace) = tail.find('{') else {
            break;
        };
        if tail[..brace].contains(';') {
            found.push((header, ""));
            continue;
        }
        if let Some(body) = braced_body(source, line_start + brace) {
            found.push((header, body));
        }
    }
    found
}

pub fn parameter_type(declaration: &str) -> &str {
    declaration
        .split_once(':')
        .map_or(declaration, |(_, kind)| kind.trim())
}

pub fn swift_label(declaration: &str) -> String {
    let names = declaration.split_once(':').map_or("", |(names, _)| names);
    match names.split_whitespace().next() {
        Some("_") | None => String::new(),
        Some(label) => format!("{label}: "),
    }
}

pub fn annotated_type(annotation: &str) -> Option<String> {
    let after_colon = annotation.strip_prefix(':')?;
    let before_value = after_colon.split('=').next().unwrap_or_default();
    Some(before_value.trim().to_owned())
}
