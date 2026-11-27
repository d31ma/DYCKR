
pub fn with_required_constructor(
    language: crate::companion_language::CompanionLanguage,
    source: &str,
    source_path: &str,
    needs_all: bool,
    constructed: &[(String, String)],
) -> String {
    let source = crate::without_bom::without_bom(source);
    let wanted = crate::declared_service::declared_service(source_path, source);
    let Some((name, body)) = crate::companion_syntax::class_body_named(source, wanted.as_deref()) else {
        return source.to_owned();
    };
    let fields = constructed;
    if fields.is_empty() {
        return source.to_owned();
    }
    let constructor = match language {
        crate::companion_language::CompanionLanguage::Swift => crate::companion_converters::swift_constructor(&fields),
        crate::companion_language::CompanionLanguage::Kotlin => crate::companion_converters::kotlin_constructor(&fields),
        crate::companion_language::CompanionLanguage::CSharp | crate::companion_language::CompanionLanguage::CSharpBrowser | crate::companion_language::CompanionLanguage::Rust => {
            return source.to_owned();
        }
    };
    let body_end = body.as_ptr() as usize - source.as_ptr() as usize + body.len();
    format!(
        "{}{constructor}{}",
        &source[..body_end],
        &source[body_end..]
    )
}
