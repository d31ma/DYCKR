use dyckr::external_command::ToolOutput;

use std::path::{Path, PathBuf};

use std::time::Duration;

fn executable_path(directory: &Path, name: &str) -> PathBuf {
    if cfg!(windows) {
        directory.join(format!("{name}.exe"))
    } else {
        directory.join(name)
    }
}

async fn run_successfully(command: &mut tokio::process::Command, description: &str) -> ToolOutput {
    let output = dyckr::external_command::run(command, Duration::from_mins(2), 65_536)
        .await
        .expect(description);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

async fn compile_and_run(
    source: &Path,
    executable: &Path,
    compiler_description: &str,
    execution_description: &str,
) -> String {
    let mut compiler = tokio::process::Command::new("rustc");
    compiler
        .args(["--edition", "2024"])
        .arg(source)
        .arg("-o")
        .arg(executable);
    run_successfully(&mut compiler, compiler_description).await;
    let mut command = tokio::process::Command::new(executable);
    let output = run_successfully(&mut command, execution_description).await;
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[tokio::test]
async fn rust_behavior_runtime_contracts_execute_with_owned_processes() {
    let root = tempfile::tempdir().expect("temporary executable directory");
    let executable = executable_path(root.path(), "behaviors");
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/behavior/rust_runtime_contract.rs");
    let output = compile_and_run(
        &source,
        &executable,
        "Rust compiler",
        "Rust runtime execution",
    )
    .await;
    assert!(output.contains("rust behavior runtime contracts passed"));
}

fn lower_client(source: &str) -> String {
    super::super::lower(Path::new("server/remotes/ReportClient.rs"), source, "yon")
        .expect("Rust gateway lowering")
        .expect("lowered source")
}

#[test]
fn a_gateway_method_calls_the_transport_without_a_behavior_context() {
    let lowered = lower_client(
        "struct ReportClient;\nimpl ReportClient {\n    #[Gateway({\"class\":\"server/services/ReportService.py\"})]\n    fn summarize(&self, mode: BehaviorValue, detail: BehaviorValue) -> BehaviorResult<BehaviorValue> {}\n}\n",
    );
    assert!(lowered.contains(
            "fn summarize(&self, mode: BehaviorValue, detail: BehaviorValue) -> BehaviorResult<BehaviorValue> {\nlet __tachyon_arguments = Behaviors::gateway_arguments(vec![mode, detail])?;\nBehaviors::gateway_result(BehaviorValue::from(YonGateway::call(\"ReportClient.summarize\", &__tachyon_arguments)))\n}"
        ));
    assert!(!lowered.contains("impl BehaviorService for ReportClient"));
}

#[test]
fn a_gateway_trait_becomes_a_struct_without_a_behavior_context() {
    let lowered = lower_client(
        "trait ReportClient {\n    #[Gateway({\"command\":[\"python3\"]})]\n    fn ping(&self) -> BehaviorResult<BehaviorValue>;\n}\n",
    );
    assert!(lowered.contains(
            "struct ReportClient {}\nimpl ReportClient { pub fn new() -> Self { Self {} } }\nimpl ReportClient {"
        ));
    assert!(lowered.contains("YonGateway::call(\"ReportClient.ping\", &__tachyon_arguments)"));
    assert!(!lowered.contains("behaviors: BehaviorContext"));
}

#[test]
fn a_gateway_parameter_must_be_a_behavior_value() {
    let failure = super::super::lower(
            Path::new("server/remotes/ReportClient.rs"),
            "struct ReportClient;\nimpl ReportClient {\n    #[Gateway({\"command\":[\"python3\"]})]\n    fn ping(&self, mode: String) -> BehaviorResult<BehaviorValue> {}\n}\n",
            "yon",
        )
        .expect_err("typed parameter");
    let message = failure.to_string();
    assert!(message.contains("TY1312"), "{message}");
    assert!(
        message.contains("must take BehaviorValue parameters"),
        "{message}"
    );
}
