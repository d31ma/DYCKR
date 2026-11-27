use std::fs;
use std::io;
use std::path::Path;

pub struct Publish;

impl Publish {
    pub fn publish(stage: tempfile::TempDir, destination: &Path) -> io::Result<()> {
        let persisted = stage.keep();
        Self::move_into_place(&persisted, destination)
    }

    fn move_into_place(persisted: &Path, destination: &Path) -> io::Result<()> {
        if !destination.exists() {
            return fs::rename(persisted, destination);
        }
        Self::replace_existing(persisted, destination)
    }

    fn replace_existing(persisted: &Path, destination: &Path) -> io::Result<()> {
        let backup = persisted.with_extension("backup");
        fs::rename(destination, &backup)?;
        fs::rename(persisted, destination)
            .inspect_err(|_| Self::roll_back(persisted, destination, &backup))?;
        fs::remove_dir_all(&backup)
            .inspect_err(|_| Self::restore_backup(persisted, destination, &backup))
    }

    fn roll_back(persisted: &Path, destination: &Path, backup: &Path) {
        let _rollback = fs::rename(backup, destination);
        let _cleanup = fs::remove_dir_all(persisted);
    }

    fn restore_backup(persisted: &Path, destination: &Path, backup: &Path) {
        let move_new = fs::rename(destination, persisted);
        let restore_old = fs::rename(backup, destination);
        if move_new.is_ok() && restore_old.is_ok() {
            let _cleanup = fs::remove_dir_all(persisted);
        }
    }
}
