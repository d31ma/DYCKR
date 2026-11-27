use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Realm {
    Tac,
    Yon,
}

impl Realm {
    pub fn of(path: &Path) -> Option<Self> {
        for part in path.components() {
            match part.as_os_str().to_str() {
                Some("client") => return Some(Self::Tac),
                Some("server") => return Some(Self::Yon),
                _ => {}
            }
        }
        None
    }

    pub fn named(environment: &str) -> Option<Self> {
        match environment {
            "tac" => Some(Self::Tac),
            "yon" => Some(Self::Yon),
            _ => None,
        }
    }

    pub const fn word(self) -> &'static str {
        match self {
            Self::Tac => "Tac",
            Self::Yon => "Yon",
        }
    }

    pub fn languages(self) -> &'static [&'static str] {
        match self {
            Self::Tac => TAC,
            Self::Yon => YON,
        }
    }
}

    pub const LANGUAGES: [&'static str; 16] = [
        "js", "ts", "py", "php", "java", "groovy", "kt", "cs", "rs", "dart", "go", "swift", "rb",
        "lua", "fs", "hpp",
    ];
    pub const YON: &'static [&'static str] = &[
        "js", "ts", "py", "php", "java", "groovy", "kt", "cs", "fs", "rs", "dart", "go", "rb",
        "lua", "hpp",
    ];
    pub const TAC: &'static [&'static str] = &[
        "js", "ts", "rs", "cs", "dart", "go", "swift", "kt", "rb", "lua", "py", "java", "fs", "hpp",
    ];
