use cap_fs_ext::DirExt as _;
use cap_std::fs::{Dir, DirEntry};
use dyckr::migration::{MigrationFinding, MigrationStatus};
use std::path::Path;

pub struct OldTopicLogs;

impl OldTopicLogs {
    pub const OLD_PARENT: &'static str = ".tachyon";
    pub const OLD_NAME: &'static str = "topics";
    pub const OLD_LOCATION: &'static str = ".tachyon/topics";
    const LOG_EXTENSION: &'static str = ".jsonl";
    const NOT_READ: &'static str = "Old topic logs in .tachyon/topics cannot be read; move them to the topic log folder by hand.";

    pub fn migration_findings(root: &Path) -> Vec<MigrationFinding> {
        let found = Self::old_logs_at(root).is_some();
        found.then(Self::migration_finding).into_iter().collect()
    }
    fn old_logs_at(root: &Path) -> Option<(Dir, Vec<String>)> {
        let project = Dir::open_ambient_dir(root, cap_std::ambient_authority()).ok()?;
        Self::old_logs(&project).ok().flatten()
    }
    pub fn old_logs(project: &Dir) -> Result<Option<(Dir, Vec<String>)>, &'static str> {
        let old = project.open_dir_nofollow(Self::OLD_LOCATION).ok();
        let found = old.map(Self::with_log_names).transpose()?;
        Ok(found.filter(Self::has_logs))
    }
    fn with_log_names(old: Dir) -> Result<(Dir, Vec<String>), &'static str> {
        let names = Self::log_names(&old)?;
        Ok((old, names))
    }
    fn has_logs(found: &(Dir, Vec<String>)) -> bool {
        !found.1.is_empty()
    }
    pub fn log_names(directory: &Dir) -> Result<Vec<String>, &'static str> {
        let mut names = Vec::new();
        for entry in directory.entries().map_err(|_| Self::NOT_READ)? {
            names.extend(Self::log_name(&entry.map_err(|_| Self::NOT_READ)?));
        }
        Ok(names)
    }
    fn log_name(entry: &DirEntry) -> Option<String> {
        let name = entry.file_name().into_string().ok()?;
        let is_file = entry.file_type().is_ok_and(|kind| kind.is_file());
        (is_file && name.ends_with(Self::LOG_EXTENSION)).then_some(name)
    }
    fn migration_finding() -> MigrationFinding {
        MigrationFinding {
            source: String::from(Self::OLD_LOCATION),
            feature: String::from("server.topic_logs"),
            status: MigrationStatus::Changed,
            detail: String::from(
                "Topic logs now live in the folder that topics.directory in server/application.json names (default data/topics). .tachyon/ holds only generated files.",
            ),
            action: Some(String::from(
                "Start the server once: it moves these logs to the topic log folder and prints a notice. Or move them by hand.",
            )),
        }
    }
}
