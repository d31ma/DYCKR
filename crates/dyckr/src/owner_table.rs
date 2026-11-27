use crate::failure::Failure;


pub fn owner_table(
    language: crate::companion_language::CompanionLanguage,
    source: &str,
    source_path: &str,
    excluded: &[String],
    extra: &str,
    needs_all: bool,
    constructed: &[(String, String)],
) -> Result<String, Failure> {
    let source = crate::without_bom::without_bom(source);
    if crate::declares_table::declares_table(language, source) {
        return Err(crate::companion_failures::authored_table(source_path));
    }
    if language == crate::companion_language::CompanionLanguage::Rust {
        return crate::rust_owner_table::rust_owner_table(source, source_path, excluded, extra, needs_all);
    }
    class_owner_table(language, source, source_path, excluded, extra, needs_all, constructed)
}

pub fn class_owner_table(
    language: crate::companion_language::CompanionLanguage,
    source: &str,
    source_path: &str,
    excluded: &[String],
    extra: &str,
    needs_all: bool,
    constructed: &[(String, String)],
) -> Result<String, Failure> {
    let wanted = crate::declared_service::declared_service(source_path, source);
    let Some((name, body)) = crate::companion_syntax::class_body_named(source, wanted.as_deref()) else {
        return Err(crate::companion_failures::unnamed_service(source_path, wanted.as_deref(), "class"));
    };
    let constructs = !constructed.is_empty();
    let members = crate::companion_member::members(language, body, source_path)?;
    let mut entries = String::new();
    for member in &members {
        if excluded.contains(&member.name) {
            continue;
        }
        entries.push_str(&crate::companion_member::entry(language, member));
    }
    entries.push_str(extra);
    if entries.trim().is_empty() {
        return Err(crate::companion_failures::exposes_nothing(source_path, &name));
    }
    let required = if matches!(
        language,
        crate::companion_language::CompanionLanguage::CSharp | crate::companion_language::CompanionLanguage::CSharpBrowser
    ) {
        crate::class_table::csharp_required_members(body)
    } else {
        Vec::new()
    };
    Ok(crate::class_table::class_table(
        language, &name, &entries, &required, constructs,
    ))
}
