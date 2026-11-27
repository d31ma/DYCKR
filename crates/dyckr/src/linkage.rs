use crate::build_profile::BuildProfile;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Linkage {
    Static,
    Shared,
    Web(BuildProfile),
}

impl Linkage {
    const DEVELOPMENT_ARGUMENTS: [&'static str; 8] = [
        "-C",
        "opt-level=s",
        "-C",
        "lto",
        "-C",
        "panic=abort",
        "-C",
        "strip=symbols",
    ];
    const RELEASE_ARGUMENTS: [&'static str; 10] = [
        "-C",
        "opt-level=z",
        "-C",
        "lto",
        "-C",
        "codegen-units=1",
        "-C",
        "panic=abort",
        "-C",
        "strip=symbols",
    ];

    pub const fn crate_type(self) -> &'static str {
        match self {
            Self::Static => "staticlib",
            Self::Shared | Self::Web(_) => "cdylib",
        }
    }

    pub const fn profile_arguments(self) -> &'static [&'static str] {
        match self {
            Self::Static | Self::Shared => &[],
            Self::Web(BuildProfile::Release) => &Self::RELEASE_ARGUMENTS,
            Self::Web(BuildProfile::Development) => &Self::DEVELOPMENT_ARGUMENTS,
        }
    }
}
