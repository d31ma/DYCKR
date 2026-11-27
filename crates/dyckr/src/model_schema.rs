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

    pub fn polyglot_projection(
        input: &crate::behavior_input::Input<'_>,
        language: &str,
        environment: &str,
        annotations: &dyn Fn(&crate::behavior_input::Input<'_>, &str, &str) -> Result<Vec<crate::behavior_marker::Marker>, crate::failure::Failure>,
        prepare: &dyn Fn(&std::path::Path, &str, &str) -> Result<Option<crate::prepared::Prepared>, crate::failure::Failure>,
    ) -> Result<String, crate::failure::Failure> {
        let markers = annotations(input, language, environment)?;
        Self::project(input, &markers, environment, prepare)
    }

    pub fn project(
        input: &crate::behavior_input::Input<'_>,
        markers: &[crate::behavior_marker::Marker],
        environment: &str,
        prepare: &dyn Fn(&std::path::Path, &str, &str) -> Result<Option<crate::prepared::Prepared>, crate::failure::Failure>,
    ) -> Result<String, crate::failure::Failure> {
        crate::model_schema::require_schema(input, markers)?;
        let prepared = prepare(input.path, input.source, environment)?;
        crate::model_schema::projection(prepared).map_err(|message| input.failure(0, message))
    }


    pub fn script_marked(input: &crate::behavior_input::Input<'_>, markers_fn: &dyn Fn(&crate::behavior_input::Input<'_>) -> Result<Vec<crate::behavior_marker::Marker>, crate::failure::Failure>, lower_fn: &dyn Fn(&std::path::Path, &str, &str) -> Result<Option<String>, crate::failure::Failure>) -> Result<Option<String>, Failure> {
        let markers = markers_fn(input)?;
        let schema = markers
            .iter()
            .any(|marker| crate::schema_marker_names::NAMES.contains(&marker.name.as_str()));
        schema
            .then(|| Self::script_projection(input, &markers, lower_fn))
            .transpose()
    }

    pub fn script_projection(input: &crate::behavior_input::Input<'_>, markers: &[crate::behavior_marker::Marker], lower_fn: &dyn Fn(&std::path::Path, &str, &str) -> Result<Option<String>, crate::failure::Failure>) -> Result<String, crate::failure::Failure> {
        crate::model_schema::require_schema(input, markers)?;
        let environment = ["yon", "tac"][usize::from(input.path.starts_with("client"))];
        lower_fn(input.path, input.source, environment)?;
        Ok(crate::polyglot::erase(input.source, markers))
    }


    pub fn native_plain(path: &Path, source: &str, script_plain: fn(&std::path::Path, &str) -> Result<Option<String>, crate::failure::Failure>, polyglot_plain: fn(&std::path::Path, &str) -> Result<Option<String>, crate::failure::Failure>) -> Result<Option<String>, Failure> {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let project = Self::native_projector(extension, script_plain, polyglot_plain);
        project(path, source)
    }

    pub fn native_projector(extension: &str, script_plain: fn(&std::path::Path, &str) -> Result<Option<String>, crate::failure::Failure>, polyglot_plain: fn(&std::path::Path, &str) -> Result<Option<String>, crate::failure::Failure>) -> fn(&Path, &str) -> Result<Option<String>, Failure> {
        match extension {
            "java" => crate::model_schema::java_plain,
            "js" | "ts" => script_plain,
            _ => polyglot_plain,
        }
    }


    pub fn require_realm(path: &Path, source: &str, schema: bool) -> Result<(), crate::failure::Failure> {
        let unsupported = ![
            schema,
            path.starts_with(crate::model_schema::CLIENT_MODELS),
            !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| crate::model_schema::CLIENT_LANGUAGES.contains(&extension)),
        ]
        .contains(&false);
        unsupported
            .then(|| crate::model_schema::client_failure(path, source))
            .map_or(Ok(()), Err)
    }


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
