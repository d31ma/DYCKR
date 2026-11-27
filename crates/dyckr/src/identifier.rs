pub fn class_reference<'a>(extension: &str, text: &'a str) -> Option<&'a str> {
    let text = text.trim();
    let name = match extension {
        "java" => text.strip_suffix(".class").unwrap_or(text),
        "php" | "kt" => text.strip_suffix("::class").unwrap_or(text),
        "swift" => text.strip_suffix(".self").unwrap_or(text),
        "cs" => text
            .strip_prefix("typeof(")
            .and_then(|value| value.strip_suffix(')'))
            .map_or(text, str::trim),
        _ => text,
    };
    identifier(name).then_some(name)
}

pub fn class_reference_example(extension: &str, name: &str) -> String {
    match extension {
        "java" => format!("{name}.class"),
        "php" | "kt" => format!("{name}::class"),
        "cs" => format!("typeof({name})"),
        _ => name.to_owned(),
    }
}

pub fn identifier(value: &str) -> bool {
    identifier_characters(value)
        && value.len() <= 128
        && !reserved_identifier(value)
        && !value.starts_with("__tachyon_")
}

pub fn identifier_characters(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() && !matches!(first, b'_' | b'$') {
        return false;
    }
    bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$'))
}

pub fn reserved_identifier(value: &str) -> bool {
    matches!(
        value,
        "__proto__"
            | "prototype"
            | "constructor"
            | "Behaviors"
            | "await"
            | "break"
            | "case"
            | "catch"
            | "class"
            | "const"
            | "continue"
            | "debugger"
            | "default"
            | "delete"
            | "do"
            | "else"
            | "enum"
            | "export"
            | "extends"
            | "false"
            | "finally"
            | "for"
            | "function"
            | "if"
            | "import"
            | "in"
            | "instanceof"
            | "let"
            | "new"
            | "null"
            | "return"
            | "super"
            | "switch"
            | "this"
            | "throw"
            | "true"
            | "try"
            | "typeof"
            | "var"
            | "void"
            | "while"
            | "with"
            | "yield"
    )
}
