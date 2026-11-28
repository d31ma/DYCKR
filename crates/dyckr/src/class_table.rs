pub fn csharp_required_members(body: &str) -> Vec<String> {
    body.lines()
        .filter(|line| !line.contains('('))
        .filter_map(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let at = words.iter().position(|word| *word == "required")?;
            let name = crate::unicode_name::UnicodeName::leading(words.get(at + 2)?);
            (!name.is_empty()).then_some(name)
        })
        .collect()
}

pub fn class_table(
    language: crate::companion_language::CompanionLanguage,
    name: &str,
    entries: &str,
    required: &[String],
    constructs: bool,
) -> String {
    match language {
        crate::companion_language::CompanionLanguage::Swift => {
            let instance = if constructs {
                format!("guard let tacInstance = {name}(tacProps: tacProps) else {{ return nil }}")
            } else {
                format!("let tacInstance = {name}()")
            };
            format!(
                "\nstatic func tacCreate(_ tacProps: [String: Any]) -> [String: TacMember]? {{\n{instance}\nreturn [\n{entries}]\n}}\n"
            )
        }
        crate::companion_language::CompanionLanguage::Kotlin => {
            let arguments = if constructs { "tacProps, tacPage" } else { "" };
            format!(
                "\nfun tacCreate(tacProps: Map<String, Any?>?, tacPage: TacPageScope): Map<String, Any> {{\nval tacInstance = {name}({arguments})\nreturn linkedMapOf(\n{entries})\n}}\n"
            )
        }
        crate::companion_language::CompanionLanguage::CSharp
        | crate::companion_language::CompanionLanguage::CSharpBrowser => {
            let filled = required
                .iter()
                .map(|member| format!("{member} = default!"))
                .collect::<Vec<_>>();
            let initializer = if filled.is_empty() {
                String::new()
            } else {
                format!(" {{ {} }}", filled.join(", "))
            };
            format!(
                "\npublic static class TacBridge {{\npublic static Dictionary<string, TacMember> Create() {{\nvar Instance = new {name}(){initializer};\nreturn new() {{\n{entries}}};\n}}\n}}\n"
            )
        }
        crate::companion_language::CompanionLanguage::Rust => String::new(),
    }
}
