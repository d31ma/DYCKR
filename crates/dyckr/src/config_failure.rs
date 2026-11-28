use crate::failure::{Failure, diagnostic, source_span};

pub fn config_failure(message: &str) -> Failure {
    let file_name = "client/manifest.json";
    Failure::one(diagnostic(
        1601,
        message,
        Some(String::from(
            "Correct client/manifest.json. Its tac member holds Tachyon's settings: origin, application, scripts, styles, cache, metadata and sitemap.",
        )),
        source_span(file_name, 0, file_name.len()),
    ))
}
