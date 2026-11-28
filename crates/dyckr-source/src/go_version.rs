pub struct GoVersion;

impl GoVersion {
    pub const OLDEST_MINOR: u32 = 27;

    pub fn supported(output: &str) -> bool {
        output
            .split_whitespace()
            .find_map(|word| word.strip_prefix("go1."))
            .is_some_and(Self::supported_minor)
    }

    fn supported_minor(version: &str) -> bool {
        let minor: String = version.chars().take_while(char::is_ascii_digit).collect();
        minor
            .parse::<u32>()
            .is_ok_and(|minor| minor >= Self::OLDEST_MINOR)
    }
}

#[cfg(test)]
mod tests {
    use super::GoVersion;

    #[test]
    fn go_1_27_and_newer_are_supported() {
        assert!(GoVersion::supported("go version go1.27.3 darwin/arm64"));
        assert!(GoVersion::supported("go version go1.30 linux/amd64"));
    }

    #[test]
    fn older_or_unreadable_go_versions_are_not_supported() {
        assert!(!GoVersion::supported("go version go1.26.9 darwin/arm64"));
        assert!(!GoVersion::supported("go version devel"));
        assert!(!GoVersion::supported(""));
    }
}
