use crate::Failure;
use dyckr::failure::{diagnostic, source_span};
use dyckr::native_site::SiteSettings;
use std::path::Path;

pub struct ScriptVendors;

impl ScriptVendors {
    pub const CODE: u16 = 1365;

    pub fn verify_vendor(site: &SiteSettings, path: &Path, bytes: &[u8]) -> Result<bool, Failure> {
        let relative = path
            .strip_prefix("client")
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let url = format!("/{relative}");
        site.script_sources.get(&url).map_or(Ok(false), |vendor| {
            Self::require_vendor_path(path)?;
            Self::require_script(site, &url, path)?;
            vendor.verify(&path.to_string_lossy().replace('\\', "/"), bytes)?;
            Ok(true)
        })
    }

    fn require_vendor_path(path: &Path) -> Result<(), Failure> {
        let shared = path.starts_with("client/shared");
        let utility = crate::annotation_layout::UtilityLayout::is_client_utility(path);
        if [!shared, utility].contains(&true) {
            return Err(Self::failure(
                path,
                "Script vendor exceptions require captured client/shared resources outside the registered utility folders.",
            ));
        }
        Ok(())
    }

    fn require_script(site: &SiteSettings, url: &str, path: &Path) -> Result<(), Failure> {
        if !site.scripts.iter().any(|script| script == url) {
            return Err(Self::failure(
                path,
                "Every tac.scriptSources vendor must also appear in tac.scripts.",
            ));
        }
        Ok(())
    }

    pub fn failure(path: &Path, message: &str) -> Failure {
        let portable = path.to_string_lossy().replace('\\', "/");
        Failure::one(diagnostic(Self::CODE, message, Some("Declare only exact pinned third-party resource paths in tac.scripts and tac.scriptSources; keep authored app markup and styles in component files.".to_owned()), source_span(&portable, 0, 0)))
    }
}
