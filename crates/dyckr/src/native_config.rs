use crate::valid_icon_source::valid_icon_source;
use crate::valid_route::valid_route;
use crate::config_failure::config_failure;
use crate::native_site::{RawCacheRule, SiteMetadata, SitemapValues};
use crate::failure::Failure;
use crate::failure::{diagnostic, source_span};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeApplication {
    pub name: String,
    pub executable_name: String,
    pub application_id: String,
    pub version: String,
    pub build: u32,
    pub entry_route: String,
    pub icons: Vec<(u32, String)>,
    pub window: WindowConfiguration,
    pub native_capabilities: Vec<String>,
    pub menus: crate::native_menu_config::NativeMenuConfig,
    pub tray: Option<crate::native_tray::NativeTray>,
    pub rust: crate::native_rust_project::NativeRustProject,
    pub outgoing: crate::native_outgoing_hosts::NativeOutgoingHosts,
    pub permissions: crate::browser_api_permissions::BrowserApiPermissions,
    pub minimum_system: crate::minimum_system::MinimumSystem,
    pub description: String,
    pub links: crate::native_links::NativeLinks,
    pub media: crate::media_member::MediaMember,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WindowConfiguration {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default)]
    pub min_width: Option<u32>,
    #[serde(default)]
    pub min_height: Option<u32>,
    #[serde(default)]
    pub controls: Vec<String>,
    #[serde(default)]
    pub title_bar: crate::native_title_bar::NativeTitleBar,
    #[serde(default)]
    pub title_bar_button_inset:
        Option<crate::native_title_bar_button_inset::NativeTitleBarButtonInset>,
    #[serde(skip)]
    pub theme_color: Option<crate::native_theme_color::NativeThemeColor>,
    #[serde(skip)]
    pub display: crate::native_display::NativeDisplay,
    #[serde(skip)]
    pub background_color: Option<crate::native_theme_color::NativeThemeColor>,
    #[serde(skip)]
    pub orientation: crate::native_orientation::NativeOrientation,
}

const fn default_width() -> u32 {
    1024
}

const fn default_height() -> u32 {
    768
}

impl Default for WindowConfiguration {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            min_width: None,
            min_height: None,
            controls: Vec::new(),
            title_bar: crate::native_title_bar::NativeTitleBar::default(),
            title_bar_button_inset: None,
            theme_color: None,
            display: crate::native_display::NativeDisplay::default(),
            background_color: None,
            orientation: crate::native_orientation::NativeOrientation::default(),
        }
    }
}

impl WindowConfiguration {
    pub const KNOWN: [&'static str; 6] = [
        "close",
        "fullscreen",
        "maximize",
        "minimize",
        "resize",
        "title",
    ];
    const MIN_EDGE: u32 = 64;
    const MAX_EDGE: u32 = 16_384;

    pub fn grants(&self, control: &str) -> bool {
        self.controls.iter().any(|value| value == control)
    }

    fn edge_is_supported(edge: u32) -> bool {
        (Self::MIN_EDGE..=Self::MAX_EDGE).contains(&edge)
    }

    fn has_duplicate_controls(&self) -> bool {
        let unique = self
            .controls
            .iter()
            .collect::<std::collections::BTreeSet<_>>();
        unique.len() != self.controls.len()
    }

    fn is_within_bounds(&self) -> bool {
        if !Self::edge_is_supported(self.width) || !Self::edge_is_supported(self.height) {
            return false;
        }
        if self.min_width.is_some_and(|value| value > self.width) {
            return false;
        }
        if self.min_height.is_some_and(|value| value > self.height) {
            return false;
        }
        if self.controls.len() > Self::KNOWN.len() {
            return false;
        }
        !self.has_duplicate_controls()
    }

    fn ensure_known_controls(&self) -> Result<(), Failure> {
        for control in &self.controls {
            if !Self::KNOWN.contains(&control.as_str()) {
                return Err(config_failure(&format!(
                    "'{control}' is not a window control. Grant any of: {}.",
                    Self::KNOWN.join(", ")
                )));
            }
        }
        Ok(())
    }

