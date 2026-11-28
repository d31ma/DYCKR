use super::super::{Input, LuaView};
use super::tokens::Tokens;
use crate::Failure;
use std::path::Path;

pub struct SchemaMarkers;

impl SchemaMarkers {
    pub fn contains(path: &Path, source: &str) -> Result<bool, Failure> {
        LuaView::either(path, source, Self::native_contains)
    }

    fn native_contains(path: &Path, source: &str) -> Result<bool, Failure> {
        let input = Input::new(path, source);
        let tokens = Tokens::read(&input)?;
        tokens.check_complete(&input)?;
        let python = Self::python_marked(&input)?;
        Ok([tokens.has_marker(Self::matches), python].contains(&true))
    }

    fn matches(written: &str) -> bool {
        let name = written.rsplit([':', '.', '\\']).next().unwrap_or(written);
        let name = name.replace('_', "");
        dyckr::schema_marker_names::NAMES
            .iter()
            .any(|known| known.eq_ignore_ascii_case(&name))
    }

    fn python_marked(input: &Input<'_>) -> Result<bool, Failure> {
        let code = input.masked_code()?;
        let found =
            dyckr::python_parameters::PythonAnnotated::markers(&code, input.source, Self::matches);
        Ok(![input.extension == "py", !found.markers.is_empty()].contains(&false))
    }
}
