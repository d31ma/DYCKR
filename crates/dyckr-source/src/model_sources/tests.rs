use std::path::{Path, PathBuf};

#[test]
fn rust_yon_codecs_are_owned_by_private_field_model_modules() {
    let root = tempfile::tempdir().expect("project");
    let path = root.path().join("server/models/PrivateInput.rs");
    std::fs::create_dir_all(path.parent().expect("models")).expect("directory");
    let source = "#[Checked]\npub struct PrivateInput {\n #[Min(1)]\n value: i64,\n}\n";
    std::fs::write(&path, source).expect("model");
    let models = super::ModelSources::read(root.path(), "server", "rs").expect("schema");
    let program = models.add_to_yon_program("struct Handler;", true);
    let module = program
        .split("mod model_0 {")
        .nth(1)
        .expect("owning namespace");
    assert!(module.contains("use super::yon_codec_names::*;"));
    assert!(program.contains("pub(super) use crate::{Json, YonRead};"));
    assert!(module.contains("impl YonRead for PrivateInput"));
    assert!(module.contains("impl From<PrivateInput> for Json"));
    assert!(module.contains("value.value"));
    assert!(module.contains("value: i64"));
    assert!(!module.contains("pub value"));
    assert!(!program.contains("impl YonRead for crate::models::"));
    assert!(
        crate::route_types::Converters::program_text("rs", root.path())
            .expect("server codecs")
            .is_empty()
    );
    assert_eq!(std::fs::read_to_string(path).expect("authored"), source);
}

#[test]
fn rust_plain_handler_and_native_models_keep_their_prior_projection() {
    let root = tempfile::tempdir().expect("project");
    let path = root.path().join("server/models/PrivateInput.rs");
    std::fs::create_dir_all(path.parent().expect("models")).expect("directory");
    std::fs::write(path, "pub struct PrivateInput { value: i64 }\n").expect("model");
    let models = super::ModelSources::read(root.path(), "server", "rs").expect("plain model");
    let plain = models.add_to_program("struct Handler;");
    assert_eq!(models.add_to_yon_program("struct Handler;", false), plain);
    assert!(!plain.contains("YonRead"));
    assert!(!plain.contains("Json"));
    assert_eq!(
        models.rust_module(),
        "#[allow(dead_code, unused_imports)]\npub mod models {\npub use self::model_0::*;\n#[doc = \"server/models/PrivateInput.rs\"]\nmod model_0 {\npub struct PrivateInput { value: i64 }\n\n}\n}\n"
    );
}

#[test]
fn java_yon_checked_models_preserve_metadata_before_sdk_projection() {
    let root = tempfile::tempdir().expect("project");
    let path = root.path().join("server/models/OrderInput.java");
    std::fs::create_dir_all(path.parent().expect("parent")).expect("models");
    let source = "import java.util.List;\n\n@Checked\nclass OrderInput {\n    @Required\n    String name;\n}\n";
    std::fs::write(&path, source).expect("model");
    let models = super::ModelSources::read(root.path(), "server", "java").expect("schema model");
    let entry = Path::new("server/services/OrderService.java");
    let authored = models.metadata().assembled(entry, "@Service\nclass OrderService {\n    Object accept(@Check(OrderInput) Object order) { return order; }\n}\n").expect("metadata projection");
    let lowered = crate::behavior::lower(entry, &authored, "yon")
        .expect("schema lowering")
        .expect("annotated source");
    assert!(lowered.contains("case \"OrderInput\": return new Rule[]"));
    assert!(lowered.contains("new Rule(\"name\", true, false, null, null, null, null)"));
    let role = models
        .metadata()
        .without_declarations(entry, &lowered)
        .expect("data separated");
    assert!(!role.contains("class OrderInput"));
    let sdk = models.add_to_program(&role);
    assert!(!sdk.contains("@Checked") && !sdk.contains("@Required"));
    assert!(sdk.contains("class OrderInput") && sdk.contains("String name"));
}

