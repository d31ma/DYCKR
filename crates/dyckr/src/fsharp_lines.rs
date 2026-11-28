use std::path::Path;

pub struct FsharpLines;

impl FsharpLines {
    const NO_GAP: &'static str = "";
    const LINE_GAP: &'static str = "\n";

    const LOCATIONS: [fn(&Path, &str) -> String; 2] = [Self::unlocated, Self::located];

    pub fn located_in(extension: &str, path: &Path, source: &str) -> String {
        Self::LOCATIONS[usize::from(extension == crate::fsharp_syntax::EXTENSION)](path, source)
    }

    fn unlocated(_path: &Path, source: &str) -> String {
        source.to_owned()
    }

    pub fn hoisted_gap(extension: &str) -> &'static str {
        [Self::NO_GAP, Self::LINE_GAP][usize::from(extension == crate::fsharp_syntax::EXTENSION)]
    }

    pub fn directive(line: usize, file: &str) -> String {
        format!("# {line} \"{file}\"\n")
    }

    pub fn located(path: &Path, source: &str) -> String {
        let file = path.to_string_lossy().replace('\\', "/");
        format!("{}{source}", Self::directive(1, &file))
    }

    pub fn is_directive(trimmed: &str) -> bool {
        trimmed.starts_with('#')
    }
}

#[cfg(test)]
mod tests {
    use crate::fsharp_lines::FsharpLines;
    use std::path::Path;

    #[test]
    fn a_located_module_starts_its_own_line_numbers() {
        assert_eq!(
            FsharpLines::located(Path::new("server/services/A.fs"), "type A() = class end\n"),
            "# 1 \"server/services/A.fs\"\ntype A() = class end\n"
        );
        assert!(FsharpLines::is_directive("# 4 \"server/services/A.fs\""));
        assert!(!FsharpLines::is_directive("member this.A() = 1"));
    }

    #[test]
    fn a_line_directive_inside_a_member_keeps_the_class_open() {
        let source = "[<Service>]\ntype A() =\n    member this.One() : int =\n        BehaviorCalls.Record(\"A.One\")\n# 4 \"server/services/A.fs\"\n        1\n\n    member this.Two() : int = 2\n";
        let shadow = crate::fsharp_shadow::FsharpShadow::of(source);
        assert!(shadow.text().contains("Two("), "{}", shadow.text());
    }
}
