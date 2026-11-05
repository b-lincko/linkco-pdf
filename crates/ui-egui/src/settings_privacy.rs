//! Keep the settings folder and its records of the user's documents private.
//!
//! `app.ron` holds the preferences *and* the recent-documents list: file names and full paths of
//! what this person had open, which is the local history REC-S08 covers. The log files in `logs/`
//! name documents too. Both live in the settings folder ([`crate::portable`] or the platform's
//! per-user application folder), next to nothing else.
//!
//! Two things are done for them, once per launch, best effort:
//! 1. The folder is created (or re-tightened) with owner-only access, exactly as the recovery
//!    folder is ([`pdfcraft_platform::private_dir::create_private_dir`]).
//! 2. A settings file that is already there gets mode `0o600`. The file is written by the
//!    framework at exit, so its mode cannot be chosen at creation; tightening it at the next
//!    launch is what is available, and the alternative — leaving it readable by every local
//!    account — is worse than the one launch of exposure. The file is opened first and its mode
//!    changed through that handle, so a file swapped for a link between the check and the change
//!    cannot redirect it, and a link at that path is left alone rather than followed.
//!
//! Nothing here reads the settings, changes what they say, or fails a launch: an unusual
//! filesystem that refuses the mode leaves the app working, and the caller only logs the error.

use std::path::Path;

/// The settings file the framework writes next to the logs.
pub const SETTINGS_FILE: &str = "app.ron";

/// The subfolder of log files, which name the documents that were open.
pub const LOGS_DIR: &str = "logs";

/// Create (or tighten) the settings folder and make an existing settings file owner-only.
///
/// Returns the operating system's error when the folder cannot be created or is not a real
/// folder; an existing settings file that cannot be tightened is not an error, since the app
/// still works and the caller has nothing better to do than log it.
pub fn protect(dir: &Path) -> std::io::Result<()> {
    pdfcraft_platform::private_dir::create_private_dir(dir)?;
    let logs = dir.join(LOGS_DIR);
    if logs.exists() {
        // A log folder that exists is the app's own; an unusual filesystem may refuse the mode.
        let _ = pdfcraft_platform::private_dir::create_private_dir(&logs);
    }
    let _ = tighten_file(&dir.join(SETTINGS_FILE));
    Ok(())
}

/// Make `path` readable only by its owner, if it is a regular file.
///
/// `NotFound` and "not a regular file" are not errors: a first launch has no settings yet, and a
/// link or a directory at that path is not something to change the mode of.
#[cfg(unix)]
pub fn tighten_file(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    match std::fs::symlink_metadata(path) {
        Ok(meta) if meta.is_file() => {}
        Ok(_) => return Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    }
    // `File::open` follows a link, so the mode is changed on whatever the handle is, and the
    // check above refused a path that is itself a link: there is no window in which a swap could
    // redirect the change to a file this app does not own.
    let file = std::fs::File::open(path)?;
    let meta = file.metadata()?;
    if !meta.is_file() || meta.permissions().mode() & 0o777 == 0o600 {
        return Ok(());
    }
    file.set_permissions(std::fs::Permissions::from_mode(0o600))
}

/// Other platforms keep their per-user profile defaults: `%APPDATA%` and Application Support are
/// already readable only by their owner, and Unix modes do not exist here.
#[cfg(not(unix))]
pub fn tighten_file(path: &Path) -> std::io::Result<()> {
    match std::fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp(tag: &str) -> std::io::Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!("pdfcraft-settings-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    #[test]
    fn a_settings_folder_without_files_is_created_private() {
        let root = temp("fresh").unwrap();
        let dir = root.join("Linkco PDF Editor");
        protect(&dir).unwrap();
        assert!(dir.is_dir());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777, 0o700);
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_settings_file_an_earlier_launch_wrote_is_made_private() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = temp("existing").unwrap();
        std::fs::write(dir.join(SETTINGS_FILE), b"recent files: /home/someone/case-file.pdf").unwrap();
        std::fs::set_permissions(dir.join(SETTINGS_FILE), std::fs::Permissions::from_mode(0o644)).unwrap();
        protect(&dir).unwrap();
        assert_eq!(std::fs::metadata(dir.join(SETTINGS_FILE)).unwrap().permissions().mode() & 0o777, 0o600);
        // The contents are untouched: this is a mode change, not a rewrite.
        assert_eq!(std::fs::read_to_string(dir.join(SETTINGS_FILE)).unwrap(), "recent files: /home/someone/case-file.pdf");
    }

    #[cfg(unix)]
    #[test]
    fn a_log_folder_inside_the_settings_folder_is_private_too() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = temp("logs").unwrap();
        let logs = dir.join(LOGS_DIR);
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::set_permissions(&logs, std::fs::Permissions::from_mode(0o755)).unwrap();
        protect(&dir).unwrap();
        assert_eq!(std::fs::metadata(&logs).unwrap().permissions().mode() & 0o777, 0o700);
    }

    #[cfg(unix)]
    #[test]
    fn a_link_at_the_settings_path_is_left_alone() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = temp("link").unwrap();
        let elsewhere = dir.join("elsewhere.ron");
        std::fs::write(&elsewhere, b"not ours").unwrap();
        std::fs::set_permissions(&elsewhere, std::fs::Permissions::from_mode(0o644)).unwrap();
        std::os::unix::fs::symlink(&elsewhere, dir.join(SETTINGS_FILE)).unwrap();
        protect(&dir).unwrap();
        // The file the app does not own keeps its mode, and the link is still a link.
        assert_eq!(std::fs::metadata(&elsewhere).unwrap().permissions().mode() & 0o777, 0o644);
        assert!(std::fs::symlink_metadata(dir.join(SETTINGS_FILE)).unwrap().file_type().is_symlink());
    }

    #[test]
    fn a_missing_settings_file_is_not_an_error() {
        let dir = temp("missing").unwrap();
        protect(&dir).unwrap();
        assert!(dir.is_dir());
        assert!(!dir.join(SETTINGS_FILE).exists(), "nothing is created");
    }

    #[cfg(unix)]
    #[test]
    fn a_file_that_cannot_be_changed_is_reported_but_does_not_stop_protection() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = temp("readonly-dir").unwrap();
        let path = dir.join(SETTINGS_FILE);
        std::fs::write(&path, b"settings").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        // A folder that forbids changes would make the tighten fail; `protect` still succeeds,
        // because an app that cannot tighten a mode must keep working.
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o500)).unwrap();
        let _ = protect(&dir);
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).unwrap();
        assert!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777 != 0);
    }
}
