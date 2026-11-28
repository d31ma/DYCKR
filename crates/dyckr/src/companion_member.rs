use crate::failure::Failure;
use crate::unicode_name::UnicodeName;
use crate::companion_language::CompanionLanguage;
use crate::modifiers::Modifiers;
use crate::companion_syntax::*;
use crate::companion_failures::missing_type;
use crate::rust_model_type::RustModelType;
use crate::companion_converters::kotlin_value;
use crate::lower_first::lower_first;

pub struct Member {
    pub name: String,
    pub declared_type: Option<String>,
    pub method: bool,
    pub mutable: bool,
    pub parameters: Option<Vec<String>>,
    pub asynchronous: bool,
}


impl Member {
    pub fn is_runtime_resource(annotation: &str) -> bool {
        let declared = annotated_type(annotation).unwrap_or_default();
        let initializer = annotation.split_once('=').map_or("", |(_, value)| value);
        let constructed = initializer.split('(').next().unwrap_or_default();
        [declared.trim_end_matches(['!', '?']), constructed.trim()]
            .iter()
            .any(|name| Self::is_resource_type(name))
    }

    pub fn is_resource_type(name: &str) -> bool {
        name == "BehaviorContext" || crate::web_api_clients::WebApiClients::is_client_type(name)
    }

