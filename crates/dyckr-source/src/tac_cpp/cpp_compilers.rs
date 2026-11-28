pub struct CppCompilers;

impl CppCompilers {
    pub const EMSCRIPTEN_VERSION: &'static str = "6.0.10";
    pub const TEST_COMPILER_SETTING: &'static str = "TAC_CPP_TEST_COMPILER";
    const DEFAULT_TEST_COMPILER: &'static str = "clang++";

    pub fn test_compiler() -> String {
        std::env::var(Self::TEST_COMPILER_SETTING)
            .unwrap_or_else(|_| String::from(Self::DEFAULT_TEST_COMPILER))
    }
}
