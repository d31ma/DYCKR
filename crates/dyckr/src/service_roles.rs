pub const NAMES: [&str; 6] = [
        "Service", "Lifetime", "NeedsAll", "Delegate", "Store", "Client",
    ];
use crate::behavior_marker::Marker;


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
