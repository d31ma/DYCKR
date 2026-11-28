use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;

pub struct ProgramSearch;

impl ProgramSearch {
    pub fn on_path(paths: &OsStr, program: &Path) -> bool {
        std::env::split_paths(paths).any(|directory| Self::runnable_in(&directory, program))
    }

    pub fn windows_executable(directory: &Path, program: &Path) -> bool {
        directory.join(Self::windows_name(program)).is_file()
    }

    fn runnable_in(directory: &Path, program: &Path) -> bool {
        if cfg!(windows) {
            return Self::windows_executable(directory, program);
        }
        Self::executable_file(&directory.join(program))
    }

    fn windows_name(program: &Path) -> OsString {
        let mut name = program.as_os_str().to_os_string();
        if !name.as_encoded_bytes().contains(&b'.') {
            name.push(".exe");
        }
        name
    }

    #[cfg(unix)]
    fn executable_file(path: &Path) -> bool {
        fs::metadata(path).is_ok_and(|metadata| Self::is_executable(&metadata))
    }

    #[cfg(unix)]
    fn is_executable(metadata: &fs::Metadata) -> bool {
        use std::os::unix::fs::PermissionsExt;
        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    fn executable_file(path: &Path) -> bool {
        path.is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::ProgramSearch;
    use std::ffi::OsStr;
    use std::path::Path;

    #[test]
    fn a_program_in_no_listed_folder_is_not_on_path() {
        assert!(!ProgramSearch::on_path(
            OsStr::new(""),
            Path::new("tachyon-no-such-program")
        ));
    }

    #[test]
    fn a_windows_name_without_an_extension_gets_exe() {
        assert_eq!(
            ProgramSearch::windows_name(Path::new("go")),
            OsStr::new("go.exe")
        );
        assert_eq!(
            ProgramSearch::windows_name(Path::new("go.cmd")),
            OsStr::new("go.cmd")
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_shell_found_by_its_folder_is_on_path() {
        assert!(ProgramSearch::on_path(OsStr::new("/bin"), Path::new("sh")));
    }
}
