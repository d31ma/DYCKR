use crate::failure::Failure;
use crate::failure::{diagnostic, source_span};
use crate::realm::Realm;
use crate::route_access::RouteAccess;
use crate::topic_access::TopicAccess;
use std::ops::Range;
use std::path::{Component, Path};

pub struct Topics;

impl Topics {
    pub const SPELLINGS: [&'static str; 9] = [
        "@Topic",
        "@Topic()",
        "@topic",
        "@topic()",
        "#[Topic]",
        "#[Topic()]",
        "[Topic]",
        "[<Topic>]",
        "[<Topic()>]",
    ];
    pub const MAX_SLUG_BYTES: usize = 64;
    pub const CODE: u16 = 1308;
    pub const MISSING_CODE: u16 = 1322;
    pub const ANNOTATION: &'static str = "Topic";
    pub const HELP: &'static str = "Write @Topic on its own line above a model in a models folder, for example 'server/models/OrderPlaced.py'. JavaScript writes it inside the model's @typedef block (ADR 0061).";
    pub const MISSING_HELP: &'static str = "Tachyon refuses every topic stream by default (ADR 0061, ADR 0068). Next to @Topic, write @Public when anyone may read the topic stream, @SignedIn when any signed-in user may read it, or @Allow(Role) when only some roles may read it.";
    pub const ACCESS_HELP: &'static str = "Write one of @Public, @SignedIn or @Allow(Role) next to @Topic on the server model, in the spelling of its language: for example `// @SignedIn` in Go, `/** @SignedIn */` in TypeScript, `#[SignedIn]` in Rust, `[SignedIn]` in C# and `@signed_in` in Python (ADR 0061).";
    pub const CLIENT_ACCESS: &'static str = "A page cannot decide who reads a topic. Write the access rule on the server model of this topic, in server/models";

    pub fn markers(source: &str) -> Vec<Range<usize>> {
        let mut offset = 0;
        source
            .split_inclusive('\n')
            .filter_map(|line| {
                let start = offset;
                offset += line.len();
                Self::marker_in(line).map(|marker| start + marker.start..start + marker.end)
            })
            .collect()
    }

    pub fn marker_in(line: &str) -> Option<Range<usize>> {
        let text = TopicAccess::uncommented(line);
        Self::SPELLINGS.contains(&text).then_some(())?;
        let start = line.find(text)?;
        Some(start..start + text.len())
    }

    pub fn blanked(source: &str) -> String {
        let mut bytes = source.as_bytes().to_vec();
        Self::markers(source)
            .into_iter()
            .flat_map(|marker| {
                let access = TopicAccess::spans(source, &marker);
                std::iter::once(marker).chain(access)
            })
            .for_each(|span| bytes[span].fill(b' '));
        String::from_utf8(bytes).unwrap_or_else(|_| source.to_owned())
    }

    pub fn named(source: &str, declared: &[String]) -> Vec<String> {
        Self::markers(source)
            .into_iter()
            .filter_map(|marker| Self::next_declared(&source[marker.end..], declared))
            .collect()
    }

    pub fn rules(path: &Path, source: &str, declared: &[String]) -> Vec<(String, RouteAccess)> {
        let extension = Self::extension(path);
        Self::markers(source)
            .into_iter()
            .filter_map(|marker| Self::rule_of(extension, source, &marker, declared))
            .collect()
    }

    pub fn rule_of(
        extension: &str,
        source: &str,
        marker: &Range<usize>,
        declared: &[String],
    ) -> Option<(String, RouteAccess)> {
        let name = Self::next_declared(&source[marker.end..], declared)?;
        let access = TopicAccess::rule(extension, source, marker).ok()?;
        Some((name, access))
    }

    pub fn extension(path: &Path) -> &str {
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("")
    }

    pub fn next_declared(rest: &str, declared: &[String]) -> Option<String> {
        let mut first: Option<(usize, &String)> = None;
        for name in declared {
            let Some(at) = Self::word_at(rest, name) else {
                continue;
            };
            if first.is_none_or(|(earliest, _)| at < earliest) {
                first = Some((at, name));
            }
        }
        first.map(|(_, name)| name.clone())
    }

    pub fn word_at(text: &str, word: &str) -> Option<usize> {
        let is_word = |character: char| character.is_ascii_alphanumeric() || character == '_';
        for (at, _) in text.match_indices(word) {
            let before = text[..at].chars().next_back();
            let after = text[at + word.len()..].chars().next();
            if !before.is_some_and(is_word) && !after.is_some_and(is_word) {
                return Some(at);
            }
        }
        None
    }

    pub fn slug(name: &str) -> Option<String> {
        let mut characters = name.chars();
        let first = characters.next()?;
        if !first.is_ascii_uppercase() {
            return None;
        }
        let mut slug = String::from(first.to_ascii_lowercase());
        for character in characters {
            if character.is_ascii_uppercase() {
                slug.push('-');
                slug.push(character.to_ascii_lowercase());
            } else if character.is_ascii_lowercase() || character.is_ascii_digit() {
                slug.push(character);
            } else {
                return None;
            }
        }
        (slug.len() <= Self::MAX_SLUG_BYTES).then_some(slug)
    }

    pub fn check_access(
        path: &Path,
        source: &str,
        marker: &Range<usize>,
        name: &str,
    ) -> Result<(), Failure> {
        let rule = TopicAccess::rule(Self::extension(path), source, marker)
            .map_err(|(at, message)| Self::access_failure(path, source, at, message))?;
        match (Realm::of(path), rule) {
            (Some(Realm::Yon), RouteAccess::Missing) => {
                Err(Self::missing(path, source, marker.start, name))
            }
            (Some(Realm::Tac), RouteAccess::Missing) | (Some(Realm::Yon) | None, _) => Ok(()),
            (Some(Realm::Tac), _) => {
                let at = TopicAccess::spans(source, marker)
                    .first()
                    .map_or(marker.start, |span| span.start);
                Err(Self::access_failure(path, source, at, Self::CLIENT_ACCESS))
            }
        }
    }

    pub fn missing(path: &Path, source: &str, at: usize, name: &str) -> Failure {
        let message = format!("The topic {name} does not say who may read its stream");
        Self::located(
            (Self::MISSING_CODE, Self::MISSING_HELP),
            path,
            source,
            at,
            &message,
        )
    }

    pub fn access_failure(path: &Path, source: &str, at: usize, message: &str) -> Failure {
        Self::located((Self::CODE, Self::ACCESS_HELP), path, source, at, message)
    }

    pub fn in_models_folder(path: &Path) -> bool {
        let mut components = path.components();
        let realm = components.next();
        let folder = components.next();
        let is_realm =
            matches!(realm, Some(Component::Normal(name)) if name == "server" || name == "client");
        let is_models = matches!(folder, Some(Component::Normal(name)) if name == "models");
        is_realm && is_models
    }

    pub fn failure(path: &Path, source: &str, at: usize, message: &str) -> Failure {
        Self::located((Self::CODE, Self::HELP), path, source, at, message)
    }

    pub fn located(
        (code, help): (u16, &str),
        path: &Path,
        source: &str,
        at: usize,
        message: &str,
    ) -> Failure {
        let portable = path.to_string_lossy().replace('\\', "/");
        let line = source[..at.min(source.len())].matches('\n').count() + 1;
        Failure::one(diagnostic(
            code,
            format!("{message} ({portable}:{line})"),
            Some(String::from(help)),
            source_span(&portable, at, (at + 1).min(source.len())),
        ))
    }
}
