use crate::toolchain_state::ToolchainState;
use dyckr::external_command::{ToolOutput, run_sync};
use std::time::Duration;

pub struct CommandProbe;

impl CommandProbe {
    const DEADLINE: Duration = Duration::from_secs(10);
    const OUTPUT_BYTES: usize = 64 * 1_024;

    pub fn version_state(program: &str, arguments: &[&str]) -> ToolchainState {
        Self::run(program, arguments).map_or(ToolchainState::Missing, ToolchainState::Ready)
    }

    pub fn run(program: &str, arguments: &[&str]) -> Option<String> {
        let text = Self::output(program, arguments)?;
        let first_line = text.lines().next()?;
        Some(first_line.trim().to_owned())
    }

    pub fn output(program: &str, arguments: &[&str]) -> Option<String> {
        let mut command = dyckr::kotlin_home::KotlinHome::command(program);
        command.args(arguments);
        let result = run_sync(&mut command, Self::DEADLINE, Self::OUTPUT_BYTES).ok()?;
        let result = Self::successful(result)?;
        Some(Self::readable_output(&result))
    }

    fn successful(result: ToolOutput) -> Option<ToolOutput> {
        result.status.success().then_some(result)
    }

    fn readable_output(result: &ToolOutput) -> String {
        let text = String::from_utf8_lossy(&result.stdout);
        if text.trim().is_empty() {
            return String::from_utf8_lossy(&result.stderr).into_owned();
        }
        text.into_owned()
    }
}
