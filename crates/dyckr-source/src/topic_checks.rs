use crate::Failure;
use crate::annotation_rules::AnnotationRulesExt;
use dyckr::failure::{diagnostic, source_span};
use dyckr::realm::Realm;
use dyckr::topics::Topics;
use std::path::Path;

pub fn check(path: &Path, source: &str, declared: &[String]) -> Result<(), Failure> {
    let markers = Topics::markers(source);
    let Some(first) = markers.first() else {
        return Ok(());
    };
    if let Some(refusal) = refusal(path) {
        return Err(refused(path, source, first.start, &refusal));
    }
    let is_javascript = path.extension().is_some_and(|extension| extension == "js");
    if !is_javascript && !Topics::in_models_folder(path) {
        return Err(Topics::failure(
            path,
            source,
            first.start,
            "Topic goes on a model in a models folder",
        ));
    }
    for marker in &markers {
        let Some(name) = Topics::next_declared(&source[marker.end..], declared) else {
            return Err(Topics::failure(
                path,
                source,
                marker.start,
                "Topic must be written above a model declaration",
            ));
        };
        if Topics::slug(&name).is_none() {
            return Err(Topics::failure(
                path,
                source,
                marker.start,
                "A Topic model name starts with a capital letter, uses only letters and digits, and is short (its topic name is at most 64 bytes)",
            ));
        }
        Topics::check_access(path, source, marker, &name)?;
    }
    Ok(())
}

fn refusal(path: &Path) -> Option<String> {
    let language = path.extension()?.to_str()?;
    let realm = Realm::of(path)?;
    dyckr::annotation_rule::AnnotationRules::checked_refusal(Topics::ANNOTATION, language, realm)
}

fn refused(path: &Path, source: &str, at: usize, refusal: &str) -> Failure {
    let portable = path.to_string_lossy().replace('\\', "/");
    Failure::one(diagnostic(
        Topics::CODE,
        format!("{refusal} ({portable})"),
        None,
        source_span(&portable, at, (at + 1).min(source.len())),
    ))
}
