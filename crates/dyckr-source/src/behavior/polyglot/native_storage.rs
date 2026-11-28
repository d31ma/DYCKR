use super::{Input, Marker, Type};
use crate::Failure;
use crate::behavior::HttpClientSourceExt;
use crate::behavior::ServiceRolesExt;
use std::path::Path;

pub struct NativeStorage;

impl NativeStorage {
    pub fn refuse_builtins(path: &Path, source: &str) -> Result<(), Failure> {
        let Some(rewritten) = dyckr::http_client_source::HttpClientSource::rewrite(path, source)?
        else {
            return Self::refuse_in(path, source);
        };
        let roles = crate::behavior::ServiceRoles::read(path, &rewritten)?;
        Self::refuse_in(path, &roles.source)
    }

    fn refuse_in(path: &Path, source: &str) -> Result<(), Failure> {
        let language = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let mask = dyckr::lexical::code_mask(language, source);
        let mut input = Input {
            path,
            source,
            code: super::visible_code(source, mask.code),
        };
        super::check_behavior_source(&input, mask.error, "tac")?;
        let markers: Vec<Marker> = super::annotations(&input, language, "tac")?
            .into_iter()
            .filter(|marker| {
                marker.name != "Schedule" && !dyckr::stored_state::StoredState::marks(&marker.name)
            })
            .collect();
        if markers.is_empty() {
            return Ok(());
        }
        super::mask_annotations(&mut input, &markers, language);
        let types = super::behavior_types(&input, language, "tac", &markers)?;
        let setups = Self::setup_texts(&input, language, &types);
        for needed in dyckr::native_storage::needed_adapters(&types) {
            let quoted = format!("\"{}\"", needed.name);
            if !setups.iter().any(|setup| setup.contains(&quoted)) {
                return Err(
                    input.failure(needed.at, dyckr::native_storage::refusal(language, &needed))
                );
            }
        }
        Ok(())
    }

    fn setup_texts<'a>(input: &Input<'a>, language: &str, types: &[Type]) -> Vec<&'a str> {
        let mut texts = Vec::new();
        for setup in types.iter().filter(|ty| ty.configuration) {
            let body = if language == "rs" {
                super::rust_impl_body(input, &setup.name).unwrap_or_else(|| setup.body.clone())
            } else {
                setup.body.clone()
            };
            texts.push(&input.source[body]);
        }
        texts
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use super::NativeStorage;
    use std::path::Path;

    const KOTLIN_SERVICE: &str = "@CacheSettings({\"name\":\"catalog\",\"ttlMs\":30000,\"maxBytes\":65536,\"maxEntries\":256})\nclass CatalogService {\n    @Cached\n    fun find(sku: String): Any? {\n        return mapOf(\"sku\" to sku)\n    }\n}\n";
    const KOTLIN_SETUP: &str = "@Setup\nclass CatalogSetup {\n    companion object {\n        fun configure(execution: Behaviors.Execution): Behaviors.Configuration = Behaviors.Configuration(\n            caches = mapOf(\"catalog\" to MemoryCache()))\n    }\n}\n";
    const KOTLIN_EMPTY_SETUP: &str = "@Setup\nclass CatalogSetup {\n    companion object {\n        fun configure(execution: Behaviors.Execution): Behaviors.Configuration = Behaviors.Configuration()\n    }\n}\n";
    const SWIFT_STORE: &str = "@Entity({\"adapter\":\"publicCatalog\"})\n@Table(\"products\")\nfinal class ProductEntity {\n    @Id\n    var id: Int = 0\n}\n@Store(ProductEntity, {\"fields\":[\"id\",\"status\"],\"defaultSize\":20,\"maxSize\":100})\nfinal class ProductStore {\n    @Needs\n    let behaviors: BehaviorContext\n    func findByStatus(_ status: BehaviorJSON) async throws -> BehaviorJSON {}\n}\n";
    const RUST_SERVICE: &str = "#[CacheSettings({\"name\":\"catalog\",\"ttlMs\":30000,\"maxBytes\":65536,\"maxEntries\":256})]\nstruct CatalogService {}\nimpl CatalogService {\n    #[Cached]\n    fn find(&self, sku: BehaviorValue) -> BehaviorResult<BehaviorValue> {\n        Ok(sku)\n    }\n}\n";
    const RUST_SETUP: &str = "#[tac::setup]\nstruct CatalogSetup {}\nimpl CatalogSetup {\n    fn configure(_execution: &BehaviorExecution) -> BehaviorResult<BehaviorConfiguration> {\n        let mut configuration = BehaviorConfiguration::default();\n        configuration.caches.insert(\"catalog\".into(), std::sync::Arc::new(MemoryCache::default()));\n        Ok(configuration)\n    }\n}\n";

    fn refusal(path: &str, source: &str) -> String {
        NativeStorage::refuse_builtins(Path::new(path), source)
            .expect_err("refused")
            .to_string()
    }

    #[test]
    fn a_kotlin_cache_without_a_setup_adapter_is_refused() {
        let message = refusal("client/services/CatalogService.kt", KOTLIN_SERVICE);
        assert!(
            message.contains("Kotlin services need a cache adapter in their Setup"),
            "{message}"
        );
        assert!(message.contains("\"catalog\""), "{message}");
        assert!(
            message.contains("Go, Rust, C#, Kotlin and Swift WebAssembly"),
            "{message}"
        );
        let empty = format!("{KOTLIN_SERVICE}{KOTLIN_EMPTY_SETUP}");
        assert!(refusal("client/services/CatalogService.kt", &empty).contains("Kotlin services"));
    }

    #[test]
    fn a_kotlin_cache_named_by_the_setup_is_accepted() {
        let source = format!("{KOTLIN_SERVICE}{KOTLIN_SETUP}");
        NativeStorage::refuse_builtins(Path::new("client/services/CatalogService.kt"), &source)
            .expect("the Setup names the cache");
    }

    #[test]
    fn a_swift_store_without_an_entity_adapter_is_refused() {
        let message = refusal("client/services/ProductService.swift", SWIFT_STORE);
        assert!(
            message.contains("Swift services need an entity adapter in their Setup"),
            "{message}"
        );
        assert!(message.contains("\"publicCatalog\""), "{message}");
    }

    #[test]
    fn a_native_rust_cache_needs_its_setup_adapter() {
        let message = refusal("client/services/CatalogService.rs", RUST_SERVICE);
        assert!(
            message.contains("Native Rust services need a cache adapter in their Setup"),
            "{message}"
        );
        let source = format!("{RUST_SERVICE}{RUST_SETUP}");
        NativeStorage::refuse_builtins(Path::new("client/services/CatalogService.rs"), &source)
            .expect("the namespaced Setup names the cache");
    }

    #[test]
    fn a_service_without_storage_markers_is_accepted() {
        NativeStorage::refuse_builtins(
            Path::new("client/services/PlainService.kt"),
            "class PlainService {\n    var count: Int = 0\n}\n",
        )
        .expect("plain");
    }
}