    pub const BEHAVIOR_JSON: &'static str = "BehaviorJSON";
    pub const SWIFT_KINDS: [&'static str; 2] = ["method", "task"];

    pub fn method_effects(signature: &str) -> &str {
        let after = closing_parenthesis(signature).map_or("", |close| &signature[close + 1..]);
        after.split('{').next().unwrap_or_default()
    }

    pub fn require_name(name: &str) -> Option<()> {
        (!name.is_empty()).then_some(())
    }

    pub fn method_returned_type(signature: &str, language: &str) -> Option<String> {
        let effects = Self::method_effects(signature);
        Self::returned_type(effects, language)
            .or_else(|| Self::block_returned_type(signature, language))
    }

    pub fn block_returned_type(signature: &str, language: &str) -> Option<String> {
        let close = closing_parenthesis(signature)?;
        let after = signature[close + 1..].trim_start();
        Self::block_unit(after, language)
    }

    pub fn block_unit(after: &str, language: &str) -> Option<String> {
        match (language, after.as_bytes().first()) {
            ("tac.kt", Some(b'{')) => Some(String::from("Unit")),
            _ => None,
        }
    }

    pub fn returned_type(effects: &str, language: &str) -> Option<String> {
        match language {
            "tac.kt" => Self::kotlin_returned_type(effects),
            _ => Self::swift_returned_type(effects),
        }
    }

    pub fn swift_returned_type(effects: &str) -> Option<String> {
        let (_, returned) = effects.split_once("->")?;
        Some(returned.trim().to_owned())
    }

    pub fn kotlin_returned_type(effects: &str) -> Option<String> {
        let returned = effects.trim_start().strip_prefix(':')?;
        let declared = returned.split('=').next().unwrap_or_default().trim();
        (!declared.is_empty()).then(|| declared.to_owned())
    }

    pub fn property_type(annotation: &str, language: &str) -> Option<String> {
        match language {
            "tac.kt" => Self::kotlin_property_type(annotation),
            _ => annotated_type(annotation),
        }
    }

    pub fn kotlin_property_type(annotation: &str) -> Option<String> {
        let declared = annotated_type(annotation)?;
        let end = Self::getter_start(&declared).unwrap_or(declared.len());
        Some(declared[..end].trim().to_owned())
    }

    pub fn getter_start(declared: &str) -> Option<usize> {
        declared
            .match_indices("get")
            .find_map(|(at, _)| Self::getter_word(declared, at).then_some(at))
    }

    pub fn getter_word(declared: &str, at: usize) -> bool {
        let before = declared.as_bytes()[..at]
            .last()
            .is_some_and(u8::is_ascii_whitespace);
        let call = declared[at + 3..].trim_start().starts_with('(');
        before && call
    }

    pub fn is_asynchronous(effects: &str) -> bool {
        effects.split_whitespace().any(|word| word == "async")
    }

    pub fn swift_behavior_entry(&self, parameters: &[String]) -> String {
        let name = &self.name;
        let call = format!("tacInstance.{name}({})", swift_arguments(parameters));
        let answer = self.swift_answer(&self.swift_finished(call));
        let kind = Self::SWIFT_KINDS[usize::from(self.asynchronous)];
        format!("    \"{name}\": .{kind}({{ tacArguments in {answer} }}),\n")
    }

    pub fn is_swift_behavior_member(&self) -> bool {
        self.asynchronous || self.returns_behavior_json()
    }

    pub fn returns_behavior_json(&self) -> bool {
        self.declared_type.as_deref() == Some(Self::BEHAVIOR_JSON)
    }

    pub fn swift_finished(&self, call: String) -> String {
        if self.asynchronous {
            return format!("await {call}");
        }
        call
    }

    pub fn swift_answer(&self, call: &str) -> String {
        if self.returns_behavior_json() {
            return format!("tacParseJSON(try {call}.encoded())");
        }
        format!("try {call}")
    }

    pub fn swift_argument(kind: &str, index: usize) -> String {
        if kind == Self::BEHAVIOR_JSON {
            return format!("BehaviorJSON.parse(tacArgumentText(tacArguments, {index}))");
        }
        format!("tacArgument(tacArguments, {index})")
    }
}















pub fn members(
    language: CompanionLanguage,
    body: &str,
    source_path: &str,
) -> Result<Vec<Member>, Failure> {
    let mut members = Vec::new();
    let mut depth = 0i32;
    for line in body.lines() {
        let trimmed = line.trim();
        let entering = depth;
        let opened = i32::try_from(trimmed.matches('{').count()).unwrap_or(0);
        let closed = i32::try_from(trimmed.matches('}').count()).unwrap_or(0);
        depth += opened - closed;
        if entering != 0 || trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        if let Some(member) = declaration(language, trimmed, source_path)? {
            members.push(member);
        }
    }
    members.sort_by(|left, right| left.name.cmp(&right.name));
    members.dedup_by(|left, right| left.name == right.name);
    Ok(members)
}

pub fn rust_struct_fields(body: &str) -> Vec<Member> {
    let mut fields = Vec::new();
    for line in body.lines() {
        for declaration in rust_field_declarations(line) {
            if let Some(field) = rust_field(declaration.trim()) {
                fields.push(field);
            }
        }
    }
    fields
}


pub fn rust_field(line: &str) -> Option<Member> {
    let line = line
        .trim_start_matches("pub ")
        .trim_start_matches("pub(crate) ");
    if line.starts_with("//") || line.starts_with('#') {
        return None;
    }
    let name = UnicodeName::leading(line);
    if name.is_empty() {
        return None;
    }
    let rest = line[name.len()..].trim_start();
    let declared = rest.strip_prefix(':')?.trim().trim_end_matches(',').trim();
    let declared = if matches!(declared, "&'static str" | "&str") {
        "&str"
    } else if declared.is_empty()
        || declared.contains(['&', '\''])
        || (declared.contains('<') && !RustModelType::is_model(declared))
        || RustModelType::is_runtime(declared)
        || crate::web_api_clients::WebApiClients::is_client_type(declared)
        || !RustModelType::crosses(declared)
    {
        return None;
    } else {
        declared
    };
    Some(Member {
        name,
        declared_type: Some(String::from(declared)),
        method: false,
        mutable: true,
        parameters: Some(Vec::new()),
        asynchronous: false,
    })
}

pub fn rust_methods(body: &str) -> Vec<(Member, Vec<String>)> {
    let mut methods = Vec::new();
    for line in body.lines() {
        if let Some(method) = rust_method(line) {
            methods.push(method);
        }
    }
    methods
}

pub fn rust_method(line: &str) -> Option<(Member, Vec<String>)> {
    let line = line.trim().strip_prefix("pub ")?;
    let asynchronous = line.starts_with("async ");
    let line = line.trim_start_matches("async ").trim_start();
    let rest = line.strip_prefix("fn").and_then(after_keyword)?;
    let name = UnicodeName::leading(rest);
    let arguments = rest[name.len()..].trim_start();
    let takes_self = arguments.starts_with("(&self") || arguments.starts_with("(&mut self");
    if name.is_empty() || !takes_self {
        return None;
    }
    let returns = rust_return_type(arguments);
    let crosses = |kind: &str| kind == "()" || RustModelType::crosses(kind);
    let task = [asynchronous, crate::rust_task_member::RustTaskMember::fallible(returns.as_deref())].contains(&true);
    let shown = if task {
        crate::rust_task_member::RustTaskMember::crosses(returns.as_deref())
    } else {
        returns.as_deref().is_none_or(crosses)
    };
    if !shown {
        return None;
    }
    let parameters = rust_parameters(arguments)?;
    let member = Member {
        name,
        declared_type: returns,
        method: true,
        mutable: false,
        parameters: Some(Vec::new()),
        asynchronous,
    };
    Some((member, parameters))
}

pub fn rust_return_type(arguments: &str) -> Option<String> {
    let returned = arguments.split("->").nth(1)?;
    let returned = returned.split('{').next().unwrap_or_default().trim();
    if returned.is_empty() {
        return None;
    }
    Some(returned.to_owned())
}

pub fn rust_parameters(arguments: &str) -> Option<Vec<String>> {
    let close = closing_parenthesis(arguments).unwrap_or(arguments.len());
    let inside = &arguments[1..close];
    let mut kinds = Vec::new();
    for parameter in inside.split(',').skip(1) {
        let parameter = parameter.trim();
        if parameter.is_empty() {
            continue;
        }
        let kind = parameter.split_once(':')?.1.trim();
        if !RustModelType::crosses(kind) {
            return None;
        }
        kinds.push(kind.to_owned());
    }
    Some(kinds)
}








pub fn declaration(
    language: CompanionLanguage,
    line: &str,
    source_path: &str,
) -> Result<Option<Member>, Failure> {
    match language {
        CompanionLanguage::Swift => {
            name_first(line, source_path, "var", "let", "func", "tac.swift")
        }
        CompanionLanguage::Kotlin => crate::kotlin_member_syntax::public_declaration(strip_attribute(line))
            .map_or(Ok(None), |line| {
                let (asynchronous, line) = crate::kotlin_member_syntax::suspended(line);
                let found = name_first(line, source_path, "var", "val", "fun", "tac.kt")?;
                Ok(found.map(|member| Member {
                    asynchronous,
                    ..member
                }))
            }),
        CompanionLanguage::CSharp | CompanionLanguage::CSharpBrowser => Ok(type_first(line)),
        CompanionLanguage::Rust => Ok(None),
    }
}

pub fn name_first(
    line: &str,
    source_path: &str,
    mutable: &str,
    immutable: &str,
    function: &str,
    language: &str,
) -> Result<Option<Member>, Failure> {
    let line = strip_attribute(line);
    if line.starts_with("private ") || line.starts_with("fileprivate ") {
        return Ok(None);
    }
    let line = line.trim_start_matches("public ");
    if let Some(rest) = line.strip_prefix(function).and_then(after_keyword) {
        return Ok(name_first_method(rest, language));
    }
    for (keyword, writable) in [(mutable, true), (immutable, false)] {
        let Some(rest) = line.strip_prefix(keyword).and_then(after_keyword) else {
            continue;
        };
        let name = UnicodeName::leading(rest);
        if name.is_empty() {
            continue;
        }
        let annotation = rest[name.len()..].trim_start();
        if Member::is_runtime_resource(annotation) {
            return Ok(None);
        }
        let declared_type = Member::property_type(annotation, language);
        if writable && declared_type.is_none() {
            return Err(missing_type(&name, source_path, language));
        }
        return Ok(Some(Member {
            name,
            declared_type,
            method: false,
            mutable: writable,
            parameters: Some(Vec::new()),
            asynchronous: false,
        }));
    }
    Ok(None)
}

pub fn name_first_method(rest: &str, language: &str) -> Option<Member> {
    let name = UnicodeName::leading(rest);
    Member::require_name(&name)?;
    let signature = rest[name.len()..].trim_start();
    let parameters = signature
        .strip_prefix('(')
        .map_or_else(Vec::new, typed_parameters);
    let effects = Member::method_effects(signature);
    Some(Member {
        name,
        declared_type: Member::method_returned_type(signature, language),
        method: true,
        mutable: false,
        parameters: Some(parameters),
        asynchronous: Member::is_asynchronous(effects),
    })
}

pub fn typed_parameters(list: &str) -> Vec<String> {
    let mut types = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for character in list.chars() {
        match character {
            '(' | '<' | '[' => depth += 1,
            ')' | '>' | ']' if depth == 0 => break,
            ')' | '>' | ']' => depth -= 1,
            ',' if depth == 0 => {
                types.extend(typed_parameter(&current));
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(character);
    }
    types.extend(typed_parameter(&current));
    types
}

pub fn typed_parameter(parameter: &str) -> Option<String> {
    let (names, declared) = crate::parameter_text::ParameterText::without_annotations(parameter).split_once(':')?;
    let kind = declared.split('=').next().unwrap_or_default().trim();
    (!kind.is_empty()).then(|| format!("{}: {kind}", names.trim()))
}




pub fn type_first(line: &str) -> Option<Member> {
    let line = line.trim();
    if !Modifiers::CSHARP_READABLE
        .iter()
        .any(|access| line.starts_with(access))
    {
        return None;
    }
    let (line, fixed) = without_type_first_modifiers(line);
    if line.starts_with("//") || line.starts_with('@') {
        return None;
    }
    let (declared_type, rest) = csharp_declared_type(line)?;
    let name = UnicodeName::leading(rest);
    if name.is_empty() {
        return None;
    }
    let after = rest[name.len()..].trim_start();
    let method = after.starts_with('(');
    let hidden_type = crate::csharp_model_type::CsharpModelType::is_runtime(&declared_type)
        || crate::web_api_clients::WebApiClients::is_client_type(&declared_type);
    if !method && hidden_type {
        return None;
    }
    let getter = is_getter_only(after);
    let parameters = if method {
        csharp_parameters(after)
    } else {
        Some(Vec::new())
    };
    Some(Member {
        name,
        declared_type: Some(declared_type),
        method,
        mutable: !method && !fixed && !getter,
        parameters,
        asynchronous: false,
    })
}

pub fn csharp_declared_type(line: &str) -> Option<(String, &str)> {
    for (written, declared) in Modifiers::CSHARP_TASKS {
        if let Some(rest) = line.strip_prefix(written).and_then(after_keyword) {
            return Some((String::from(declared), rest));
        }
    }
    let name = UnicodeName::leading(line);
    if name.is_empty() {
        return None;
    }
    let (declared_type, rest) = crate::csharp_model_type::CsharpModelType::declared(line, name.len())?;
    if rest.starts_with('<') || rest.starts_with('?') || crate::csharp_model_type::CsharpModelType::is_task(&declared_type) {
        return None;
    }
    Some((declared_type, rest))
}

pub fn without_type_first_modifiers(line: &str) -> (&str, bool) {
    let mut line = line;
    let mut fixed = false;
    'strip: loop {
        for (modifier, immutable) in Modifiers::TYPE_FIRST {
            if let Some(rest) = line.strip_prefix(modifier) {
                fixed |= immutable;
                line = rest.trim_start();
                continue 'strip;
            }
        }
        return (line, fixed);
    }
}

pub fn is_getter_only(after_name: &str) -> bool {
    if after_name.starts_with("=>") {
        return true;
    }
    if !after_name.starts_with('{') {
        return false;
    }
    let accessors = match after_name.split_once('}') {
        Some((accessors, _)) => accessors,
        None => after_name,
    };
    let mut words = accessors.split(|character: char| !character.is_alphanumeric());
    !words.any(|word| word == "set" || word == "init")
}

pub fn strip_attribute(line: &str) -> &str {
    let mut rest = line.trim_start();
    while let Some(after) = rest.strip_prefix('@') {
        let name_length = UnicodeName::leading(after).len();
        if name_length == 0 {
            return rest;
        }
        let tail = &after[name_length..];
        rest = match tail.strip_prefix('(') {
            Some(arguments) => after_closing_parenthesis(arguments),
            None => tail,
        };
        rest = rest.trim_start();
    }
    rest
}

pub fn after_closing_parenthesis(arguments: &str) -> &str {
    let mut depth = 1i32;
    for (index, character) in arguments.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return &arguments[index + 1..];
                }
            }
            _ => {}
        }
    }
    arguments
}



