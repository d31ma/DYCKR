pub struct SharedRuntimes;

impl SharedRuntimes {
    pub const BEHAVIOR_JAVASCRIPT: &'static str = include_str!("behavior/runtime.js");
    pub const CLIENT_JAVASCRIPT: &'static str = include_str!("behavior/client_runtime.js");
    pub const LUA_JSON: &'static str = include_str!("lua_source/lua_json.lua");
    pub const LUA_BEHAVIOR: &'static str = include_str!("lua_source/lua_behaviors.lua");
    pub const RUBY_BEHAVIOR: &'static str = include_str!("behavior/polyglot/ruby_runtime.rb");
    pub const RUST_NATIVE_HTTP_STUB: &'static str =
        include_str!("behavior/polyglot/behavior_native_http.rs");
    pub const GO_BEHAVIOR: &'static str = include_str!("behavior/polyglot/go_runtime.go");
    pub const GO_STORAGE: &'static str = include_str!("behavior/polyglot/go_storage.go");
    pub const GO_BROWSER_HTTP: &'static str = include_str!("behavior/polyglot/go_browser_http.go");
    pub const CSHARP_BEHAVIOR: &'static str = include_str!("behavior/polyglot/csharp_runtime.cs");
    pub const CSHARP_BROWSER_HTTP: &'static str =
        include_str!("behavior/polyglot/csharp_browser_http/BehaviorBrowserHttp.cs");
    pub const CSHARP_BROWSER_HTTP_ADAPTER: &'static str =
        include_str!("behavior/polyglot/csharp_browser_http/BehaviorBrowserHttpAdapter.cs");
    pub const CSHARP_HTTP_REVALIDATION: &'static str =
        include_str!("behavior/polyglot/csharp_browser_http/BehaviorHttpRevalidation.cs");
    pub const CSHARP_BROWSER_SOCKET: &'static str =
        include_str!("behavior/polyglot/csharp_browser_http/BehaviorBrowserSocket.cs");
    pub const JAVA_BEHAVIOR: &'static str = include_str!("behavior/polyglot/java_runtime.java");
    pub const SWIFT_BEHAVIOR: &'static str = include_str!("behavior/polyglot/swift_runtime.swift");
    pub const SWIFT_PLATFORM: &'static str =
        include_str!("behavior/polyglot/swift_foundation/BehaviorPlatform.swift");
    pub const SWIFT_LOCK: &'static str =
        include_str!("behavior/polyglot/swift_foundation/BehaviorLock.swift");
}
