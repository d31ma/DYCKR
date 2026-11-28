mod lua_behavior_edits;
mod lua_events;
mod lua_fields;
mod lua_input;
mod lua_lowering;
mod lua_resources;
mod lua_sockets;
mod lua_style;
mod lua_test_unit;
mod lua_view;
mod lua_yon;

#[cfg(any(test, feature = "test-support"))]
mod lua_program;
#[cfg(test)]
mod runtime_tests;
#[cfg(test)]
mod tests;

pub use dyckr::lua_models::LuaModels;
pub use dyckr::lua_shadow::LuaShadow;
#[cfg(test)]
use dyckr::lua_types::LuaTypes;
pub use lua_behavior_edits::LuaBehaviorEdits;
use lua_events::LuaEvents;
pub use lua_fields::LuaFields;
use lua_input::LuaInput;
use lua_lowering::LuaLowering;
#[cfg(any(test, feature = "test-support"))]
pub use lua_program::LuaProgram;
use lua_resources::LuaResources;
use lua_sockets::LuaSockets;
pub use lua_style::LuaStyle;
pub use lua_test_unit::LuaTestUnit;
pub use lua_view::LuaView;
pub use lua_yon::LuaYon;

pub struct LuaSource;

impl LuaSource {
    pub const EXTENSION: &'static str = "lua";
    pub const RUNTIME: &'static str = "lua";

    pub fn long_string(text: &str) -> String {
        let equals = "=".repeat(Self::closing_level(text));
        format!("[{equals}[{text}]{equals}]")
    }

    fn closing_level(text: &str) -> usize {
        let probe = format!("{text}]");
        (0..=text.len())
            .find(|level| !probe.contains(&format!("]{}]", "=".repeat(*level))))
            .unwrap_or_default()
    }
}
