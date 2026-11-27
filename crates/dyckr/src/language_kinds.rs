pub struct LanguageKinds {
    pub text: &'static str,
    pub number: &'static str,
    pub flag: &'static str,
}

impl LanguageKinds {
    pub const fn new(text: &'static str, number: &'static str, flag: &'static str) -> Self {
        Self { text, number, flag }
    }
}
