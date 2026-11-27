#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CppToken {
    pub text: String,
    pub start: usize,
}

impl CppToken {
    pub fn end(&self) -> usize {
        self.start + self.text.len()
    }

    pub fn is(&self, text: &str) -> bool {
        self.text == text
    }

    pub fn is_word(&self) -> bool {
        self.text.starts_with(|character: char| {
            [character.is_ascii_alphabetic(), character == '_'].contains(&true)
        })
    }

    pub fn touches(&self, next: &Self) -> bool {
        self.end() == next.start
    }
}
