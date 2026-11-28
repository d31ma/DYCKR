use crate::companion_member::{Member, csharp_arguments, csharp_parameters, without_type_first_modifiers};
use crate::lower_first::lower_first;
use crate::unicode_name::UnicodeName;

pub struct FsharpTaskMembers;

impl FsharpTaskMembers {
    pub const EXISTING: [&'static str; 2] = ["Task", "Task<BehaviorJSON>"];
    pub const NAMESPACE: &'static str = "System.Threading.Tasks.";

    pub fn entry(line: &str) -> Option<(String, String)> {
        let member = crate::csharp_task_members::CsharpTaskMembers::member(line).or_else(|| Self::value_task(line))?;
        let kind = member.declared_type.as_deref().unwrap_or_default();
        let kind = kind.strip_prefix(Self::NAMESPACE).unwrap_or(kind);
        (!Self::EXISTING.contains(&kind))
            .then(|| (member.name.clone(), Self::written(&member, kind)))
    }

    pub fn name(line: &str) -> Option<String> {
        let member = crate::csharp_task_members::CsharpTaskMembers::member(line).or_else(|| Self::value_task(line))?;
        Some(member.name)
    }

    pub fn value_task(line: &str) -> Option<Member> {
        let line = line.trim().strip_prefix("public ")?;
        let (signature, _) = without_type_first_modifiers(line);
        let signature = signature.strip_prefix(Self::NAMESPACE).unwrap_or(signature);
        let rest = signature.strip_prefix("ValueTask ")?.trim_start();
        let name = UnicodeName::leading(rest);
        let parameters = csharp_parameters(rest[name.len()..].trim_start())?;
        Some(Member {
            name,
            declared_type: Some(String::from("ValueTask")),
            method: true,
            mutable: false,
            parameters: Some(parameters),
            asynchronous: true,
        })
    }

    pub fn written(member: &Member, kind: &str) -> String {
        let key = lower_first(&member.name);
        let arguments = csharp_arguments(member.parameters.as_deref().unwrap_or_default());
        let call = format!("Instance.{}({arguments})", member.name);
        let result = Self::result(kind, &call);
        format!("        [\"{key}\"] = TacMember.TaskMethod(async arguments => {{ {result} }}),\n")
    }

    pub fn result(kind: &str, call: &str) -> String {
        match kind {
            "ValueTask" => format!("await {call}; return null;"),
            "ValueTask<BehaviorJSON>" => format!("return Tac.PlainJson(await {call});"),
            _ => format!("return TacModels.Plain(await {call});"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FsharpTaskMembers;

    pub fn entry(line: &str) -> String {
        FsharpTaskMembers::entry(line)
            .map(|(_, entry)| entry)
            .unwrap_or_default()
    }

    #[test]
    pub fn typed_tasks_answer_plain_values() {
        let model = entry("public Task<Greeting> Ask() {");
        assert!(
            model.contains("[\"ask\"] = TacMember.TaskMethod(async arguments => { return TacModels.Plain(await Instance.Ask()); })"),
            "{model}"
        );
        let list = entry("public Task<FSharpList<Greeting>> All() {");
        assert!(
            list.contains("TacModels.Plain(await Instance.All())"),
            "{list}"
        );
        assert!(
            entry("public ValueTask Pause() {").contains("await Instance.Pause(); return null;")
        );
        assert!(
            entry("public ValueTask<BehaviorJSON> Raw() {")
                .contains("Tac.PlainJson(await Instance.Raw())")
        );
    }

    #[test]
    pub fn existing_task_shapes_keep_their_entries() {
        assert!(FsharpTaskMembers::entry("public Task Pause() {").is_none());
        assert!(FsharpTaskMembers::entry("public Task<BehaviorJSON> Read() {").is_none());
    }
}
