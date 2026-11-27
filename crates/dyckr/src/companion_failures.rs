
pub fn exposes_nothing(source_path: &str, name: &str) -> crate::failure::Failure {
    crate::failure::Failure::one(crate::failure::diagnostic(
        1406,
        format!("Native service type '{name}' exposes no members."),
        Some(String::from(
            "Give the type a stored property or a method, or remove the service. A native \
             service reaches the page through its members; one with none has nothing to bind.",
        )),
        crate::failure::source_span(source_path, 0, source_path.len()),
    ))
}

pub fn unnamed_service(source_path: &str, wanted: Option<&str>, keyword: &str) -> crate::failure::Failure {
    let Some(wanted) = wanted else {
        return missing_type("Companion", source_path, "native");
    };
    crate::failure::Failure::one(crate::failure::diagnostic(
        1406,
        format!("Native service '{source_path}' declares no {keyword} named '{wanted}'."),
        Some(format!(
            "A service file owns the type named after it (ADR 0030). Name the {keyword} \
             '{wanted}', and keep helpers, entities and configuration types beside it."
        )),
        crate::failure::source_span(source_path, 0, source_path.len()),
    ))
}

pub fn missing_type(name: &str, source_path: &str, language: &str) -> crate::failure::Failure {
    crate::failure::Failure::one(crate::failure::diagnostic(
        1405,
        format!("Companion field '{name}' needs a declared type."),
        Some(format!(
            "Write it as a typed declaration, so the generated setter knows what \
             to convert an incoming value into. A read-only field needs no type. \
             This applies to a class-shaped {language} companion.",
        )),
        crate::failure::source_span(source_path, 0, source_path.len()),
    ))
}
