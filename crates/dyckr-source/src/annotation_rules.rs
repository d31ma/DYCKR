mod field_case;
mod field_declaration;
mod field_name;
mod injected_field;
mod needed_field;
mod nullable_field;
mod placed_marker;
pub mod required_fields;
mod scan;

pub use needed_field::NeededField;
pub use placed_marker::PlacedMarker;
pub use scan::Messages;

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests;

#[cfg(test)]
#[allow(clippy::expect_used)]
mod injected_field_tests;

use crate::Failure;
use dyckr::annotation_rule::{AnnotationRules, Rule};
use dyckr::annotation_target::{Arguments, Target};
use dyckr::failure::{diagnostic, source_span};
use dyckr::realm::{LANGUAGES, Realm};
use std::path::Path;

pub trait AnnotationRulesExt {
    fn check(path: &Path, source: &str) -> Result<(), Failure> {
        crate::tac_cpp::CppMarkerCheck::either(path, source, |path, source| {
            crate::fsharp_source::FsharpView::run(path, source, Self::check_direct).map_or_else(
                || Self::check_direct(path, source),
                |checked| checked.and_then(|()| Self::check_fsharp(path, source)),
            )
        })
    }

    fn check_fsharp(path: &Path, source: &str) -> Result<(), Failure> {
        let view = crate::fsharp_source::FsharpView::unmoved(path, source);
        Self::check_fsharp_reach(view.path(), view.text()).map_err(|failure| view.failure(&failure))
    }

    fn check_fsharp_reach(path: &Path, source: &str) -> Result<(), Failure> {
        let found = scan::Scan::run_as("cs", "cs", source);
        let outside = Realm::of(path)
            .and_then(|realm| found.outside_reach("fs", realm).map(|hit| (hit, realm)));
        outside.map_or(Ok(()), |((at, rule), realm)| {
            let message = scan::Messages::unavailable(rule, "cs", realm);
            Err(Self::failure_at(path, source, at, rule.code, &message))
        })
    }

    fn check_lua_shadow(path: &Path, source: &str) -> Result<(), Failure> {
        Self::check_as(path, source, dyckr::lua_syntax::EXTENSION)
    }

    fn check_direct(path: &Path, source: &str) -> Result<(), Failure> {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        crate::lua_source::LuaView::of(path, source).map_or_else(
            || Self::check_as(path, source, extension),
            |view| {
                Self::check_as(view.path(), view.text(), dyckr::lua_syntax::EXTENSION)
                    .map_err(|failure| view.failure(&failure))
            },
        )
    }

    fn check_as(path: &Path, source: &str, spelling: &str) -> Result<(), Failure> {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if !LANGUAGES.contains(&extension) {
            return Ok(());
        }
        let found = scan::Scan::run_as(extension, spelling, source);
        let realm = Realm::of(path);
        let problem = found
            .problem(realm)
            .map(|(at, rule, message)| (at, rule.code, message))
            .or_else(|| found.nullable_problem(source, realm))
            .or_else(|| found.injected_problem(source, realm));
        let Some((at, code, message)) = problem else {
            return Ok(());
        };
        Err(Self::failure_at(path, source, at, code, &message))
    }

    fn failure_at(path: &Path, source: &str, at: usize, code: u16, message: &str) -> Failure {
        let portable = path.to_string_lossy().replace('\\', "/");
        let line = source[..at.min(source.len())].matches('\n').count() + 1;
        Failure::one(diagnostic(
            code,
            format!("{} ({portable}:{line})", message.trim_end_matches('.')),
            Some(String::from(
                "Each annotation checks the class, method, field or parameter it is written on (ADR 0056). docs/BEHAVIOR_ANNOTATIONS.md lists where each one goes.",
            )),
            source_span(&portable, at, (at + 1).min(source.len())),
        ))
    }

    fn file_scan(path: &Path, source: &str) -> scan::Scan {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        scan::Scan::run(extension, source)
    }

    fn injected_fields(path: &Path, source: &str) -> Vec<injected_field::InjectedField> {
        Self::file_scan(path, source).injected_fields(source, Realm::of(path))
    }

    fn needed_fields(path: &Path, source: &str) -> Vec<NeededField> {
        NeededField::through_view(path, source)
            .unwrap_or_else(|| Self::file_scan(path, source).needed_fields(source, Realm::of(path)))
    }

    fn nullable_fields(path: &Path, source: &str) -> Vec<nullable_field::NullableField> {
        Self::file_scan(path, source).nullable_fields(source, Realm::of(path))
    }

    fn framework_members(path: &Path, source: &str) -> Vec<usize> {
        Self::file_scan(path, source).framework_members(source, Realm::of(path))
    }

    fn python_parameter(name: &str) -> String {
        match dyckr::annotation_rule::AnnotationRules::find(name) {
            Some(rule) => scan::Messages::python_annotated(rule, Target::Parameter),
            None => format!("{name} goes inside the parameter's type in Python"),
        }
    }

    fn needs_all_constructor(language: &str) -> String {
        dyckr::annotation_rule::AnnotationRules::find("NeedsAll").map_or_else(String::new, |rule| {
            scan::Messages::constructor(rule, language)
        })
    }

    fn no_arguments(name: &str) -> String {
        match dyckr::annotation_rule::AnnotationRules::find(name) {
            Some(rule) if !rule.argument_advice.is_empty() => {
                format!("{}. {}", rule.argument_advice, rule.explained())
            }
            Some(rule) => format!(
                "{name} accepts no arguments: write it alone. {}",
                rule.explained()
            ),
            None => format!("{name} accepts no arguments"),
        }
    }

    fn belongs(name: &str) -> String {
        match dyckr::annotation_rule::AnnotationRules::find(name) {
            Some(rule) if !rule.is_retired() => format!(
                "{name} belongs on {}. {}",
                rule.allowed_targets(),
                rule.explained()
            ),
            _ => format!("{name} is not written here"),
        }
    }

    fn checked_refusal(name: &str, language: &str, realm: Realm) -> Option<String> {
        let rule = dyckr::annotation_rule::AnnotationRules::find(name)?;
        let checked = realm.languages().contains(&language);
        (checked && !rule.allows(language, realm))
            .then(|| scan::Messages::unavailable(rule, language, realm))
    }

    fn unavailable(name: &str, language: &str, environment: &str) -> Option<String> {
        let rule = dyckr::annotation_rule::AnnotationRules::find(name)?;
        let realm = Realm::named(environment)?;
        if rule.allows(language, realm) {
            return None;
        }
        Some(scan::Messages::unavailable(rule, language, realm))
    }
}

impl AnnotationRulesExt for AnnotationRules {}
