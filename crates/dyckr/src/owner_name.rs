pub fn owner_name(
    language: crate::companion_language::CompanionLanguage,
    source: &str,
    source_path: &str,
) -> Option<String> {
    let wanted = crate::declared_service::declared_service(source_path, source);
    if language == crate::companion_language::CompanionLanguage::Rust {
        crate::companion_syntax::block_named(source, "struct", wanted.as_deref())
            .map(|(name, _)| name)
    } else {
        crate::companion_syntax::class_body_named(source, wanted.as_deref()).map(|(name, _)| name)
    }
}
