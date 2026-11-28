use crate::companion_failures::unnamed_service;
use crate::companion_language::CompanionLanguage;
use crate::companion_member::{Member, members};
use crate::companion_syntax::{class_body_named, closing_parenthesis, parameter_type};
use crate::declared_service::declared_service;
use crate::failure::Failure;
use crate::failure::{diagnostic, source_span};
use crate::kotlin_member::KotlinMember;
use std::collections::BTreeSet;
use std::path::Path;

pub struct KotlinMembers;

impl KotlinMembers {
    pub fn owned(
        path: &Path,
        source: &str,
        excluded: &BTreeSet<String>,
    ) -> Result<Vec<KotlinMember>, Failure> {
        let shown = path.to_string_lossy();
        let wanted = declared_service(&shown, source);
        let (_, body) = class_body_named(source, wanted.as_deref())
            .ok_or_else(|| unnamed_service(&shown, wanted.as_deref(), "class"))?;
        let found = members(CompanionLanguage::Kotlin, body, &shown)?;
        found
            .iter()
            .filter(|member| Self::visible(member, excluded))
            .map(|member| Self::typed(path, source, member))
            .collect()
    }

    pub fn visible(member: &Member, excluded: &BTreeSet<String>) -> bool {
        !excluded.contains(&member.name) && !Self::publisher(member)
    }

    pub fn publisher(member: &Member) -> bool {
        member
            .declared_type
            .as_deref()
            .map(str::trim)
            .map(|kind| kind.trim_end_matches('?'))
            == Some("ApplicationEventPublisher")
    }

    pub fn typed(path: &Path, source: &str, member: &Member) -> Result<KotlinMember, Failure> {
        let value_type = Self::declared_type(source, member)
            .ok_or_else(|| Self::unknown_type(path, source, member))?;
        Self::require_type(path, source, member, &value_type)?;
        let parameters = member
            .parameters
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|parameter| String::from(parameter_type(parameter)))
            .collect();
        Ok(KotlinMember {
            name: member.name.clone(),
            value_type,
            parameters,
            method: member.method,
            mutable: member.mutable,
        })
    }

    pub fn declared_type(source: &str, member: &Member) -> Option<String> {
        member
            .declared_type
            .clone()
            .or_else(|| Self::block_unit(source, member))
    }

    pub fn block_unit(source: &str, member: &Member) -> Option<String> {
        Self::method_signature(source, member).and_then(Self::signature_unit)
    }

    pub fn method_signature<'a>(source: &'a str, member: &Member) -> Option<&'a str> {
        member
            .method
            .then(|| source.get(Self::member_at(source, member) + member.name.len()..))
            .flatten()
    }

    pub fn signature_unit(signature: &str) -> Option<String> {
        let close = closing_parenthesis(signature)?;
        signature[close + 1..]
            .trim_start()
            .starts_with('{')
            .then(|| String::from("Unit"))
    }

    pub fn require_type(
        path: &Path,
        source: &str,
        member: &Member,
        kind: &str,
    ) -> Result<(), Failure> {
        Self::shape(kind).map_err(|message| Self::failure(path, source, member, &message))
    }

    pub fn shape(kind: &str) -> Result<(), String> {
        if kind == "Unit" {
            return Ok(());
        }
        crate::route_types::TypeText::shape("kt", kind).map(|_| ())
    }

    pub fn unknown_type(path: &Path, source: &str, member: &Member) -> Failure {
        Self::failure(
            path,
            source,
            member,
            "Kotlin browser facade metadata requires an explicit public member type; inferred returns and values remain supported by the existing runtime, but cannot yet provide composed types.",
        )
    }

    pub fn failure(path: &Path, source: &str, member: &Member, message: &str) -> Failure {
        let at = Self::member_at(source, member);
        Failure::one(diagnostic(
            1405,
            message,
            Some(String::from(
                "Declare the Kotlin member's return or property type explicitly before using composed type generation.",
            )),
            source_span(&path.to_string_lossy(), at, at + member.name.len()),
        ))
    }

    pub fn member_at(source: &str, member: &Member) -> usize {
        let prefix = if member.method { "fun " } else { "" };
        source
            .find(&format!("{prefix}{}", member.name))
            .map_or(0, |at| at + prefix.len())
    }
}
