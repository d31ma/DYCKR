
pub fn declares_table(language: crate::companion_language::CompanionLanguage, source: &str) -> bool {
    let prefixes: &[&str] = match language {
        crate::companion_language::CompanionLanguage::Swift => &["let tac", "var tac"],
        crate::companion_language::CompanionLanguage::Kotlin => &["val tac", "var tac"],
        crate::companion_language::CompanionLanguage::CSharp | crate::companion_language::CompanionLanguage::CSharpBrowser => &["class TacBridge"],
        crate::companion_language::CompanionLanguage::Rust => &["fn tac()"],
    };
    for line in source.lines() {
        let line = line.trim_start();
        if prefixes.iter().any(|prefix| line.starts_with(prefix)) {
            return true;
        }
    }
    false
}
