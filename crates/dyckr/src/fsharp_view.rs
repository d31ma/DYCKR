    pub fn with_fsharp_attributes(mut text: String, rules: &[&'static str]) -> String {
        for name in rules {
            let csharp = format!("[{}]", name);
            text = text.replace(&csharp, &format!("[<{}>]", name));
        }
        text
    }