#[test]
fn groovy_metadata_removes_schema_declarations_after_an_import() {
    let path = Path::new("server/controllers/ConfigurationController.groovy");
    let schema = "@Settings({\"prefix\":\"YON_CATALOG_\"})\nclass CatalogSettings {\n    int pageSize = 20\n}\n";
    let controller = "@RequestRoute(\"/configuration\")\nclass ConfigurationController {\n}\n";
    let metadata = super::ModelMetadata::new(vec![(
        PathBuf::from("server/models/CatalogSettings.groovy"),
        schema.to_owned(),
    )]);
    let composed = format!("import models.Configuration\n{schema}\n{controller}");
    let retained = metadata
        .without_declarations(path, &composed)
        .expect("schema separated");
    assert!(!retained.contains("class CatalogSettings"));
    assert!(retained.contains("import models.Configuration"));
    assert!(retained.contains(controller));
}

#[test]
fn javascript_metadata_removes_only_injected_schema_declarations() {
    let path = Path::new("client/services/InputService.js");
    let schema = "@Checked\nclass ProductInput {\n  @Required\n  name = '';\n}\n";
    let service =
        "@Service\nexport default class InputService { accept(value) { return value; } }\n";
    let metadata = super::ModelMetadata::new(vec![(
        PathBuf::from("client/models/ProductInput.js"),
        schema.to_owned(),
    )]);
    let composed = format!("{schema}\n{service}");
    let retained = metadata
        .without_declarations(path, &composed)
        .expect("schema separated");
    assert!(!retained.contains("class ProductInput"));
    assert!(retained.contains(service));
    assert_eq!(retained.len(), composed.len());
}

#[test]
fn lua_schema_models_are_read_in_both_realms_and_role_markers_stay_refused() {
    let root = tempfile::tempdir().expect("project");
    let source = "-- @Checked\n---@class OrderInput\nOrderInput = {\n  -- @Required\n  ---@type string\n  name = nil,\n}\n";
    for realm in ["client", "server"] {
        let path = root.path().join(format!("{realm}/models/OrderInput.lua"));
        std::fs::create_dir_all(path.parent().expect("parent")).expect("models");
        std::fs::write(&path, source).expect("schema model");
        let models = super::ModelSources::read(root.path(), realm, "lua")
            .unwrap_or_else(|failure| panic!("{realm}: {failure}"));
        assert!(
            models.joined_sources().contains("OrderInput = {"),
            "{realm}"
        );
    }
    let path = root.path().join("server/models/OrderInput.lua");
    std::fs::write(&path, source.replace("-- @Checked", "-- @Service")).expect("role model");
    let failure = super::ModelSources::read(root.path(), "server", "lua")
        .err()
        .expect("a role marker on a Lua model stays refused");
    assert_eq!(failure.diagnostics()[0].code.number(), 1310);
    assert_eq!(
        failure.diagnostics()[0].spans[0].file,
        "server/models/OrderInput.lua"
    );
}

fn write(root: &Path, relative: &str, source: &str) -> PathBuf {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("directory");
    std::fs::write(&path, source).expect("source");
    path
}

#[test]
fn projects_without_models_keep_their_programs() {
    let root = tempfile::tempdir().expect("project");
    write(
        root.path(),
        "server/models/Guide.ts",
        "export interface Guide {}\n",
    );
    for extension in ["rs", "cs", "py", "php", "java", "swift", "go"] {
        let models = super::ModelSources::read(root.path(), "server", extension).expect("read");
        assert!(!models.changes_program_text(), "{extension}");
        assert_eq!(models.add_to_program("program"), "program");
        assert!(models.separate_files().is_empty());
    }
    let typescript = super::ModelSources::read(root.path(), "server", "ts").expect("read");
    assert!(typescript.separate_files().is_empty());
}

