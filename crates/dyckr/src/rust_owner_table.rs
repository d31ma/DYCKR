use crate::companion_failures::exposes_nothing;
use crate::companion_member::Member;
use crate::companion_member::rust_methods;
use crate::companion_syntax::blocks;
use crate::failure::Failure;
use std::collections::HashMap;
use std::fmt::Write as _;

pub fn rust_owner_table(
    source: &str,
    source_path: &str,
    excluded: &[String],
    extra: &str,
    needs_all: bool,
) -> Result<String, Failure> {
    let wanted = crate::declared_service::declared_service(source_path, source);
    let Some((name, body)) =
        crate::companion_syntax::block_named(source, "struct", wanted.as_deref())
    else {
        return Err(crate::companion_failures::unnamed_service(
            source_path,
            wanted.as_deref(),
            "struct",
        ));
    };
    let mut members = crate::companion_member::rust_struct_fields(body);

    let collaborators =
        crate::rust_collaborators::RustCollaborators::declared_beside(&name, source);
    members.retain(|member| !collaborators.holds(member.declared_type.as_deref()));
    let mut parameters = HashMap::new();
    for (header, impl_body) in blocks(source, "impl") {
        if header.trim() != name {
            continue;
        }
        for (member, declared) in rust_methods(impl_body) {
            parameters.insert(member.name.clone(), declared);
            members.push(member);
        }
    }
    let instance = crate::rust_construction::RustConstruction::instance(
        source,
        source_path,
        &name,
        needs_all,
    )?;
    let mut output = format!(
        "pub fn tac_create() -> Vec<(&'static str, TacMember)> {{\nlet instance = std::rc::Rc::new(core::cell::RefCell::new({instance}));\nvec![\n(\"__tachyon_busy\", TacMember::Busy({{let it = instance.clone(); Box::new(move || it.try_borrow_mut().is_err())}})),\n"
    );
    members.sort_by(|left, right| left.name.cmp(&right.name));
    members.dedup_by(|left, right| left.name == right.name);
    write_rust_members(&mut output, &members, excluded, &parameters);
    if extra.trim().is_empty() && members.iter().all(|member| excluded.contains(&member.name)) {
        return Err(exposes_nothing(source_path, &name));
    }
    output.push_str(extra);
    output.push_str("]\n}\n");
    Ok(output)
}

pub fn write_rust_members(
    output: &mut String,
    members: &[Member],
    excluded: &[String],
    parameters: &HashMap<String, Vec<String>>,
) {
    for member in members {
        if excluded.contains(&member.name) {
            continue;
        }
        let returns = member.declared_type.as_deref();
        if [
            member.asynchronous,
            crate::rust_task_member::RustTaskMember::fallible(returns),
        ]
        .contains(&true)
        {
            let declared = parameters.get(&member.name).map_or(&[][..], Vec::as_slice);
            let entry = crate::rust_task_member::RustTaskMember::entry(
                (
                    &crate::companion_names::camel_case(&member.name),
                    &member.name,
                ),
                returns,
                declared,
                member.asynchronous,
            );
            let _ = writeln!(output, "{entry}");
        } else if member.method {
            let declared = parameters.get(&member.name).map_or(&[][..], Vec::as_slice);
            write_rust_method(output, member, declared);
        } else {
            write_rust_field(output, member);
        }
    }
}

pub fn write_rust_method(output: &mut String, member: &Member, parameters: &[String]) {
    let name = &member.name;
    let key = crate::companion_names::camel_case(name);
    let kind = member.declared_type.as_deref().unwrap_or("()");
    let (bindings, passed) = crate::rust_arguments::rust_arguments(parameters);
    let value =
        crate::rust_model_type::rust_read(kind, &format!("it.borrow_mut().{name}({passed})"));
    let _ = writeln!(
        output,
        "(\"{key}\", TacMember::Method({{let it = instance.clone(); Box::new(move |arguments: Vec<TacValue>| {{ #[allow(unused_mut, unused_variables)] let mut arguments = arguments.into_iter(); {bindings}{value} }})}})),"
    );
}

pub fn write_rust_field(output: &mut String, member: &Member) {
    let name = &member.name;
    let key = crate::companion_names::camel_case(name);
    let kind = member.declared_type.as_deref().unwrap_or("i64");
    let access = if kind == "&str" {
        format!("it.borrow().{name}")
    } else {
        format!("it.borrow().{name}.clone()")
    };
    let value = crate::rust_model_type::rust_read(kind, &access);
    let write = match crate::rust_model_type::rust_write(kind) {
        Some(conversion) => format!(
            "Some({{let it = instance.clone(); Box::new(move |value| it.borrow_mut().{name} = {conversion})}})"
        ),
        None => String::from("None"),
    };
    let _ = writeln!(
        output,
        "(\"{key}\", TacMember::Field {{read: {{let it = instance.clone(); Box::new(move || {value})}}, write: {write}}}),"
    );
}
