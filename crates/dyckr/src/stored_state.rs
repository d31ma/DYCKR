pub struct SigilField {
    pub line_start: usize,
    pub indent: usize,
    pub at: usize,
    pub written: String,
}
pub struct StoredState;

impl StoredState {
    pub const LOCAL: &'static str = "LocalState";
    pub const SESSION: &'static str = "SessionState";
    pub const SPELLINGS: [&'static str; 4] =
        ["LocalState", "SessionState", "local_state", "session_state"];
    pub const SIGIL_LANGUAGES: [&'static str; 3] = ["js", "ts", "dart"];

    pub fn marks(name: &str) -> bool {
        name == Self::LOCAL || name == Self::SESSION
    }

}
use crate::behavior_marker::Marker;

impl StoredState {

    pub fn mentioned(source: &str) -> bool {
        crate::stored_state::StoredState::SPELLINGS
            .iter()
            .any(|spelling| source.contains(spelling))
    }

    pub fn area(marker: &Marker) -> String {
        if marker.name == crate::stored_state::StoredState::LOCAL {
            String::from("local")
        } else {
            String::from("session")
        }
    }

    pub fn visible_name(language: &str, name: &str) -> String {
        match language {
            "cs" | "go" => crate::lower_first::lower_first(name),
            "rs" | "rb" | "py" => crate::companion_names::camel_case(name),
            _ => name.to_owned(),
        }
    }
}
use std::path::Path;
use crate::failure::{diagnostic, source_span};

impl StoredState {

    pub fn marker_for(written: &str) -> &'static str {
        if written.starts_with("$$") {
            crate::stored_state::StoredState::LOCAL
        } else {
            crate::stored_state::StoredState::SESSION
        }
    }

    pub fn sigil_field(line: &str) -> Option<(usize, &str)> {
        let declaration = line.split(['=', ';', ':']).next().unwrap_or_default();
        if declaration.contains(['(', '{', '}', '.']) {
            return None;
        }
        let word = declaration
            .split_whitespace()
            .last()?
            .trim_end_matches(['!', '?']);
        let bare = word.trim_start_matches('$');
        let sigils = word.len() - bare.len();
        let named = bare
            .chars()
            .next()
            .is_some_and(|first| first.is_alphabetic() || first == '_')
            && bare
                .chars()
                .all(|character| character.is_alphanumeric() || character == '_');
        if !(1..=2).contains(&sigils) || !named {
            return None;
        }
        Some((line.find(word)?, word))
    }

    pub fn sigil_failure(path: &Path, source: &str, at: usize, written: &str) -> crate::failure::Failure {
        let bare = written.trim_start_matches('$');
        let marker = crate::stored_state::StoredState::marker_for(written);
        let portable = path.to_string_lossy().replace('\\', "/");
        let line = source[..at.min(source.len())].matches('\n').count() + 1;
        crate::failure::Failure::one(diagnostic(
            1306,
            format!(
                "The field `{written}` uses a `$` prefix to keep its value, and Tac no longer reads that prefix. Write `@{marker}` on the line above and name the field `{bare}` ({portable}:{line})"
            ),
            Some(String::from(
                "@SessionState keeps a field in sessionStorage (the old `$name`). @LocalState keeps it in localStorage (the old `$$name`). `ty migrate fix --write` converts the fields and the views that read them (ADR 0059).",
            )),
            source_span(&portable, at, (at + written.len()).min(source.len())),
        ))
    }
}
