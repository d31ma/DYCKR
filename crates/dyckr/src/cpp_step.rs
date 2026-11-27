use crate::cpp_tokens::CppTokens;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CppStep {
    Separator,
    Access(bool),
    Member,
}

impl CppStep {
    const ACCESS: [&'static str; 3] = ["public", "private", "protected"];

    pub fn at(tokens: &CppTokens, at: usize) -> Self {
        let separator = tokens.is(at, ";");
        let access = [
            Self::ACCESS.contains(&tokens.text(at)),
            tokens.is(at + 1, ":"),
        ] == [true, true];
        Self::classify(separator, access, tokens.is(at, "public"))
    }

    fn classify(separator: bool, access: bool, public: bool) -> Self {
        match (separator, access) {
            (true, _) => Self::Separator,
            (false, true) => Self::Access(public),
            (false, false) => Self::Member,
        }
    }
}
