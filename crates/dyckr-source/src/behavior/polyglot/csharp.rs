use super::Module;

pub(super) fn lower(module: &Module) -> Result<String, String> {
    dyckr::csharp::lower(
        module,
        &|module_id, environment, registry, configurations, entities, policies| {
            runtime(
                module_id,
                environment,
                registry,
                configurations,
                entities,
                policies,
            )
        },
        &|module, type_declaration, unit| {
            format!(
                "{}{}",
                dyckr::csharp::test_members(
                    unit,
                    &module.environment,
                    &super::super::ValueSource::csharp_calls(&unit.cases, &unit.inputs)
                ),
                super::typed_test_construction::TypedTestConstruction::members(
                    module,
                    type_declaration,
                    unit,
                    "cs"
                ),
            )
        },
        &|module| {
            super::super::testing::unit_helper(&module.types, super::super::TestAssert::CSHARP)
        },
        &|t, m, p| super::csharp_model_exchange::CsharpModelExchange::body(t, m, p),
    )
}

pub(super) fn runtime(
    module_id: &str,
    environment: &str,
    registry: &str,
    configurations: &str,
    entities: &str,
    policies: &str,
) -> String {
    dyckr::csharp::runtime(
        module_id,
        environment,
        registry,
        configurations,
        entities,
        policies,
        &[
            include_str!("csharp_runtime.cs"),
            include_str!("csharp_browser_http/BehaviorBrowserHttp.cs"),
            include_str!("csharp_browser_http/BehaviorBrowserHttpAdapter.cs"),
            include_str!("csharp_browser_http/BehaviorHttpRevalidation.cs"),
            include_str!("csharp_browser_http/BehaviorBrowserSocket.cs"),
        ],
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    fn dotnet_is_available() -> bool {
        std::process::Command::new("dotnet")
            .arg("--version")
            .output()
            .is_ok()
    }

    fn annotated_source() -> String {
        let annotations = include_str!("../../../tests/fixtures/behavior/csharp-annotations.cs")
            .replace(
                "using var signal =",
                "ConfigurationAnnotationTests.Run(); await ValidationAnnotationTests.Run(); using var signal =",
            );
        format!(
            "{}\n{}\n{}",
            annotations,
            include_str!("../../../tests/fixtures/behavior/csharp-configuration.cs"),
            include_str!("../../../tests/fixtures/behavior/csharp-validation.cs")
        )
    }

    async fn compile_and_execute(emitted: String) {
        let directory = tempfile::tempdir().expect("emitted source directory");
        let path = directory.path().join("Emitted.cs");
        std::fs::write(&path, emitted).expect("emitted C# source");
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scripts/behavior-native-runtime-test.mjs");
        let mut command = dyckr::node_driver::NodeDriver::async_command();
        command.arg(script).arg("cs").arg("--emitted").arg(path);
        let result = dyckr::node_driver::NodeDriver::run_within(
            &mut command,
            std::time::Duration::from_mins(4),
        )
        .await
        .expect("C# compile and execution tool");
        assert!(
            result.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    }

    fn lower_client(source: &str) -> String {
        super::super::lower(
            std::path::Path::new("server/remotes/ReportClient.cs"),
            source,
            "yon",
        )
        .expect("C# gateway lowering")
        .expect("lowered source")
    }

    #[test]
    fn a_gateway_method_calls_the_transport_without_a_behavior_context() {
        let lowered = lower_client(
            "class ReportClient {\n    [Gateway({\"class\":\"server/services/ReportService.py\"})]\n    public BehaviorJSON Summarize(BehaviorJSON mode, BehaviorJSON detail) {}\n}\n",
        );
        assert!(lowered.contains(
            "public BehaviorJSON Summarize(BehaviorJSON mode, BehaviorJSON detail) {\nreturn Behaviors.GatewayResult(YonGateway.Call(\"ReportClient.Summarize\", Behaviors.GatewayArguments(new BehaviorJSON[] { mode, detail })));\n}"
        ));
        assert!(!lowered.contains("BehaviorContext behaviors"));
    }

    #[test]
    fn a_gateway_interface_becomes_a_class_without_a_behavior_context() {
        let lowered = lower_client(
            "interface ReportClient {\n    [Gateway({\"command\":[\"python3\"]})]\n    BehaviorJSON Ping();\n}\n",
        );
        assert!(lowered.contains("class ReportClient {"));
        assert!(lowered.contains(
            "public BehaviorJSON Ping() { \nreturn Behaviors.GatewayResult(YonGateway.Call(\"ReportClient.Ping\", Behaviors.GatewayArguments(new BehaviorJSON[] {  })));\n }"
        ));
        assert!(!lowered.contains("BehaviorContext behaviors"));
    }

    #[test]
    fn a_gateway_parameter_must_be_behavior_json() {
        let failure = super::super::lower(
            std::path::Path::new("server/remotes/ReportClient.cs"),
            "class ReportClient {\n    [Gateway({\"command\":[\"python3\"]})]\n    BehaviorJSON Ping(string mode) {}\n}\n",
            "yon",
        )
        .expect_err("typed parameter");
        let message = failure.to_string();
        assert!(message.contains("TY1312"), "{message}");
        assert!(
            message.contains("must take BehaviorJSON parameters"),
            "{message}"
        );
    }

    #[tokio::test]
    async fn csharp_json_serializes_without_reflection() {
        if !dotnet_is_available() {
            return;
        }
        let runtime = super::runtime("json-test", "tac", "{}", "", "", "");
        let checks = r#"
public static class EmittedBehaviorTests {
    public static void Main() {
        if (System.Text.Json.JsonSerializer.IsReflectionEnabledByDefault)
            throw new System.Exception("reflection must be disabled");
        var text = "Catalog \"title\"\n<\u00e9>\\";
        var key = "a\"\n";
        var value = BehaviorJSON.Object(new System.Collections.Generic.Dictionary<string, BehaviorJSON> {
            ["z"] = BehaviorJSON.Array(BehaviorJSON.From(true), BehaviorJSON.From(25), BehaviorJSON.From(null)),
            [key] = BehaviorJSON.From(text),
        });
        var encoded = value.ToString();
        if (!encoded.StartsWith("{\"a") || encoded.Contains("\n"))
            throw new System.Exception("canonical order or escaping changed");
        var decoded = BehaviorJSON.Parse(encoded);
        if (decoded.Get(key).Text != text || !decoded.Get("z").Items[0].Boolean
            || decoded.Get("z").Items[1].Integer != 25 || !decoded.Get("z").Items[2].IsNull)
            throw new System.Exception("JSON round trip changed");
    }
}
"#;
        compile_and_execute(format!("{runtime}\n{checks}")).await;
    }

    #[tokio::test]
    async fn a_tac_service_without_adapters_uses_the_page_storage_host() {
        if !dotnet_is_available() {
            return;
        }
        let source = r#"[Setup]
public sealed class StorageSetup {
    public static BehaviorAdapters configure(BehaviorExecution execution) { return new BehaviorAdapters(); }
}
[Entity({"adapter":"indexeddb"})]
[Table("notes")]
public sealed class Note {
    [Id]
    public string id;
}
[CacheSettings({"name":"notes","ttlMs":60000,"maxBytes":4096,"maxEntries":8})]
public sealed class NotesCatalog {
    [Needs]
    public BehaviorContext behaviors;
    public int reads = 0;
    [Cached]
    public async System.Threading.Tasks.Task<BehaviorJSON> find(BehaviorJSON key) {
        await System.Threading.Tasks.Task.CompletedTask;
        reads++;
        return BehaviorJSON.From(reads);
    }
    [ClearCache({"method":"find"})]
    public async System.Threading.Tasks.Task<BehaviorJSON> forget(BehaviorJSON key) {
        await System.Threading.Tasks.Task.CompletedTask;
        return key;
    }
    [AllOrNothing({"adapter":"indexeddb"})]
    public async System.Threading.Tasks.Task<BehaviorJSON> refuse(BehaviorJSON id) {
        await behaviors.repository(typeof(Note)).save(BehaviorJSON.Object(new System.Collections.Generic.Dictionary<string, BehaviorJSON> { ["id"] = id }));
        throw new BehaviorError("REFUSED");
    }
}
public static class PageStorage {
    public static readonly System.Collections.Generic.Dictionary<string, string> Cache = new System.Collections.Generic.Dictionary<string, string>();
    public static readonly System.Collections.Generic.SortedDictionary<string, string> Rows = new System.Collections.Generic.SortedDictionary<string, string>();
    public static readonly System.Collections.Generic.Dictionary<string, string> Staged = new System.Collections.Generic.Dictionary<string, string>();
    public static string Answer(string text) {
        var request = BehaviorJSON.Parse(text);
        var operation = request.Get("op").Text;
        if (operation == "cache.get") return Cache.TryGetValue(request.Get("key").Text, out var found) ? "{\"value\":" + found + "}" : "{\"value\":{\"hit\":false}}";
        if (operation == "cache.set") { Cache[request.Get("key").Text] = "{\"hit\":true,\"value\":" + request.Get("value") + ",\"expiresAt\":" + request.Get("expiresAt") + "}"; return "{\"value\":null}"; }
        if (operation == "cache.delete") { Cache.Remove(request.Get("key").Text); return "{\"value\":null}"; }
        var command = request.Get("command");
        var action = command.Get("operation").Text;
        if (action == "transaction") {
            if (command.Get("action").Text == "begin") return "{\"value\":{\"transaction\":\"memory-1\"}}";
            if (command.Get("action").Text == "commit") foreach (var pair in Staged) Rows[pair.Key] = pair.Value;
            Staged.Clear();
            return "{\"value\":null}";
        }
        if (action == "save") { (command.Has("transaction") ? (System.Collections.Generic.IDictionary<string, string>)Staged : Rows)[command.Get("value").Get("id").Text] = command.Get("value").ToString(); return "{\"value\":" + command.Get("value") + "}"; }
        if (action == "find") return "{\"value\":[" + string.Join(",", Rows.Values) + "]}";
        return "{\"error\":\"The IndexedDB adapter does not know the operation '" + action + "'.\"}";
    }
}
public static class EmittedBehaviorTests {
    public static async System.Threading.Tasks.Task Main() {
        BehaviorStorage.Host = PageStorage.Answer;
        using var signal = new BehaviorSignal();
        using var context = Behaviors.create(typeof(StorageSetup), "notes", signal);
        var catalog = new NotesCatalog(context);
        var key = BehaviorJSON.From("a");
        var first = await catalog.find(key);
        var second = await catalog.find(key);
        await catalog.forget(key);
        var third = await catalog.find(key);
        if (first.Integer != 1 || second.Integer != 1 || third.Integer != 2) throw new System.Exception("the page cache was not used: " + first + second + third);
        await context.repository(typeof(Note)).save(BehaviorJSON.Object(new System.Collections.Generic.Dictionary<string, BehaviorJSON> { ["id"] = BehaviorJSON.From("n1") }));
        try { await catalog.refuse(BehaviorJSON.From("n2")); throw new System.Exception("refuse must fail"); }
        catch (BehaviorError error) when (error.Code == "REFUSED") { }
        var listed = await context.repository(typeof(Note)).find(BehaviorJSON.Parse("{}"));
        if (listed.ToString() != "[{\"id\":\"n1\"}]") throw new System.Exception("the refused note was saved: " + listed);
        System.Console.WriteLine("C# storage host: cache and store");
    }
}
"#;
        let emitted =
            super::super::lower(std::path::Path::new("client/pages/tac.cs"), source, "tac")
                .expect("C# storage lowering")
                .expect("annotated source");
        compile_and_execute(emitted).await;
    }

    #[tokio::test]
    async fn csharp_all_annotations_compile_and_execute() {
        if !dotnet_is_available() {
            return;
        }
        let path = std::path::Path::new("client/pages/tac.cs");
        let emitted = super::super::lower(path, &annotated_source(), "tac")
            .expect("C# annotation lowering")
            .expect("annotated source");
        assert!(emitted.contains("public AnnotatedCatalog(BehaviorContext behaviors)"));
        compile_and_execute(emitted).await;
    }
}