pub fn entry(language: CompanionLanguage, member: &Member) -> String {
    match language {
        CompanionLanguage::Swift => swift_entry(member),
        CompanionLanguage::Kotlin => kotlin_entry(member),
        CompanionLanguage::CSharp | CompanionLanguage::CSharpBrowser => {
            crate::csharp_browser_members::CsharpBrowserMembers::entry(member)
        }
        CompanionLanguage::Rust => String::new(),
    }
}

pub fn swift_entry(member: &Member) -> String {
    let name = &member.name;
    let parameters = member.parameters.as_deref().unwrap_or_default();
    if member.is_swift_behavior_member() {
        return member.swift_behavior_entry(parameters);
    }
    if member.method && parameters.is_empty() {
        return format!("    \"{name}\": .method({{ _ in try tacInstance.{name}() }}),\n");
    }
    if member.method {
        let arguments = swift_arguments(parameters);
        return format!(
            "    \"{name}\": .method({{ tacArguments in try tacInstance.{name}({arguments}) }}),\n"
        );
    }
    match (&member.declared_type, member.mutable) {
        (Some(kind), true) => format!(
            "    \"{name}\": .field({{ tacInstance.{name} }}, \
             {{ tacInstance.{name} = tacField({kind}.self, $0) ?? tacInstance.{name} }}),\n"
        ),
        _ => format!("    \"{name}\": .field({{ tacInstance.{name} }}),\n"),
    }
}

