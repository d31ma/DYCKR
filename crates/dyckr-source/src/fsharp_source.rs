mod behavior_body;
mod fsharp_answer;
mod fsharp_behaviors;
mod fsharp_constructor_props;
mod fsharp_events;
mod fsharp_helper_ownership;
mod fsharp_mapping_models;
mod fsharp_markers;
mod fsharp_project;
mod fsharp_spans;
mod fsharp_style;
#[cfg(test)]
mod fsharp_tac_helper_tests;
mod fsharp_tac_helpers;
mod fsharp_test_construction;
mod fsharp_test_unit;
mod fsharp_view;
mod fsharp_yon;
mod later_features;
mod mapping_record;
mod mapping_shape;

use behavior_body::BehaviorBody;
use dyckr::fsharp_attribute_line::AttributeLine;
pub use dyckr::fsharp_field_needs::FieldNeeds;
pub use dyckr::fsharp_lexer::FsharpLexer;
use dyckr::fsharp_line_kind::LineKind;
pub use dyckr::fsharp_lines::FsharpLines;
pub use dyckr::fsharp_member_line::MemberLine;
use dyckr::fsharp_source_line::SourceLine;
use dyckr::fsharp_type::FsharpType;
use dyckr::fsharp_type_header::TypeHeader;
pub use fsharp_answer::FsharpAnswer;
pub use fsharp_behaviors::FsharpBehaviors;
pub use fsharp_constructor_props::FsharpConstructorProps;
use fsharp_events::FsharpEvents;
use fsharp_helper_ownership::FsharpHelperOwnership;
pub use fsharp_mapping_models::FsharpMappingModels;
pub use fsharp_markers::FsharpMarkers;
pub use fsharp_project::FsharpProject;
pub use fsharp_spans::FsharpSpans;
pub use fsharp_style::FsharpStyle;
pub use fsharp_tac_helpers::FsharpTacHelpers;
use fsharp_test_construction::FsharpTestConstruction;
pub use fsharp_test_unit::FsharpTestUnit;
pub use fsharp_view::FsharpView;
pub use fsharp_yon::FsharpYon;
use later_features::LaterFeatures;

use std::path::{Path, PathBuf};

pub struct FsharpSource;

impl FsharpSource {
    const GENERATED_PART: &'static str = "\n//__YON_CSHARP__\n";
    const WEB_PART: &'static str = "\n//__TAC_FSHARP__\n";
    const RUNTIME_PART: &'static str = "\n//__FSHARP_BEHAVIOR_RUNTIME__\n";
    const MODELS_NAMESPACE: &'static str = "namespace rec Models\n";

    pub fn refuse_unsupported(
        view: &FsharpView,
        source: &Path,
        contents: &str,
    ) -> Result<(), crate::Failure> {
        FieldNeeds::refuse(source, contents)?;
        LaterFeatures::refuse(view, source, contents)
    }

    pub fn with_generated(fsharp: &str, csharp: &str) -> String {
        format!("{fsharp}{}{csharp}", Self::GENERATED_PART)
    }

    pub fn web_body(csharp: &str, fsharp: &str) -> String {
        let joined = (!fsharp.is_empty()).then(|| format!("{csharp}{}{fsharp}", Self::WEB_PART));
        joined.unwrap_or_else(|| csharp.to_owned())
    }

    pub fn web_split(text: &str) -> (&str, &str) {
        text.split_once(Self::WEB_PART).unwrap_or((text, ""))
    }

    pub fn web_namespace(namespace: &str, fsharp: &str) -> String {
        let written =
            (!fsharp.is_empty()).then(|| format!("namespace rec {namespace}\n{fsharp}\n"));
        written.unwrap_or_default()
    }

    pub fn models_file(models: &[(PathBuf, String)]) -> String {
        Self::models_text(models)
    }

    fn models_text(models: &[(PathBuf, String)]) -> String {
        let mut file = String::from(Self::MODELS_NAMESPACE);
        for (path, source) in models {
            file.push_str(&FsharpLines::located(path, source));
            file.push('\n');
        }
        file
    }

    pub fn with_runtime(csharp: &str, runtime: &str) -> String {
        let joined =
            (!runtime.is_empty()).then(|| format!("{csharp}{}{runtime}", Self::RUNTIME_PART));
        joined.unwrap_or_else(|| csharp.to_owned())
    }

    pub fn runtime_namespace(namespace: &str, runtime: &str) -> String {
        let written =
            (!runtime.is_empty()).then(|| format!("namespace {namespace} {{\n{runtime}\n}}\n"));
        written.unwrap_or_default()
    }

    pub fn runtime_split(csharp: &str) -> (&str, &str) {
        csharp
            .split_once(Self::RUNTIME_PART)
            .unwrap_or((csharp, ""))
    }

    pub fn split(program: &str) -> (&str, &str) {
        program
            .split_once(Self::GENERATED_PART)
            .unwrap_or((program, ""))
    }
}

#[cfg(test)]
mod tests {
    use super::FsharpSource;
    use std::path::Path;

    #[test]
    fn the_generated_csharp_follows_the_authored_fsharp() {
        let program =
            FsharpSource::with_generated("type A() = class end", "sealed class Handler {}");
        assert_eq!(
            FsharpSource::split(&program),
            ("type A() = class end", "sealed class Handler {}")
        );
        assert_eq!(FsharpSource::split("type A() = class end").1, "");
    }

    #[test]
    fn csharp_reads_the_fsharp_models_of_its_realm_too() {
        assert_eq!(
            dyckr::dotnet_model_languages::dotnet_model_languages("cs"),
            ["cs", "fs"]
        );
        assert_eq!(
            dyckr::dotnet_model_languages::dotnet_model_languages("kt"),
            ["kt"]
        );
    }

    #[test]
    fn only_fs_files_are_fsharp() {
        assert!(dyckr::fsharp_syntax::is_fsharp(Path::new(
            "server/controllers/A.fs"
        )));
        assert!(!dyckr::fsharp_syntax::is_fsharp(Path::new(
            "server/controllers/A.cs"
        )));
    }
}
