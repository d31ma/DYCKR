use crate::companion_member::{
    Member, csharp_arguments, csharp_parameters, without_type_first_modifiers,
};
use crate::companion_syntax::class_body_named;
use crate::csharp_model_type::CsharpModelType;
use crate::declared_service::declared_service;
use crate::failure::Failure;
use crate::failure::{diagnostic, source_span};
use crate::lower_first::lower_first;
use crate::route_types::ModelTable;
use crate::unicode_name::UnicodeName;

pub struct CsharpTaskMembers;

impl CsharpTaskMembers {
    pub fn entries(
        source: &str,
        path: &str,
        models: &ModelTable,
        excluded: &[String],
    ) -> Result<String, Failure> {
        Self::declared_entries(source, path, (models, excluded))
    }

    pub fn declared_entries(
        source: &str,
        path: &str,
        context: (&ModelTable, &[String]),
    ) -> Result<String, Failure> {
        let wanted = declared_service(path, source);
        let code = crate::source_mask::masked_source(std::path::Path::new(path), source)?;
        let body = class_body_named(&code, wanted.as_deref()).map_or("", |(_, body)| body);
        let mut entries = String::new();
        let mut depth = 0_i32;
        for line in body.lines() {
            let entering = depth;
            depth = depth.saturating_add(Self::brace_change(line));
            Self::append(&mut entries, (entering, line), (&code, path), context)?;
        }
        Ok(entries)
    }

    pub fn brace_change(line: &str) -> i32 {
        Self::brace_count(line, '{') - Self::brace_count(line, '}')
    }

    pub fn brace_count(line: &str, brace: char) -> i32 {
        i32::try_from(line.chars().filter(|character| *character == brace).count())
            .unwrap_or(i32::MAX)
    }

    pub fn append(
        entries: &mut String,
        line: (i32, &str),
        origin: (&str, &str),
        context: (&ModelTable, &[String]),
    ) -> Result<(), Failure> {
        (line.0 == 0)
            .then_some(line.1)
            .map(|selected| Self::append_member(entries, selected, origin, context))
            .transpose()
            .map(|_| ())
    }

    pub fn append_member(
        entries: &mut String,
        line: &str,
        origin: (&str, &str),
        context: (&ModelTable, &[String]),
    ) -> Result<(), Failure> {
        Self::member(line)
            .map(|member| Self::append_result(entries, &member, (origin, line), context))
            .transpose()
            .map(|_| ())
    }

    pub fn append_result(
        entries: &mut String,
        member: &Member,
        origin: ((&str, &str), &str),
        context: (&ModelTable, &[String]),
    ) -> Result<(), Failure> {
        let kind = member.declared_type.as_deref().unwrap_or("");
        let model = crate::csharp_task_result::CsharpTaskResult::model(kind, context.0)
            .map_err(|message| Self::failure(origin.0, origin.1, message))?;
        if model.is_some() && !context.1.contains(&member.name) {
            entries.push_str(&Self::entry(member));
        }
        Ok(())
    }

    pub fn member(line: &str) -> Option<Member> {
        let line = line.trim();
        line.strip_prefix("public ")?;
        let (signature, _) = without_type_first_modifiers(line);
        let open = signature.find('<')?;
        let (kind, rest) = CsharpModelType::declared(signature, open)?;
        Self::task_kind(&kind)?;
        let name = UnicodeName::leading(rest);
        let parameters = csharp_parameters(rest[name.len()..].trim_start())?;
        Some(Member {
            name,
            declared_type: Some(kind),
            method: true,
            mutable: false,
            parameters: Some(parameters),
            asynchronous: true,
        })
    }

    pub fn task_kind(kind: &str) -> Option<()> {
        let base = kind.split('<').next()?;
        [
            "Task",
            "System.Threading.Tasks.Task",
            "ValueTask",
            "System.Threading.Tasks.ValueTask",
        ]
        .contains(&base)
        .then_some(())
    }

    pub fn entry(member: &Member) -> String {
        let key = lower_first(&member.name);
        let arguments = csharp_arguments(member.parameters.as_deref().unwrap_or_default());
        format!(
            "        [\"{key}\"] = TacMember.TaskMethod(async arguments => TacModels.Plain(await Instance.{}({arguments}))),\n",
            member.name
        )
    }

    pub fn failure(origin: (&str, &str), line: &str, message: String) -> Failure {
        let at = origin.0.find(line).unwrap_or(0);
        Failure::one(diagnostic(
            1405,
            message,
            None,
            source_span(origin.1, at, at + line.len()),
        ))
    }
}