#[test]
fn go_models_are_staged_as_files_of_package_main() {
    let root = tempfile::tempdir().expect("project");
    write(
        root.path(),
        "server/models/Guide.go",
        "package main\n\ntype Guide struct {\n\tTitle string `json:\"title\"`\n}\n",
    );
    write(
        root.path(),
        "server/models/nested/Kind.go",
        "// kinds\ntype Kind string\n",
    );
    let models = super::ModelSources::read(root.path(), "server", "go").expect("read");
    assert!(!models.changes_program_text());
    assert_eq!(models.add_to_program("program"), "program");
    let staged = models.separate_files();
    assert_eq!(
        staged,
        vec![
            (
                PathBuf::from("model_0.go"),
                String::from(
                    "//line server/models/Guide.go:1\npackage main\n\ntype Guide struct {\n\tTitle string `json:\"title\"`\n}\n"
                ),
            ),
            (
                PathBuf::from("model_1.go"),
                String::from(
                    "//line server/models/nested/Kind.go:1\npackage main; // kinds\ntype Kind string\n"
                ),
            ),
        ]
    );
}

#[test]
fn a_go_model_with_behavior_is_refused_when_read() {
    let root = tempfile::tempdir().expect("project");
    write(
        root.path(),
        "client/models/Guide.go",
        "type Guide struct{}\n\nfunc (Guide) Title() string { return \"\" }\n",
    );
    let Err(failure) = super::ModelSources::read(root.path(), "client", "go") else {
        panic!("a Go model method must be refused");
    };
    assert_eq!(failure.diagnostics()[0].code.number(), 1310);
}

#[test]
fn a_script_service_gets_only_the_models_it_names_and_the_models_they_name() {
    let root = tempfile::tempdir().expect("project");
    for (relative, source) in [
        (
            "client/models/Order.js",
            "/**\n * @typedef {Object} Order\n * @property {OrderLine[]} lines\n */\nexport {}\n",
        ),
        (
            "client/models/OrderLine.js",
            "/**\n * @typedef {Object} OrderLine\n * @property {Money} price\n */\nexport {}\n",
        ),
        (
            "client/models/Money.js",
            "/** @typedef {number} Money */\nexport {}\n",
        ),
        (
            "client/models/Unrelated.js",
            "/**\n * @typedef {Object} Unrelated\n * @property {string} note\n */\nexport {}\n",
        ),
        (
            "client/models/ProductInput.js",
            "@Checked\nexport default class ProductInput {\n  @Required\n  name;\n}\n",
        ),
    ] {
        write(root.path(), relative, source);
    }
    let models = super::ModelSources::read(root.path(), "client", "js").expect("models");
    let program = "export default class Orders {\n  /** @param {Order} order */\n  place(order) { return order; }\n}\n";
    let selected = models.used_by([program, program]).add_to_program(program);
    for kept in ["{OrderLine[]} lines", "{Money} price", "{number} Money"] {
        assert!(selected.contains(kept), "{kept}: {selected}");
    }
    assert!(!selected.contains("Unrelated"), "{selected}");
    assert!(!selected.contains("class ProductInput"), "{selected}");
    assert!(selected.ends_with(program), "{selected}");
    let checked = "export default class Inputs { make() { return new ProductInput(); } }\n";
    let selected = models.used_by(["", checked]).add_to_program(checked);
    assert!(selected.contains("class ProductInput"), "{selected}");
    assert!(!selected.contains("@typedef"), "{selected}");
    assert_eq!(
        models.used_by(["", "plain"]).add_to_program("plain"),
        "plain"
    );
}

#[test]
fn a_typescript_service_keeps_the_field_types_of_the_models_it_names() {
    let root = tempfile::tempdir().expect("project");
    for (relative, source) in [
        (
            "client/models/Guide.ts",
            "export interface Guide {\n  section: Section\n}\n",
        ),
        (
            "client/models/Section.ts",
            "export interface Section {\n  title: string\n}\n",
        ),
        (
            "client/models/Other.ts",
            "export interface Other {\n  count: number\n}\n",
        ),
    ] {
        write(root.path(), relative, source);
    }
    let models = super::ModelSources::read(root.path(), "client", "ts").expect("models");
    let program = "export default class Guides { read(guide: Guide): string { return guide.section.title; } }\n";
    let selected = models.used_by([program, program]).add_to_program(program);
    assert!(selected.contains("interface Guide"), "{selected}");
    assert!(selected.contains("interface Section"), "{selected}");
    assert!(!selected.contains("interface Other"), "{selected}");
}
