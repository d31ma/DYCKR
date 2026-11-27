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

