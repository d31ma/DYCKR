pub(crate) struct RequiredToolchain;

impl RequiredToolchain {
    const REQUIREMENT: &'static str = "TAC_TEST_UNITS_REQUIRE_TOOLCHAINS";
    const CONTINUOUS_INTEGRATION: &'static str = "CI";

    pub(crate) fn checked(program: &str, found: bool) -> bool {
        let required = [Self::CONTINUOUS_INTEGRATION, Self::REQUIREMENT]
            .iter()
            .any(|variable| std::env::var_os(variable).is_some());
        assert!(
            found || !required,
            "{program} is unavailable, but CI or {} is set, so this test must run instead of skipping",
            Self::REQUIREMENT
        );
        if !found {
            eprintln!("skipping: {program} is unavailable");
        }
        found
    }
}
