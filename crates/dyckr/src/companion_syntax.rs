
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
