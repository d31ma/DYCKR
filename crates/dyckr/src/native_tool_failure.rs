use crate::failure::{Failure, diagnostic};

pub fn native_tool_failure(number: u16, message: &str) -> Failure {
    Failure::one(diagnostic(
        number,
        message,
        Some(String::from(
            "Install the selected platform toolchain and keep the previous output intact.",
        )),
        None,
    ))
}
