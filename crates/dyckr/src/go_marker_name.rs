
pub struct GoMarkerName;

impl GoMarkerName {
    const NAMESPACES: [&'static str; 2] = ["yon::", "tac::"];
    const SWAG_TAGS: [&'static str; 2] = ["Description", "ID"];
    const MAX_NAME: usize = 128;

    pub fn read(text: &str) -> &str {
        let length = text
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b':'))
            .count();
        &text[..length]
    }

    pub fn is_marker(text: &str) -> bool {
        let name = Self::read(text);
        let after = &text[name.len()..];
        let swag_text = Self::SWAG_TAGS.contains(&name)
            && after.starts_with([' ', '\t'])
            && !after.trim().is_empty();
        if swag_text || !name.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) {
            return false;
        }
        name.contains("::")
            || (name.as_bytes()[0].is_ascii_uppercase()
                && (Self::known(name) || Self::suggestion(name).is_some()))
    }

    pub fn known(name: &str) -> bool {
        let plain = Self::NAMESPACES
            .iter()
            .find_map(|namespace| name.strip_prefix(namespace))
            .unwrap_or(name);
        if crate::framework_marker::framework_marker_names().any(|marker| marker == plain) {
            return true;
        }
        if crate::framework_marker::framework_marker_names()
            .any(|marker| marker.eq_ignore_ascii_case(plain))
        {
            return false;
        }
        crate::renamed::RenamedAnnotation::message(plain).is_some()
            || crate::renamed::RoutingRename::message(plain).is_some()
            || crate::yon_marker::yon_marker_name(plain)
    }

    pub fn suggestion(name: &str) -> Option<&'static str> {
        let bounded: String = name.chars().take(Self::MAX_NAME).collect();
        let limit = if bounded.len() <= 5 { 1 } else { 2 };
        crate::framework_marker::framework_marker_names()
            .map(|candidate| (marker_distance(&bounded, candidate), candidate))
            .filter(|(distance, _)| *distance <= limit)
            .min_by_key(|(distance, _)| *distance)
            .map(|(_, candidate)| candidate)
    }

    pub fn refusal(name: &str) -> Option<String> {
        if Self::known(name) {
            return None;
        }
        let written: String = name.chars().take(Self::MAX_NAME).collect();
        let (namespace, plain) = match written.rsplit_once("::") {
            Some((namespace, plain)) => (format!("{namespace}::"), plain.to_owned()),
            None => (String::new(), written.clone()),
        };
        let lowered = namespace.to_ascii_lowercase();
        if !namespace.is_empty()
            && namespace != lowered
            && Self::NAMESPACES.contains(&lowered.as_str())
        {
            return Some(format!(
                "unknown annotation `@{written}`; namespaces are lowercase: write `// @{lowered}{plain}`"
            ));
        }
        let help = Self::suggestion(&plain).map_or_else(String::new, |candidate| {
            format!("; did you mean `// @{namespace}{candidate}`?")
        });
        Some(format!("unknown annotation `@{written}`{help}"))
    }
}

fn marker_distance(left: &str, right: &str) -> usize {
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    for (row, left_byte) in left.bytes().enumerate() {
        let mut current = vec![row + 1];
        for (column, right_byte) in right.bytes().enumerate() {
            let replacement = usize::from(!left_byte.eq_ignore_ascii_case(&right_byte));
            current.push(
                (previous[column] + replacement)
                    .min(previous[column + 1] + 1)
                    .min(current[column] + 1),
            );
        }
        previous = current;
    }
    previous[right.len()]
}
