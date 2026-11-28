use std::fs::{self, Metadata};
use std::path::Path;

pub struct ToolBuild;

impl ToolBuild {
    pub fn id(version: &str, executable: Option<&Path>) -> String {
        let metadata = executable.and_then(|path| fs::metadata(path).ok());
        metadata.map_or_else(
            || String::from(version),
            |metadata| Self::with_build(version, &metadata),
        )
    }

    fn with_build(version: &str, metadata: &Metadata) -> String {
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |since| since.as_nanos());
        Self::format(version, metadata.len(), modified)
    }

    fn format(version: &str, size: u64, modified_nanos: u128) -> String {
        format!("{version}+{size}.{modified_nanos}")
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::ToolBuild;

    #[test]
    fn the_id_is_the_version_the_size_and_the_modified_time() {
        assert_eq!(
            ToolBuild::format("1.2.3-phase4", 125_905_872, 1_764_331_234_567_890_123),
            "1.2.3-phase4+125905872.1764331234567890123"
        );
    }

    #[test]
    fn a_missing_executable_gives_the_version_alone() {
        assert_eq!(ToolBuild::id("1.2.3", None), "1.2.3");
    }

    #[test]
    fn a_real_file_gives_its_size_and_modified_time() {
        let file = tempfile::NamedTempFile::new().expect("file");
        std::fs::write(file.path(), b"abc").expect("bytes");
        let metadata = std::fs::metadata(file.path()).expect("metadata");
        let nanos = metadata
            .modified()
            .expect("modified")
            .duration_since(std::time::UNIX_EPOCH)
            .expect("after 1970")
            .as_nanos();
        assert_eq!(
            ToolBuild::id("9.9.9", Some(file.path())),
            format!("9.9.9+3.{nanos}")
        );
    }

    #[test]
    fn a_new_version_gives_a_new_id_for_the_same_file() {
        let file = tempfile::NamedTempFile::new().expect("file");
        assert_ne!(
            ToolBuild::id("1.0.0", Some(file.path())),
            ToolBuild::id("1.0.1", Some(file.path()))
        );
    }
}
