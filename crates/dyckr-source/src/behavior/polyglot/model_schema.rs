use super::Input;
use crate::Failure;
use std::path::Path;

pub struct ModelSchema;

impl ModelSchema {
    pub fn plain(path: &Path, source: &str) -> Result<Option<String>, Failure> {
        crate::lua_source::LuaView::of(path, source).map_or_else(
            || Self::other_plain(path, source),
            |view| {
                dyckr::model_schema::ModelSchema::lua_plain(&view, source, &|p, s| {
                    dyckr::model_schema::ModelSchema::native_plain(
                        p,
                        s,
                        Self::script_plain,
                        Self::polyglot_plain,
                    )
                })
            },
        )
    }

    fn other_plain(path: &Path, source: &str) -> Result<Option<String>, Failure> {
        crate::fsharp_source::FsharpView::of(path, source).map_or_else(
            || {
                dyckr::model_schema::ModelSchema::native_plain(
                    path,
                    source,
                    Self::script_plain,
                    Self::polyglot_plain,
                )
            },
            |view| {
                dyckr::model_schema::ModelSchema::fsharp_plain(&view, source, &|p, s| {
                    dyckr::model_schema::ModelSchema::native_plain(
                        p,
                        s,
                        Self::script_plain,
                        Self::polyglot_plain,
                    )
                })
            },
        )
    }

    fn script_plain(path: &Path, source: &str) -> Result<Option<String>, Failure> {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let mask = dyckr::lexical::code_mask(extension, source);
        let input = Input {
            path,
            source,
            code: dyckr::visible_code::visible_code(source, &mask.code),
        };
        crate::annotation_layout::SchemaMarkers::contains(path, source)?
            .then(|| {
                dyckr::model_schema::ModelSchema::script_marked(
                    &input,
                    &|i| super::super::markers(i),
                    &|p, s, e| super::super::lower(p, s, e),
                )
            })
            .transpose()?
            .map_or(Ok(None), Ok)
    }

    fn polyglot_plain(path: &Path, source: &str) -> Result<Option<String>, Failure> {
        let language = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let mask = dyckr::lexical::code_mask(language, source);
        let language = dyckr::groovy_source::GroovySource::syntax(language);
        let input = Input {
            path,
            source,
            code: super::visible_code(source, mask.code),
        };
        let environment = ["yon", "tac"][usize::from(path.starts_with("client"))];
        crate::annotation_layout::SchemaMarkers::contains(path, source)?
            .then(|| {
                dyckr::model_schema::ModelSchema::polyglot_projection(
                    &input,
                    language,
                    environment,
                    &|i, l, e| super::annotations(i, l, e),
                    &|p, s, e| super::prepare(p, s, e),
                )
            })
            .transpose()
    }
}
