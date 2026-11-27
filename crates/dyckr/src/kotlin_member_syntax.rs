use std::collections::BTreeSet;

pub const SUSPEND: &'static str = "suspend ";

    pub fn public_declaration(mut source: &str) -> Option<&str> {
        loop {
            match source.split_whitespace().next()? {
                "private" | "protected" | "internal" => return None,
                modifier @ ("public" | "lateinit") => {
                    source = source[modifier.len()..].trim_start();
                }
                _ => return Some(source),
            }
        }
    }

    pub fn suspended(source: &str) -> (bool, &str) {
        source
            .strip_prefix(SUSPEND)
            .map_or((false, source), |rest| (true, rest.trim_start()))
    }


    pub fn event_members(events: Option<&crate::event_module::EventModule>) -> BTreeSet<String> {
        events
            .into_iter()
            .flat_map(|module| module.methods.iter().map(|method| method.name.clone()))
            .collect()
    }

