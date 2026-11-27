pub const CLIENT_LANGUAGES: [&'static str; 13] = [
        "java", "js", "ts", "py", "cs", "fs", "rs", "swift", "kt", "go", "dart", "rb", "lua",
    ];
pub const CLIENT_MODELS: &'static str = "client/models";
use std::path::Path;
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
