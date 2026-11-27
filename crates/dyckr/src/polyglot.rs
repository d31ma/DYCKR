use crate::behavior_module::{Edit, Field, Method, Module, Type};


pub fn block(method: &Method, body: &str) -> String {
    if method.declared {
        format!(" {{ {body} }}")
    } else {
        body.to_owned()
    }
}

pub fn render(
    module: &Module,
    mut edits: Vec<Edit>,
    suffix: &str,
) -> Result<String, String> {
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
