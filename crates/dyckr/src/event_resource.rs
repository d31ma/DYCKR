use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct EventResource {
    pub field: String,
    pub name: String,
}

impl EventResource {
    pub const TAC_NAMES: [&str; 30] = [
        "view",
        "lifecycle",
        "resourceLoader",
        "taskScheduler",
        "applicationEventPublisher",
        "clock",
        "secureRandom",
        "connectivity",
        "pageVisibility",
        "preferences",
        "preferenceStrings",
        "documentTitle",
        "clipboard",
        "share",
        "notifications",
        "geolocation",
        "windowControls",
        "focus",
        "push",
        "storageSpace",
        "passkeys",
        "peers",
        "media",
        "recorder",
        "codeScanner",
        "crypto",
        "blobs",
        "locks",
        "filePicker",
        "files",
    ];
    pub const RENAMED_NAMES: [(&str, &str); 4] = [
        ("element", "view"),
        ("signal", "lifecycle"),
        ("renderScheduler", "taskScheduler"),
        ("redis", "redisTemplate"),
    ];
    pub const FIELD_MODIFIERS: [&str; 6] = [
        "private ",
        "protected ",
        "public ",
        "readonly ",
        "declare ",
        "override ",
    ];
    pub const PUBLISHER: &str = "applicationEventPublisher";
    pub const REMOVED_CACHE_MANAGER: &str = "cacheManager";
    pub const CACHE_MANAGER_REFUSAL: &str = "The cacheManager resource was removed with the IndexedDB fetch cache (ADR 0087). Mark the HTTPClient method that changes the data @ClearCache or @UpdateCache, like Spring's @CacheEvict and @CachePut";

    pub fn new_name(old_name: &str) -> Option<&'static str> {
        for (old, new) in Self::RENAMED_NAMES {
            if old == old_name {
                return Some(new);
            }
        }
        None
    }
}