    fn ensure_button_inset_needs_integrated(&self) -> Result<(), Failure> {
        self.title_bar_button_inset
            .filter(|_| !self.title_bar.is_integrated())
            .map_or(Ok(()), |_| {
                Err(config_failure(
                    "Manifest tac.application.window.titleBarButtonInset needs \"titleBar\": \"integrated\".",
                ))
            })
    }

    fn ensure_button_inset_bounds(&self) -> Result<(), Failure> {
        self.title_bar_button_inset
            .filter(|inset| !inset.is_within_bounds())
            .map_or(Ok(()), |_| {
                Err(config_failure(&format!(
                    "Manifest tac.application.window.titleBarButtonInset leading and top must be 0 to {} points.",
                    crate::native_title_bar_button_inset::NativeTitleBarButtonInset::MAX_POINTS
                )))
            })
    }
}

fn manifest_metadata(path: &Path) -> Result<Option<fs::Metadata>, Failure> {
    let manifest_name = WebAppManifest::SOURCE_PATH;
    match fs::symlink_metadata(path) {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(config_failure(&format!(
            "Cannot inspect {manifest_name}: {error}"
        ))),
    }
}

pub fn read_manifest(project_root: &Path) -> Result<Option<WebAppManifest>, Failure> {
    let legacy_name = WebAppManifest::LEGACY_NAME;
    let manifest_name = WebAppManifest::SOURCE_PATH;
    if fs::symlink_metadata(project_root.join(legacy_name)).is_ok() {
        return Err(config_failure(&format!(
            "{legacy_name} at the project root moved to {manifest_name}, beside the client it describes (ADR 0034)."
        )));
    }
    let path = project_root.join(manifest_name);
    let Some(metadata) = manifest_metadata(&path)? else {
        return Ok(None);
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(config_failure(&format!(
            "{manifest_name} must be a regular, non-symlinked file."
        )));
    }
    if metadata.len() > WebAppManifest::MAX_BYTES {
        return Err(config_failure(&format!(
            "{manifest_name} exceeds the 1 MiB limit."
        )));
    }
    let source = fs::read_to_string(&path)
        .map_err(|error| config_failure(&format!("Cannot read {manifest_name}: {error}")))?;
    WebAppManifest::from_source(&source).map(Some)
}

fn valid_manifest_name(name: &str) -> bool {
    !name.trim().is_empty()
        && name.chars().count() <= 64
        && !name.chars().any(char::is_control)
        && !executable_name(name).is_empty()
}


fn valid_version(version: &str) -> bool {
    if version.is_empty() || version.len() > 64 {
        return false;
    }
    version
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'+'))
}

fn validate_manifest(manifest: &WebAppManifest) -> Result<(), Failure> {
    for name in [&manifest.name, &manifest.short_name].into_iter().flatten() {
        if !valid_manifest_name(name) {
            return Err(config_failure(
                "Manifest names must contain 1 to 64 printable characters and an ASCII letter or digit.",
            ));
        }
    }
    if let Some(route) = &manifest.start_url
        && !valid_route(route)
    {
        return Err(config_failure(
            "Manifest start_url must be a canonical local route.",
        ));
    }
    for icon in &manifest.icons {
        if !valid_icon_source(&icon.src) {
            return Err(config_failure(
                "Manifest icons must identify contained local bundle paths.",
            ));
        }
    }
    if manifest.tachyon.is_some() {
        return Err(config_failure(
            "The manifest member tachyon moved to tac.application. Move id, version and window there.",
        ));
    }
    if let Some(tac) = &manifest.tac
        && (tac.native_capabilities.len() > 1
            || tac
                .native_capabilities
                .iter()
                .any(|grant| grant != "filePicker"))
    {
        return Err(config_failure(
            "Manifest tac.nativeCapabilities accepts filePicker once.",
        ));
    }
    manifest.browser_api_permissions().validate()?;
    if let Some(tac) = &manifest.tac
        && let Some(yon) = &tac.yon
    {
        yon.validate(tac.outgoing.as_ref())?;
    }
    if let Some(tac) = &manifest.tac
        && let Some(media) = &tac.media
    {
        media.validate()?;
    }
    let Some(application) = manifest.application() else {
        return Ok(());
    };
    validate_manifest_application(application)
}

