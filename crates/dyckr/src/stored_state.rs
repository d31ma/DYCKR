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
use crate::behavior_marker::Marker;

impl StoredState {

    pub fn mentioned(source: &str) -> bool {
        crate::stored_state::StoredState::SPELLINGS
            .iter()
            .any(|spelling| source.contains(spelling))
    }

    pub fn area(marker: &Marker) -> String {
        if marker.name == crate::stored_state::StoredState::LOCAL {
            String::from("local")
        } else {
            String::from("session")
        }
    }

    pub fn visible_name(language: &str, name: &str) -> String {
        match language {
            "cs" | "go" => crate::lower_first::lower_first(name),
            "rs" | "rb" | "py" => crate::companion_names::camel_case(name),
            _ => name.to_owned(),
        }
    }
}
