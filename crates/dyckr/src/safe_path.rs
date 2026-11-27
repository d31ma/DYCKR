
pub fn safe_path(path: &str) -> bool {
    if !path.starts_with('/') || path.starts_with("//") || path.len() > 2048 {
        return false;
    }
    let Some(decoded) = decode_path(path) else {
        return false;
    };
    !decoded.starts_with("//")
        && !decoded.chars().any(|character| {
            character.is_control()
                || character.is_whitespace()
                || matches!(character, '\\' | '?' | '#' | '%' | '\u{feff}')
        })
        && !decoded
            .split('/')
            .any(|segment| matches!(segment, "." | ".."))
}

pub fn decode_path(path: &str) -> Option<String> {
    let mut bytes = path.bytes();
    let mut output = Vec::with_capacity(path.len());
    while let Some(byte) = bytes.next() {
        if byte == b'%' {
            let high = char::from(bytes.next()?).to_digit(16)?;
            let low = char::from(bytes.next()?).to_digit(16)?;
            output.push(u8::try_from(high * 16 + low).ok()?);
        } else {
            output.push(byte);
        }
    }
    String::from_utf8(output).ok()
}
