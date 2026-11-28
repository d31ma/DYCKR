use std::path::Path;

#[test]
fn every_composed_module_keeps_its_own_file_and_line_numbers() {
    let root = tempfile::tempdir().unwrap_or_else(|error| panic!("{error}"));
    let service =
        "open System\n\n[<Service>]\ntype CatalogService() =\n    member this.Total() : int = 2\n";
    let unit = "[<Test(typeof<CatalogTestSetup>)>]\ntype CatalogServiceTest() =\n    member this.testTotals() : unit =\n        Assert.equals(2, CatalogService().Total())\n";
    for (relative, source) in [
        ("server/services/CatalogService.fs", service),
        (
            "server/tests/CatalogTestSetup.fs",
            "[<TestSetup>]\ntype CatalogTestSetup() = class end\n",
        ),
        ("server/tests/CatalogServiceTest.fs", unit),
    ] {
        let path = root.path().join(relative);
        std::fs::create_dir_all(path.parent().unwrap_or_else(|| panic!("parent")))
            .unwrap_or_else(|error| panic!("{error}"));
        std::fs::write(path, source).unwrap_or_else(|error| panic!("{error}"));
    }
    let entry = Path::new("server/tests/CatalogServiceTest.fs");
    let composed = crate::annotation_modules::assemble(root.path(), entry, unit, "yon")
        .unwrap_or_else(|failure| panic!("{failure:?}"))
        .unwrap_or_else(|| panic!("composed"));
    assert!(composed.starts_with("open System\n"), "{composed}");
    assert!(
        composed.contains("# 1 \"server/services/CatalogService.fs\"\n\n\n[<Service>]\n"),
        "{composed}"
    );
    assert!(
        composed.contains("# 1 \"server/tests/CatalogServiceTest.fs\"\n[<Test("),
        "{composed}"
    );
}
