use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NativeTitleBarButtonInset {
    pub leading: u32,
    pub top: u32,
}

impl NativeTitleBarButtonInset {
    pub const MAX_POINTS: u32 = 200;

    pub fn is_within_bounds(self) -> bool {
        self.leading.max(self.top) <= Self::MAX_POINTS
    }
}