fn validate_manifest_application(application: &ManifestApplication) -> Result<(), Failure> {
    application.menus.validate()?;
    application
        .tray
        .as_ref()
        .map_or(Ok(()), |tray| tray.validate(&application.menus))?;
    if let Some(problem) = application.rust.problem() {
        return Err(config_failure(&problem));
    }
    if let Some(problem) = application.minimum_system.problem() {
        return Err(config_failure(problem));
    }
    if let Some(problem) = application.links.problem() {
        return Err(config_failure(problem));
    }
    if let Some(id) = &application.id
        && !valid_application_id(id)
    {
        return Err(config_failure(
            "Manifest tac.application.id must be a bounded lowercase reverse-DNS identifier, such as com.example.app.",
        ));
    }
    if let Some(version) = &application.version
        && !valid_version(version)
    {
        return Err(config_failure(
            "Manifest tac.application.version must be a portable version of at most 64 letters, digits, '.', '-' or '+', such as 1.4.0.",
        ));
    }
    if application.build == Some(0) {
        return Err(config_failure(
            "Manifest tac.application.build must be a whole number from 1 to 4294967295, such as 12.",
        ));
    }
    let Some(window) = &application.window else {
        return Ok(());
    };
    window.ensure_known_controls()?;
    window.ensure_button_inset_needs_integrated()?;
    window.ensure_button_inset_bounds()?;
    if !window.is_within_bounds() {
        return Err(config_failure(
            "Manifest tac.application.window sizes must be 64 to 16384, a minimum no larger than its size, and each control listed once.",
        ));
    }
    Ok(())
}

fn icon_edge(icon: &ManifestIcon) -> u32 {
    let Some(sizes) = icon.sizes.as_deref() else {
        return 0;
    };
    let Some(first_size) = sizes.split_whitespace().next() else {
        return 0;
    };
    let Some(width) = first_size.split(['x', 'X']).next() else {
        return 0;
    };
    width.parse::<u32>().unwrap_or(0)
}

fn is_svg(source: &str) -> bool {
    std::path::Path::new(source)
        .extension()
        .is_some_and(|value| value.eq_ignore_ascii_case("svg"))
}

