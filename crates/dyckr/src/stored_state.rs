pub struct StoredState;

impl StoredState {
    pub const LOCAL: &'static str = "LocalState";
    pub const SESSION: &'static str = "SessionState";
    pub const SPELLINGS: [&'static str; 4] =
        ["LocalState", "SessionState", "local_state", "session_state"];
    pub const SIGIL_LANGUAGES: [&'static str; 3] = ["js", "ts", "dart"];

    pub fn marks(name: &str) -> bool {
        name == Self::LOCAL || name == Self::SESSION
    }

}
