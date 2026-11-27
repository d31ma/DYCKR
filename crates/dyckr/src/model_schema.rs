pub const CLIENT_LANGUAGES: [&'static str; 13] = [
        "java", "js", "ts", "py", "cs", "fs", "rs", "swift", "kt", "go", "dart", "rb", "lua",
    ];
pub const CLIENT_MODELS: &'static str = "client/models";
use crate::failure::Failure;
use crate::behavior_input::Input;


pub fn client_failure(path: &Path, source: &str) -> Failure {
        let at = crate::model_schema::marker_offset(path, source).unwrap_or(0);
        Failure::one(crate::failure::diagnostic(
            1310, "This client model schema needs its Tac metadata adapter",
            Some("Plain client models remain supported. Tac reads schema models in JavaScript, TypeScript, Python, Java, C#, F#, Rust, Swift, Kotlin, Go, Dart, Ruby and Lua (ADR0092).".to_owned()),
            crate::failure::source_span(&path.to_string_lossy(), at, at + 1),
        ))
    }

pub fn marker_offset(path: &Path, source: &str) -> Option<usize> {
        let code = crate::source_mask::masked_source(path, source).ok()?;
        crate::schema_marker_names::NAMES
            .iter()
            .flat_map(|name| {
                [
                    format!("@{name}"),
                    format!("[{name}"),
                    format!("[<{name}"),
                    format!("@{}", name.to_lowercase()),
                    format!("@yon::{name}"),
                    format!("@tac::{name}"),
                    format!("[yon::{name}"),
                    format!("[tac::{name}"),
                ]
            })
            .filter_map(|marker| code.find(&marker))
            .min()
    }

pub fn require_schema(input: &Input<'_>, markers: &[crate::behavior_marker::Marker]) -> Result<(), Failure> {
        let refused = markers
            .iter()
            .find(|marker| !crate::schema_marker_names::NAMES.contains(&marker.name.as_str()));
        refused.map_or(Ok(()), |marker| {
            Err(crate::failure::Failure::one(crate::failure::diagnostic(
                1310,
                "A model carries only declarative settings and validation metadata",
                Some(
                    "Move behavior and role annotations into the appropriate role folder."
                        .to_owned(),
                ),
                crate::failure::source_span(
                    &input.path.to_string_lossy(),
                    marker.span.start,
                    marker.span.end,
                ),
            )))
        })
    }
use crate::prepared::Prepared;
use std::path::Path;


    pub fn java_plain(path: &Path, source: &str) -> Result<Option<String>, crate::failure::Failure> {
        let plain = crate::java_model_schema::JavaModelSchema::blanked(path, source)?;
        Ok((plain != source).then_some(plain))
    }

    pub fn projection(prepared: Option<Prepared>) -> Result<String, String> {
        match prepared {
            Some(crate::prepared::Prepared::Annotated(module)) => crate::polyglot::render(&module, Vec::new(), ""),
            Some(crate::prepared::Prepared::Erased(source)) => Ok(source),
            None => Err("Model schema metadata was not consumed".to_owned()),
        }
    }
pub struct ModelSchema;

impl ModelSchema {

    pub fn lua_plain(
        view: &crate::lua_view::LuaView,
        source: &str,
        native_plain: &dyn Fn(&std::path::Path, &str) -> Result<Option<String>, crate::failure::Failure>,
    ) -> Result<Option<String>, crate::failure::Failure> {
        let plain =
            native_plain(view.path(), view.text()).map_err(|error| view.failure(&error))?;
        Ok(plain.map(|_| source.to_owned()))
    }

    pub fn fsharp_plain(
        view: &crate::fsharp_view::FsharpView,
        source: &str,
        native_plain: &dyn Fn(&std::path::Path, &str) -> Result<Option<String>, crate::failure::Failure>,
    ) -> Result<Option<String>, crate::failure::Failure> {
        let plain =
            native_plain(view.path(), view.text()).map_err(|error| view.failure(&error))?;
        Ok(plain.map(|_| crate::fsharp_markers::FsharpMarkers::strip(source, &|name| crate::annotation_rule::AnnotationRules::find(name).is_some())))
    }
}