fn icon_link(icon: &ManifestIcon) -> String {
    use crate::html_escape::html_attribute_escape;

    let kind = if is_svg(&icon.src) {
        r#" type="image/svg+xml""#
    } else {
        ""
    };
    let sizes = match icon.sizes.as_deref() {
        Some(value) => format!(r#" sizes="{}""#, html_attribute_escape(value)),
        None => String::new(),
    };
    format!(
        r#"<link rel="icon" href="{}"{kind}{sizes}>"#,
        html_attribute_escape(&icon.src)
    )
}

pub fn manifest_head(manifest: &WebAppManifest) -> String {
    use crate::html_escape::html_attribute_escape;
    use std::fmt::Write as _;

    let output_name = WebAppManifest::OUTPUT_NAME;
    let mut head = format!(r#"<link rel="manifest" href="/{output_name}">"#);
    for icon in &manifest.icons {
        head.push_str(&icon_link(icon));
    }
    if let Some(largest) = manifest.largest_icon() {
        let _ = write!(
            head,
            r#"<link rel="apple-touch-icon" href="{}">"#,
            html_attribute_escape(largest)
        );
    }
    head.push_str(&manifest.theme_color_meta());
    head
}

#[derive(Debug, Deserialize)]
pub struct WebAppManifest {
    name: Option<String>,
    short_name: Option<String>,
    start_url: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    icons: Vec<ManifestIcon>,
    #[serde(default)]
    theme_color: Option<serde_json::Value>,
    #[serde(default)]
    display: Option<crate::native_display::NativeDisplay>,
    #[serde(default)]
    background_color: Option<serde_json::Value>,
    #[serde(default)]
    orientation: Option<crate::native_orientation::NativeOrientation>,
    #[serde(default)]
    tac: Option<TacMember>,
    #[serde(default)]
    tachyon: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ManifestIcon {
    src: String,
    #[serde(default)]
    sizes: Option<String>,
}

impl WebAppManifest {
    pub fn from_captured(bytes: Option<&[u8]>) -> Result<Option<WebAppManifest>, Failure> {
        bytes.map(Self::read_captured).transpose()
    }

    fn read_captured(bytes: &[u8]) -> Result<WebAppManifest, Failure> {
        let length = u64::try_from(bytes.len()).map_err(|error| {
            config_failure(&format!("Cannot measure client/manifest.json: {error}"))
        })?;
        if length > WebAppManifest::MAX_BYTES {
            return Err(config_failure(
                "client/manifest.json exceeds the 1 MiB limit.",
            ));
        }
        let source = std::str::from_utf8(bytes).map_err(|error| {
            config_failure(&format!("Cannot read client/manifest.json: {error}"))
        })?;
        Self::from_source(source)
    }

    fn from_source(source: &str) -> Result<WebAppManifest, Failure> {
        let manifest_name = WebAppManifest::SOURCE_PATH;
        let manifest: WebAppManifest = serde_json::from_str(crate::without_bom::without_bom(source))
            .map_err(|error| config_failure(&format!("{manifest_name} is invalid: {error}")))?;
        validate_manifest(&manifest)?;
        Ok(manifest)
    }

    const LEGACY_NAME: &str = "manifest.json";
    const MAX_BYTES: u64 = 1_024 * 1_024;
    pub const SOURCE_PATH: &str = "client/manifest.json";
    pub const OUTPUT_NAME: &str = "manifest.webmanifest";

    pub fn largest_icon(&self) -> Option<&str> {
        self.icons
            .iter()
            .filter(|icon| !is_svg(&icon.src))
            .max_by_key(|icon| icon_edge(icon))
            .map(|icon| icon.src.as_str())
    }

    pub fn raster_icons(&self) -> Vec<(u32, String)> {
        let mut icons = Vec::new();
        for icon in &self.icons {
            if !is_svg(&icon.src) {
                icons.push((icon_edge(icon), icon.src.clone()));
            }
        }
        icons.sort_by_key(|(edge, _)| std::cmp::Reverse(*edge));
        icons
    }

    fn theme_color_meta(&self) -> String {
        use crate::html_escape::html_attribute_escape;

        self.theme_color
            .as_ref()
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|color| !color.is_empty())
            .map(|color| {
                format!(
                    r#"<meta name="theme-color" content="{}">"#,
                    html_attribute_escape(color)
                )
            })
            .unwrap_or_default()
    }

    fn display_name(&self) -> Option<String> {
        self.name.clone().or_else(|| self.short_name.clone())
    }

    fn application(&self) -> Option<&ManifestApplication> {
        self.tac.as_ref()?.application.as_ref()
    }

    pub fn browser_api_permissions(
        &self,
    ) -> crate::browser_api_permissions::BrowserApiPermissions {
        let granted = self
            .tac
            .as_ref()
            .map_or(&[][..], |tac| tac.browser_apis.as_slice());
        let usage = self
            .application()
            .map(|application| application.permissions.clone())
            .unwrap_or_default();
        crate::browser_api_permissions::BrowserApiPermissions::declared(granted, &usage)
    }

    pub fn link_hosts(&self) -> Vec<String> {
        self.application()
            .map(|application| application.links.hosts().to_vec())
            .unwrap_or_default()
    }

    pub fn into_tac(self) -> TacMember {
        self.tac.unwrap_or_default()
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct TacMember {
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    application: Option<ManifestApplication>,
    #[serde(default, rename = "nativeCapabilities")]
    pub native_capabilities: Vec<String>,
    #[serde(default)]
    pub scripts: Vec<String>,
    #[serde(default, rename = "blockingScripts")]
    pub blocking_scripts: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default, rename = "styleSources")]
    pub style_sources: BTreeMap<String, crate::style_source::StyleSource>,
    #[serde(default, rename = "scriptSources")]
    pub script_sources: BTreeMap<String, crate::vendor_provenance::VendorProvenance>,
    #[serde(default)]
    pub cache: Vec<RawCacheRule>,
    #[serde(default)]
    pub metadata: SiteMetadata,
    #[serde(default)]
    pub sitemap: BTreeMap<String, SitemapValues>,
    #[serde(default, rename = "browserApis")]
    pub browser_apis: Vec<String>,
    #[serde(default)]
    pub elements: Vec<String>,
    #[serde(default)]
    pub outgoing: Option<serde_json::Value>,
    #[serde(default)]
    pub yon: Option<crate::yon_member::YonMember>,
    #[serde(default)]
    pub storage: Option<crate::storage_member::StorageMember>,
    pub mount: Option<String>,
    #[serde(default)]
    pub apps: BTreeMap<String, String>,
    #[serde(default)]
    pub peers: Option<crate::peers_member::PeersMember>,
    #[serde(default)]
    pub media: Option<crate::media_member::MediaMember>,
    #[serde(default)]
    pub include: Vec<String>,
    pub layout: Option<String>,
}

impl TacMember {
    pub const FRAMEWORK_MEMBERS: [&str; 38] = [
        "origin",
        "application",
        "scripts",
        "blockingScripts",
        "styles",
        "styleSources",
        "scriptSources",
        "cache",
        "metadata",
        "sitemap",
        "cohesion",
        "duplication",
        "dependencies",
        "names",
        "signatures",
        "constants",
        "browserApis",
        "nativeCapabilities",
        "elements",
        "implicitmembers",
        "python",
        "visibility",
        "types",
        "tests",
        "accessibility",
        "simplify",
        "budgets",
        "outgoing",
        "go",
        "dotnet",
        "yon",
        "peers",
        "storage",
        "mount",
        "apps",
        "media",
        "include",
        "layout",
    ];
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestApplication {
    id: Option<String>,
    version: Option<String>,
    build: Option<u32>,
    #[serde(default)]
    window: Option<WindowConfiguration>,
    #[serde(default)]
    menus: crate::native_menu_config::NativeMenuConfig,
    #[serde(default)]
    tray: Option<crate::native_tray::NativeTray>,
    #[serde(default)]
    rust: crate::rust_companion_crate::RustCompanionCrate,
    #[serde(default)]
    permissions: BTreeMap<String, String>,
    #[serde(default, rename = "minimumSystem")]
    minimum_system: crate::minimum_system::MinimumSystem,
    #[serde(default)]
    links: crate::native_links::NativeLinks,
}

impl NativeApplication {
    pub fn discover(project_root: &Path) -> Result<Self, Failure> {
        Self::discover_from_snapshot(project_root, project_root)
    }

    pub fn discover_from_snapshot(
        project_root: &Path,
        authored_root: &Path,
    ) -> Result<Self, Failure> {
        let manifest = read_manifest(project_root)?;
        let mut application = Self::defaults(authored_root).with_manifest(manifest.as_ref());
        let declaration = manifest
            .as_ref()
            .and_then(WebAppManifest::application)
            .map(|declared| declared.rust.clone())
            .unwrap_or_default();
        application.rust = crate::native_rust_project::NativeRustProject::located(
            declaration,
            project_root,
            authored_root,
        );
        application.outgoing = crate::native_outgoing_hosts::NativeOutgoingHosts::declared(
            manifest
                .as_ref()
                .and_then(|declared| declared.tac.as_ref()?.outgoing.as_ref()),
        )?
        .with_yon(
            &manifest
                .and_then(|declared| declared.tac?.yon)
                .unwrap_or_default(),
        );
        Ok(application)
    }

    fn with_manifest(mut self, manifest: Option<&WebAppManifest>) -> Self {
        let Some(manifest) = manifest else {
            return self;
        };
        if let Some(name) = manifest.display_name()
            && !name.trim().is_empty()
        {
            self.executable_name = executable_name(&name);
            self.name = name;
        }
        if let Some(start) = &manifest.start_url
            && start.starts_with('/')
        {
            self.entry_route = String::from(start.trim_end_matches('/'));
            if self.entry_route.is_empty() {
                self.entry_route = String::from("/");
            }
        }
        if let Some(application) = manifest.application() {
            self.apply_manifest_application(application);
        }
        self.window.theme_color = crate::native_theme_color::NativeThemeColor::from_manifest(
            manifest.theme_color.as_ref(),
        );
        self.window.display = manifest.display.unwrap_or_default();
        self.window.background_color = crate::native_theme_color::NativeThemeColor::from_manifest(
            manifest.background_color.as_ref(),
        );
        self.window.orientation = manifest.orientation.unwrap_or_default();
        self.native_capabilities = manifest
            .tac
            .as_ref()
            .map_or_else(Vec::new, |tac| tac.native_capabilities.clone());
        self.icons = manifest.raster_icons();
        self.description = manifest.description.clone().unwrap_or_default();
        self.permissions = manifest.browser_api_permissions();
        self.media = manifest
            .tac
            .as_ref()
            .and_then(|tac| tac.media.clone())
            .unwrap_or_default();
        self
    }

    fn apply_manifest_application(&mut self, application: &ManifestApplication) {
        self.menus.clone_from(&application.menus);
        self.tray.clone_from(&application.tray);
        self.menus.tray = application
            .tray
            .as_ref()
            .map_or_else(Vec::new, |tray| tray.items.clone());
        self.minimum_system.clone_from(&application.minimum_system);
        self.links.clone_from(&application.links);
        if let Some(id) = &application.id {
            self.application_id.clone_from(id);
        }
        if let Some(version) = &application.version {
            self.version.clone_from(version);
        }
        if let Some(build) = application.build {
            self.build = build;
        }
        if let Some(window) = &application.window {
            self.window.clone_from(window);
        }
    }

    pub fn largest_icon(&self) -> Option<&str> {
        self.icons.first().map(|(_, src)| src.as_str())
    }

    pub fn icon_within(&self, edge: u32) -> Option<&str> {
        self.icons
            .iter()
            .find(|(size, _)| *size <= edge)
            .map(|(_, src)| src.as_str())
    }

    fn defaults(project_root: &Path) -> Self {
        let candidate = project_root
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("TachyonApp");
        let executable_name = executable_name(candidate);
        let identifier = identifier_segment(&executable_name);
        Self {
            name: executable_name.clone(),
            executable_name,
            application_id: format!("ma.del.tachyon.{identifier}"),
            version: String::from("1.0.0"),
            build: 1,
            entry_route: String::from("/"),
            icons: Vec::new(),
            window: WindowConfiguration::default(),
            native_capabilities: Vec::new(),
            menus: crate::native_menu_config::NativeMenuConfig::default(),
            tray: None,
            rust: crate::native_rust_project::NativeRustProject::default(),
            outgoing: crate::native_outgoing_hosts::NativeOutgoingHosts::default(),
            permissions: crate::browser_api_permissions::BrowserApiPermissions::default(),
            media: crate::media_member::MediaMember::default(),
            minimum_system: crate::minimum_system::MinimumSystem::default(),
            description: String::new(),
            links: crate::native_links::NativeLinks::default(),
        }
    }

    pub fn content_security_policy(&self) -> String {
        crate::content_security_policy::ContentSecurityPolicy::native_host(&self.outgoing.connect_sources())
    }
}

fn executable_name(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .take(64)
        .collect()
}

fn identifier_segment(value: &str) -> String {
    let mut identifier = String::with_capacity(value.len());
    for byte in value.bytes() {
        let lowercase = byte.to_ascii_lowercase();
        if lowercase.is_ascii_alphanumeric() {
            identifier.push(char::from(lowercase));
        } else {
            identifier.push('-');
        }
    }
    let trimmed = identifier.trim_matches('-');
    if trimmed.is_empty() {
        String::from("app")
    } else {
        String::from(trimmed)
    }
}

fn valid_application_id_segment(segment: &str) -> bool {
    if segment.is_empty() {
        return false;
    }
    let hyphen_allowed = !segment.starts_with('-') && !segment.ends_with('-');
    segment.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || (byte == b'-' && hyphen_allowed)
    })
}

fn valid_application_id(value: &str) -> bool {
    if value.len() > 255 || value.split('.').count() < 2 {
        return false;
    }
    value.split('.').all(valid_application_id_segment)
}
