#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompanionLanguage {
    Swift,
    Kotlin,
    CSharp,
    CSharpBrowser,
    Rust,
}

impl CompanionLanguage {
    pub fn for_web(self, web: bool) -> Self {
        self.browser_selection(web)
    }

    fn browser_selection(self, web: bool) -> Self {
        match (self, web) {
            (Self::CSharp, true) => Self::CSharpBrowser,
            _ => self,
        }
    }
}

