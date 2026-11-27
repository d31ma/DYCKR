use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct TypeScriptProgram;

impl TypeScriptProgram {
    #[cfg(target_arch = "aarch64")]
    const WINDOWS_ARCHITECTURES: &'static [&'static str] = &["arm64", "x64"];
    #[cfg(not(target_arch = "aarch64"))]
    const WINDOWS_ARCHITECTURES: &'static [&'static str] = &["x64"];
    pub fn program(tools_root: &Path) -> PathBuf {
        let search_path = std::env::var_os("PATH").unwrap_or_default();
        let on_windows = cfg!(windows).then(|| Self::windows_program(tools_root, &search_path));
        on_windows.unwrap_or_else(|| Self::unix_program(tools_root))
    }

    pub fn unix_program(tools_root: &Path) -> PathBuf {
        let local = tools_root.join("node_modules/.bin/tsc");
        if local.is_file() {
            local
        } else {
            PathBuf::from("tsc")
        }
    }

    pub fn windows_program(tools_root: &Path, search_path: &OsStr) -> PathBuf {
        let mut folders = vec![tools_root.join("node_modules").join(".bin")];
        folders.extend(std::env::split_paths(search_path));
        let found = folders
            .iter()
            .find_map(|folder| Self::windows_program_in(folder));
        found.unwrap_or_else(|| PathBuf::from("tsc"))
    }

    pub fn windows_program_in(folder: &Path) -> Option<PathBuf> {
        let native = folder.join("tsc.exe");
        let standalone = native.is_file().then_some(native);
        standalone.or_else(|| Self::native_compiler_behind_shim(folder))
    }

    pub fn native_compiler_behind_shim(folder: &Path) -> Option<PathBuf> {
        let has_shim = folder.join("tsc.cmd").is_file();
        let package = has_shim.then(|| Self::package_of_shim(folder))?;
        Self::native_compiler_of(&package)
    }

    pub fn package_of_shim(folder: &Path) -> PathBuf {
        let in_project = folder.file_name() == Some(OsStr::new(".bin"));
        let project_modules = folder.parent().filter(|_| in_project);
        let global_modules = folder.join("node_modules");
        let node_modules = project_modules.unwrap_or(&global_modules);
        node_modules.join("typescript")
    }

    pub fn native_compiler_of(package: &Path) -> Option<PathBuf> {
        let nested = package.join("node_modules");
        let hoisted = package.parent()?.to_path_buf();
        let mut candidates = Vec::new();
        for architecture in Self::WINDOWS_ARCHITECTURES {
            candidates.push(Self::native_compiler_in(&nested, architecture));
            candidates.push(Self::native_compiler_in(&hoisted, architecture));
        }
        candidates.into_iter().find(|candidate| candidate.is_file())
    }

    pub fn native_compiler_in(node_modules: &Path, architecture: &str) -> PathBuf {
        let platform_package = format!("typescript-win32-{architecture}");
        node_modules
            .join("@typescript")
            .join(platform_package)
            .join("lib")
            .join("tsc.exe")
    }
}
