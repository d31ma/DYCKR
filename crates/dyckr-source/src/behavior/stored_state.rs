use crate::Failure;
use dyckr::behavior_input::Input;
use std::collections::BTreeMap;
use std::path::Path;

pub struct StoredState;

impl StoredState {
    pub fn fields(path: &Path, source: &str) -> Result<BTreeMap<String, String>, Failure> {
        dyckr::cpp_source::CppSource::either(
            path,
            source,
            crate::tac_cpp::CppRoles::stored,
            |path, source| crate::lua_source::LuaView::either(path, source, Self::written_fields),
        )
    }

    fn written_fields(path: &Path, source: &str) -> Result<BTreeMap<String, String>, Failure> {
        let language = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if !dyckr::stored_state::StoredState::mentioned(source) {
            return Ok(BTreeMap::new());
        }
        let mask = dyckr::lexical::code_mask(language, source);
        let input = Input {
            path,
            source,
            code: dyckr::visible_code::visible_code(source, &mask.code),
        };
        let markers = if matches!(language, "js" | "ts") {
            super::markers(&input)?
        } else {
            super::polyglot::annotations(&input, language, "tac")?
        };
        let mut fields = BTreeMap::new();
        for marker in markers
            .iter()
            .filter(|marker| dyckr::stored_state::StoredState::marks(&marker.name))
        {
            let name = super::validation::field_name(&input, marker, language)?;
            fields.insert(
                dyckr::stored_state::StoredState::visible_name(language, &name),
                dyckr::stored_state::StoredState::area(marker),
            );
        }
        Ok(fields)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::StoredState;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn fields(file: &str, source: &str) -> BTreeMap<String, String> {
        StoredState::fields(&Path::new("client/services").join(file), source).expect("fields")
    }

    fn stored(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(name, area)| ((*name).to_owned(), (*area).to_owned()))
            .collect()
    }

    #[test]
    fn every_tac_behavior_language_marks_local_and_session_fields() {
        let wanted = stored(&[("draft", "session"), ("theme", "local")]);
        let sources = [
            (
                "EditorService.js",
                "@Service\nexport default class EditorService {\n  @SessionState\n  draft = ''\n  @LocalState\n  theme = 'light'\n}\n",
            ),
            (
                "EditorService.ts",
                "@Service\nexport default class EditorService {\n  @SessionState\n  draft: string = ''\n  @LocalState\n  theme = 'light'\n}\n",
            ),
            (
                "EditorService.dart",
                "@Service\nclass EditorService {\n  @SessionState\n  String draft = '';\n  @LocalState\n  String theme = 'light';\n}\n",
            ),
            (
                "EditorService.kt",
                "class EditorService {\n  @SessionState\n  var draft = \"\"\n  @LocalState\n  var theme: String = \"light\"\n}\n",
            ),
            (
                "EditorService.swift",
                "final class EditorService {\n  @SessionState\n  var draft = \"\"\n  @LocalState\n  var theme: String = \"light\"\n}\n",
            ),
            (
                "EditorService.cs",
                "public sealed class EditorService {\n    [SessionState]\n    public string Draft = \"\";\n    [LocalState]\n    public string theme = \"light\";\n}\n",
            ),
            (
                "EditorService.rs",
                "struct EditorService {\n    #[SessionState]\n    draft: String,\n    #[LocalState]\n    theme: String,\n}\n",
            ),
            (
                "EditorService.go",
                "type EditorService struct {\n\t// @SessionState\n\tDraft string\n\t// @LocalState\n\tTheme string\n}\n",
            ),
            (
                "EditorService.rb",
                "#[Service]\nclass EditorService\n  #[SessionState]\n  attr_accessor :draft\n  #[LocalState]\n  attr_accessor :theme\nend\n",
            ),
            (
                "EditorService.java",
                "@Service\npublic class EditorService {\n    @SessionState\n    private String draft = \"\";\n    @LocalState\n    private String theme = \"light\";\n}\n",
            ),
            (
                "EditorService.fs",
                "[<Service>]\ntype EditorService() =\n    [<SessionState>]\n    member val Draft = \"\" with get, set\n\n    [<LocalState>]\n    member val theme : string = \"light\" with get, set\n",
            ),
        ];
        for (file, source) in sources {
            assert_eq!(fields(file, source), wanted, "{file}");
        }
    }

    #[test]
    fn python_fields_use_their_lower_camel_page_names() {
        assert_eq!(
            fields(
                "EditorService.py",
                "from typing import Annotated\n\n@service\nclass EditorService:\n    draft: Annotated[str, session_state] = ''\n    dark_mode: Annotated[bool, local_state] = False\n",
            ),
            stored(&[("darkMode", "local"), ("draft", "session")])
        );
    }

    #[test]
    fn ruby_fields_use_their_lower_camel_page_names() {
        assert_eq!(
            fields(
                "EditorService.rb",
                "#[Service]\nclass EditorService\n  #[LocalState]\n  attr_accessor :dark_mode\nend\n",
            ),
            stored(&[("darkMode", "local")])
        );
    }

    #[test]
    fn a_service_without_markers_keeps_nothing() {
        assert!(
            fields(
                "PlainService.js",
                "export default class PlainService { theme = 'x' }\n"
            )
            .is_empty()
        );
    }

    #[test]
    fn dollar_prefixed_fields_are_refused_with_the_marker_to_write() {
        let cases = [
            (
                "EditorService.js",
                "export default class EditorService {\n  $$theme = 'light'\n}\n",
                "@LocalState",
                "`theme`",
            ),
            (
                "EditorService.ts",
                "export default class EditorService {\n  $draft: string = ''\n}\n",
                "@SessionState",
                "`draft`",
            ),
            (
                "EditorService.dart",
                "class EditorService {\n  String $$theme = 'light';\n}\n",
                "@LocalState",
                "`theme`",
            ),
        ];
        for (file, source, marker, name) in cases {
            let failure = dyckr::stored_state::StoredState::refuse_sigils(
                &Path::new("client/services").join(file),
                source,
            )
            .expect_err("refused");
            let message = &failure.diagnostics()[0].message;
            assert!(
                message.contains(marker) && message.contains(name),
                "{message}"
            );
            assert!(message.contains(":2)"), "{message}");
        }
    }

    #[test]
    fn dollar_names_inside_methods_and_strings_are_not_fields() {
        let source = "export default class EditorService {\n  label = '$$theme'\n  load() {\n    const $el = 1\n    this.$$theme = $el\n  }\n}\n";
        dyckr::stored_state::StoredState::refuse_sigils(
            Path::new("client/services/EditorService.js"),
            source,
        )
        .expect("plain source");
    }
}
