pub fn expression_can_start(prefix: &[u8]) -> bool {
    let Some(last) = prefix.iter().rposition(|byte| !byte.is_ascii_whitespace()) else {
        return true;
    };
    if b"=(:,;{[!?|&+-*%^~".contains(&prefix[last]) {
        return true;
    }
    let start = prefix[..=last]
        .iter()
        .rposition(|byte| !byte.is_ascii_alphabetic())
        .map_or(0, |at| at + 1);
    matches!(
        &prefix[start..=last],
        b"return" | b"throw" | b"yield" | b"case"
    )
}

pub fn regular_expression_end(code: &[u8], mut at: usize) -> Option<usize> {
    let mut character_class = false;
    while at < code.len() {
        match code[at] {
            b'\n' | b'\r' => return None,
            b'\\' => at += 1,
            b'[' => character_class = true,
            b']' => character_class = false,
            b'/' if !character_class => return Some(at + 1),
            _ => {}
        }
        at += 1;
    }
    None
}

pub fn mask_regular_expressions(code: &mut [u8]) {
    let mut at = 0;
    while at < code.len() {
        if code[at] == b'/'
            && crate::visible_code::expression_can_start(&code[..at])
            && let Some(end) = crate::visible_code::regular_expression_end(code, at + 1)
        {
            code[at..end].fill(b' ');
            at = end;
            continue;
        }
        at += 1;
    }
}

pub fn visible_code(source: &str, visible_bytes: &[bool]) -> String {
    let mut bytes = source.as_bytes().to_vec();
    for (byte, visible) in bytes.iter_mut().zip(visible_bytes) {
        if !visible && !matches!(byte, b'\n' | b'\r') {
            *byte = b' ';
        }
    }
    crate::visible_code::mask_regular_expressions(&mut bytes);
    String::from_utf8(bytes).unwrap_or_default()
}