pub fn swift_arguments(parameters: &[String]) -> String {
    parameters
        .iter()
        .enumerate()
        .map(|(index, declaration)| {
            format!(
                "{}{}",
                swift_label(declaration),
                Member::swift_argument(parameter_type(declaration), index)
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn kotlin_entry(member: &Member) -> String {
    let name = &member.name;
    if member.method {
        let arguments = member
            .parameters
            .iter()
            .flatten()
            .enumerate()
            .map(|(index, declaration)| {
                kotlin_value(
                    parameter_type(declaration),
                    &format!("it.getOrNull({index})"),
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        if member.asynchronous {
            return format!(
                "    \"{name}\" to TacTask({{ tacThrownCode(it) }}) {{ tacInstance.{name}({arguments}) }},\n"
            );
        }
        return format!("    \"{name}\" to TacMethod {{ tacInstance.{name}({arguments}) }},\n");
    }
    match (&member.declared_type, member.mutable) {
        (Some(kind), true) => {
            let converter = kotlin_value(kind, "it");
            format!(
                "    \"{name}\" to TacField({{ tacInstance.{name} }}, \
                 {{ tacInstance.{name} = {converter} }}),\n"
            )
        }
        _ => format!("    \"{name}\" to TacField({{ tacInstance.{name} }}),\n"),
    }
}

pub fn csharp_entry(member: &Member) -> String {
    let name = &member.name;
    let key = lower_first(name);
    let Some(parameters) = &member.parameters else {
        return String::new();
    };
    let passed = csharp_arguments(parameters);
    match (member.method, &member.declared_type, member.mutable) {
        (true, Some(kind), _) if kind == "void" => format!(
            "        [\"{key}\"] = TacMember.Method(arguments => {{ Instance.{name}({passed}); return null; }}),\n"
        ),
        (true, Some(kind), _) if kind == "BehaviorJSON" => format!(
            "        [\"{key}\"] = TacMember.Method(arguments => Tac.PlainJson(Instance.{name}({passed}))),\n"
        ),
        (true, Some(kind), _) if kind == "object" => format!(
            "        [\"{key}\"] = TacMember.Method(arguments => TacMember.Result(() => Instance.{name}({passed}))),\n"
        ),
        (true, kind, _) => {
            let call =
                crate::csharp_model_type::CsharpModelType::value(kind.as_deref(), &format!("Instance.{name}({passed})"));
            format!("        [\"{key}\"] = TacMember.Method(arguments => {call}),\n")
        }
        (false, kind, mutable) => {
            let read = crate::csharp_model_type::CsharpModelType::value(kind.as_deref(), &format!("Instance.{name}"));
            let write = kind
                .as_deref()
                .filter(|_| mutable)
                .and_then(|kind| crate::csharp_model_type::CsharpModelType::reading(kind, "value"))
                .map(|converter| format!(", value => Instance.{name} = {converter}"))
                .unwrap_or_default();
            format!("        [\"{key}\"] = TacMember.Field(() => {read}{write}),\n")
        }
    }
}






pub fn csharp_parameters(after_name: &str) -> Option<Vec<String>> {
    let mut kinds = Vec::new();
    for parameter in crate::parameter_text::ParameterText::csharp_parameters(after_name)? {
        let kind = parameter.split_whitespace().next()?;
        let readable_model =
            crate::csharp_model_type::CsharpModelType::is_model(kind) && crate::csharp_model_type::CsharpModelType::read(kind, kind).is_some();
        let supported = readable_model
            || matches!(
                kind,
                "string" | "int" | "long" | "double" | "float" | "bool" | "object" | "BehaviorJSON"
            );
        if !supported {
            return None;
        }
        kinds.push(kind.to_owned());
    }
    Some(kinds)
}

pub fn csharp_arguments(parameters: &[String]) -> String {
    let mut arguments = Vec::new();
    for (index, kind) in parameters.iter().enumerate() {
        let value = format!("(arguments.Length > {index} ? arguments[{index}] : null)");
        let argument = match kind.as_str() {
            "object" => value,
            "float" => format!("Convert.ToSingle({value})"),
            "BehaviorJSON" => format!("BehaviorJSON.Parse(Tac.EncodeValue({value}))"),
            other => crate::csharp_model_type::CsharpModelType::reading(other, &value).unwrap_or(value),
        };
        arguments.push(argument);
    }
    arguments.join(", ")
}
