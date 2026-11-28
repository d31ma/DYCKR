use crate::behavior_module::{Edit, Field, Method, Module, Type};

pub fn block(method: &Method, body: &str) -> String {
    if method.declared {
        format!(" {{ {body} }}")
    } else {
        body.to_owned()
    }
}

pub fn render(module: &Module, mut edits: Vec<Edit>, suffix: &str) -> Result<String, String> {
    edits.extend(module.edits.iter().cloned());
    edits.sort_by_key(|edit| std::cmp::Reverse((edit.range.start, edit.range.end)));
    let mut previous_start = module.source.len();
    let mut result = module.source.clone();
    for edit in edits {
        if edit.range.start > edit.range.end
            || edit.range.end > previous_start
            || !result.is_char_boundary(edit.range.start)
            || !result.is_char_boundary(edit.range.end)
        {
            return Err("Overlapping or invalid behavior source edits".to_owned());
        }
        previous_start = edit.range.start;
        result.replace_range(edit.range, &edit.replacement);
    }
    result.push('\n');
    result.push_str(suffix);
    Ok(result)
}

pub fn declared_fields(ty: &Type, spell: impl Fn(&Field) -> String) -> String {
    let mut output = String::new();
    if !ty.declared {
        return output;
    }
    for field in &ty.fields {
        output.push_str(&spell(field));
    }
    output
}

pub fn concrete_edit(module: &Module, ty: &Type) -> Option<Edit> {
    if !ty.declared || matches!(module.language.as_str(), "rs" | "go") {
        return None;
    }
    if module.language == "py" {
        return crate::polyglot::python_protocol_edit(module, ty);
    }
    let replacement = match module.language.as_str() {
        "swift" => "final class",
        "dart" => "class",
        language => crate::polyglot::concrete(language),
    };
    Some(Edit {
        range: ty.keyword.clone(),
        replacement: replacement.to_owned(),
    })
}

pub fn python_protocol_edit(module: &Module, ty: &Type) -> Option<Edit> {
    let header = &module.source[ty.keyword.end..ty.body.start];
    let open = ty.keyword.end + header.find('(')?;
    let close = ty.keyword.end + header.rfind(')')?;
    let mut bases = Vec::new();
    for base in module.source[open + 1..close].split(',') {
        let base = base.trim();
        if !base.is_empty() && !crate::polyglot::is_protocol_base(base) {
            bases.push(base);
        }
    }
    let replacement = if bases.is_empty() {
        String::new()
    } else {
        format!("({})", bases.join(", "))
    };
    Some(Edit {
        range: open..close + 1,
        replacement,
    })
}

pub fn is_protocol_base(base: &str) -> bool {
    matches!(base, "Protocol" | "typing.Protocol")
}

pub fn concrete(language: &str) -> &'static str {
    match language {
        "rs" => "struct",
        _ => "class",
    }
}

pub fn blank_out(text: &str) -> String {
    let mut blanked = String::with_capacity(text.len());
    for byte in text.bytes() {
        if matches!(byte, b'\n' | b'\r') {
            blanked.push(char::from(byte));
        } else {
            blanked.push(' ');
        }
    }
    blanked
}

pub fn erase(source: &str, markers: &[crate::behavior_marker::Marker]) -> String {
    let mut output = source.to_owned();
    for marker in markers.iter().rev() {
        let blanked = crate::polyglot::blank_out(&source[marker.span.clone()]);
        output.replace_range(marker.span.clone(), &blanked);
    }
    output
}
