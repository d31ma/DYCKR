pub const NAMES: [&str; 6] = [
        "Service", "Lifetime", "NeedsAll", "Delegate", "Store", "Client",
    ];


pub fn marker(extension: &str, line: &str, start: usize) -> Option<Marker> {
        let text = crate::service_roles::marker_text(extension, line)?;
        for name in crate::service_roles::NAMES {
            let Some(rest) = crate::service_roles::role_rest(extension, text, name) else {
                continue;
            };
            let arguments = if rest.is_empty() {
                ""
            } else {
                rest.strip_prefix('(')?.strip_suffix(')')?
            };
            return Some(Marker {
                name: name.to_owned(),
                arguments: arguments.to_owned(),
                span: start..start + line.len(),
                target: start + line.len(),
                parameter: false,
            });
        }
        None
    }

pub fn role_rest<'a>(extension: &str, text: &'a str, name: &str) -> Option<&'a str> {
        if extension != "py" {
            return text.strip_prefix(name);
        }
        let written = text.split('(').next().unwrap_or(text);
        let normalized = written.replace('_', "");
        normalized
            .eq_ignore_ascii_case(name)
            .then(|| &text[written.len()..])
    }

pub fn marker_text<'a>(extension: &str, line: &'a str) -> Option<&'a str> {
        match extension {
            "rs" | "rb" => line.strip_prefix("#[")?.strip_suffix(']'),
            "cs" => line.strip_prefix('[')?.strip_suffix(']'),
            _ => line.strip_prefix('@'),
        }
    }
use crate::behavior_input::Input;
use crate::behavior_marker::Marker;


    pub fn required_names(input: &Input<'_>, marker: &Marker, no_arguments: &dyn Fn(&str) -> String) -> Result<Vec<String>, crate::failure::Failure> {
        if !marker.arguments.trim().is_empty() {
            return Err(input.failure(
                marker.span.start,
                no_arguments("NeedsAll"),
            ));
        }
        Ok(vec![String::from("*")])
    }

use std::path::Path;

pub struct ServiceRoles {
    pub source: String,
    pub prototype: bool,
    pub application: Option<usize>,
    pub required: Vec<String>,
    pub explicit_prototype: Option<usize>,
}
impl ServiceRoles {

    pub fn declared(source: &str, prototype: bool, required: Vec<String>) -> ServiceRoles {
        ServiceRoles {
            source: source.to_owned(),
            prototype,
            application: None,
            required,
            explicit_prototype: None,
        }
    }

    pub fn require_supported_peer_lifetime(&self, path: &Path) -> Result<(), crate::failure::Failure> {
        if let Some(at) = self.explicit_prototype {
            return Err(crate::failure::Failure::one(crate::failure::diagnostic(
                1406,
                "Explicit prototype lifetime on a native Rust dependency is not supported yet.",
                Some(String::from(
                    "The current typed native graph shares Arc providers within one owner. Prototype overrides need separate provider instances and lifecycle registration; remove the override only if owner-local sharing is the intended contract.",
                )),
                crate::failure::source_span(&path.to_string_lossy(), at, at + "Lifetime".len()),
            )));
        }
        Ok(())
    }
}
