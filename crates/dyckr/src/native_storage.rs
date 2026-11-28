pub const CACHE_METHODS: [&str; 3] = ["cacheable", "cacheEvict", "cachePut"];
pub const BUILT_IN: &str = "The built-in browser cache and Store work for JavaScript, TypeScript, Dart and Ruby services and for Go, Rust, C#, Kotlin and Swift WebAssembly services";

use crate::behavior_module::Type;

pub struct NeededAdapter {
    pub adapter: &'static str,
    pub uses: &'static str,
    pub name: String,
    pub owner: String,
    pub at: usize,
}

pub fn needed_adapters(types: &[Type]) -> Vec<NeededAdapter> {
        let mut needed = Vec::new();
        for ty in types {
            let caches = ty.methods.iter().any(|method| {
                crate::native_storage::CACHE_METHODS
                    .iter()
                    .any(|key| method.metadata.contains_key(*key))
            });
            let cache = ty.cache.as_ref().and_then(|cache| cache["name"].as_str());
            if let Some(name) = cache
                && caches
            {
                needed.push(NeededAdapter {
                    adapter: "a cache adapter",
                    uses: "the cache",
                    name: name.to_owned(),
                    owner: ty.name.clone(),
                    at: ty.declaration,
                });
            }
            let entity = ty
                .repository
                .as_ref()
                .and_then(|store| store["entity"].as_str());
            let adapter = types
                .iter()
                .find(|other| Some(other.name.as_str()) == entity)
                .and_then(|other| other.entity.as_ref())
                .and_then(|options| options["adapter"].as_str());
            if let Some(name) = adapter {
                needed.push(NeededAdapter {
                    adapter: "an entity adapter",
                    uses: "the entity adapter",
                    name: name.to_owned(),
                    owner: ty.name.clone(),
                    at: ty.declaration,
                });
            }
        }
        needed
    }

pub fn refusal(language: &str, needed: &NeededAdapter) -> String {
        let services = match language {
            "kt" => "Android Kotlin services",
            "swift" => "Swift services",
            "rs" => "Native Rust services",
            "cs" => "Native C# services",
            _ => "Native services",
        };
        format!(
            "{services} need {adapter} in their Setup: {owner} uses {uses} \"{name}\", and no Setup names it. {built_in}; {services} answer the page from another process, so they cannot reach them.",
            adapter = needed.adapter,
            owner = needed.owner,
            uses = needed.uses,
            name = needed.name,
            built_in = crate::native_storage::BUILT_IN,
        )
    }
