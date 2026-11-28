use crate::failure::Failure;
use crate::native_menu_entry::NativeMenuEntry;
use crate::native_route_index::NativeRouteIndex;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NativeMenuConfig {
    #[serde(default)]
    pub file: Vec<NativeMenuEntry>,
    #[serde(default)]
    pub view: Vec<NativeMenuEntry>,
    #[serde(default)]
    pub help: Vec<NativeMenuEntry>,
    #[serde(skip)]
    pub tray: Vec<NativeMenuEntry>,
}

impl NativeMenuConfig {
    const MAX_ITEMS: usize = 64;

    pub fn validate(&self) -> Result<(), Failure> {
        self.require_count()?;
        self.items().try_for_each(NativeMenuEntry::validate)?;
        self.require_ids()?;
        self.require_shortcuts()
    }

    const CTRL_TARGETS: [tachyon_contracts::NativeTarget; 3] = [
        tachyon_contracts::NativeTarget::Linux,
        tachyon_contracts::NativeTarget::Windows,
        tachyon_contracts::NativeTarget::Android,
    ];

    pub fn validate_for(
        &self,
        target: tachyon_contracts::NativeTarget,
        index: &NativeRouteIndex,
    ) -> Result<(), Failure> {
        self.require_desktop_shortcuts(target)?;
        self.items()
            .try_for_each(|item| Self::require_document(item, index))
    }

    pub fn is_empty(&self) -> bool {
        self.groups().iter().all(|(_, entries)| entries.is_empty())
    }

    pub fn groups(&self) -> [(&'static str, &[NativeMenuEntry]); 3] {
        [
            ("file", &self.file),
            ("view", &self.view),
            ("help", &self.help),
        ]
    }

    pub fn table_groups(&self) -> [(&'static str, &[NativeMenuEntry]); 4] {
        let [file, view, help] = self.groups();
        [file, view, help, ("tray", &self.tray)]
    }

    pub fn items(&self) -> impl Iterator<Item = &NativeMenuEntry> {
        self.file
            .iter()
            .chain(&self.view)
            .chain(&self.help)
            .chain(&self.tray)
    }

    fn require_count(&self) -> Result<(), Failure> {
        (self.items().count() <= Self::MAX_ITEMS)
            .then_some(())
            .ok_or_else(|| {
                crate::config_failure::config_failure(
                    "Native menus may declare at most 64 items in total.",
                )
            })
    }

    fn require_ids(&self) -> Result<(), Failure> {
        let ids = self
            .items()
            .map(|item| &item.id)
            .collect::<std::collections::BTreeSet<_>>();
        (ids.len() == self.items().count())
            .then_some(())
            .ok_or_else(|| {
                crate::config_failure::config_failure(
                    "Native menu ids must be unique across File, View, Help and the tray.",
                )
            })
    }

    fn require_shortcuts(&self) -> Result<(), Failure> {
        let shortcuts = self
            .items()
            .filter_map(|item| item.shortcut.as_ref())
            .map(crate::native_menu_shortcut::NativeMenuShortcut::identity)
            .collect::<Vec<_>>();
        let unique = shortcuts.iter().collect::<std::collections::BTreeSet<_>>();
        (unique.len() == shortcuts.len())
            .then_some(())
            .ok_or_else(|| {
                crate::config_failure::config_failure(
                    "Native menu shortcuts must be unique across File, View and Help.",
                )
            })
    }

    fn require_desktop_shortcuts(
        &self,
        target: tachyon_contracts::NativeTarget,
    ) -> Result<(), Failure> {
        if !Self::CTRL_TARGETS.contains(&target) {
            return Ok(());
        }
        let shortcuts = self
            .items()
            .filter_map(|item| item.shortcut.as_ref())
            .collect::<Vec<_>>();
        shortcuts
            .iter()
            .try_for_each(|shortcut| shortcut.require_desktop_available(target))?;
        let unique = shortcuts
            .iter()
            .map(|shortcut| shortcut.desktop_identity())
            .collect::<std::collections::BTreeSet<_>>();
        (unique.len() == shortcuts.len())
            .then_some(())
            .ok_or_else(|| {
                crate::config_failure::config_failure(&format!(
                    "Native menu shortcuts must stay unique on {target:?}, where control is Ctrl like command; two shortcuts that differ only by control and command get the same key, so change one of them.",
                ))
            })
    }

    pub fn require_document(
        item: &NativeMenuEntry,
        index: &NativeRouteIndex,
    ) -> Result<(), Failure> {
        item.route
            .as_deref()
            .map_or(Ok(()), |route| Self::require_page(&item.id, route, index))
    }

    fn require_page(id: &str, route: &str, index: &NativeRouteIndex) -> Result<(), Failure> {
        let valid = index.routes.iter().any(|entry| entry.route == route)
            && !route.split('/').any(|segment| segment.starts_with('_'));
        valid.then_some(()).ok_or_else(|| crate::config_failure::config_failure(&format!(
            "Native menu '{id}' destination '{route}' must exactly identify a compiled static page route.",
        )))
    }
}
