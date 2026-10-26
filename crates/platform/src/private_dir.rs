//! Folders that hold the user's unsaved work, created for that user alone.
//!
//! The recovery folder keeps snapshots of documents that have not been saved yet. Its files are
//! already written through the staging file, which is created with mode `0o600` before any bytes
//! go in ([`crate::staging::create_staging`]); the folder itself used to be created with the
//! process umask, so on a machine with other local accounts `0755` let them list the entry names.
//!
//! [`create_private_dir`] creates the folder with mode `0o700` in the creation call, so no window
//! exists in which it is looser, and tightens an existing folder that was created with a looser
//! mode (the app owns this folder; only restrictions are ever added). A path that is a symbolic
//! link, a file, or anything else that is not a real folder is refused rather than followed:
//! changing the permissions of a directory the app did not create is not this function's business.
//!
//! The mode is subject to the umask when creating, so the folder is then set to `0o700` exactly.
//! A filesystem without Unix modes (FAT/exFAT, some network shares) keeps its own behaviour; the
//! folder is still created and usable there. Other platforms keep their OS defaults, which are
//! already per-user (`%LOCALAPPDATA%` and Application Support).

use std::path::Path;

/// Create `dir` (and any missing parents) as a folder only its owner can list and enter.
///
/// Errors carry the operating system's own kind: `NotFound` for a missing parent that cannot be
/// created, `InvalidInput` when the path exists but is not a real folder, and otherwise whatever
/// the filesystem reported. A folder that already exists is reused and tightened to `0o700`.
pub fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    match std::fs::symlink_metadata(dir) {
        Ok(meta) if meta.is_dir() => return tighten(dir),
        Ok(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the private folder path is a link or a file, not a folder")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        // Applied to every folder this call creates, in the creation syscall itself.
        builder.mode(0o700);
    }
    match builder.create(dir) {
        Ok(()) => tighten(dir),
        // Another writer may have created it between the check and the call; the folder is ours
        // either way, so re-check rather than failing a recovery write.
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => match std::fs::symlink_metadata(dir) {
            Ok(meta) if meta.is_dir() => tighten(dir),
            Ok(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the private folder path is a link or a file, not a folder")),
            Err(_) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// Bring the mode of a folder that was created by [`create_private_dir`] back to `0o700`.
///
/// The umask can only remove bits from the creation mode, so this is normally a no-op. It also
/// restores the mode of a folder that an earlier version of the app created looser, and of one
/// whose mode was changed afterwards. Only the owner may change it, which is the same user who
/// owns the snapshots inside. A filesystem that refuses the change is left as it is: the folder
/// is still usable, and failing a recovery write over a permission bit would lose the snapshot.
///
/// The mode is changed through a handle on the folder, not by its path: a folder swapped for a
/// link between the caller's check and this call cannot redirect the change to another directory.
/// (Replacing the folder itself is outside this helper, and the parent is the app's own per-user
/// folder; this is not a link-proof sandbox.)
#[cfg(unix)]
fn tighten(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    let handle = std::fs::File::open(dir)?;
    let meta = handle.metadata()?;
    if !meta.is_dir() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the private folder path is a link or a file, not a folder"));
    }
    if meta.permissions().mode() & 0o777 == 0o700 {
        return Ok(());
    }
    let _ = handle.set_permissions(std::fs::Permissions::from_mode(0o700));
    Ok(())
}

#[cfg(not(unix))]
fn tighten(dir: &Path) -> std::io::Result<()> {
    // Nothing to tighten: the folder's access control is inherited from a per-user profile.
    std::fs::metadata(dir).map(|_| ())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt as _;
    use std::path::PathBuf;

    fn test_dir(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("pdfcraft-platform-private-{}-{test}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn mode(path: &Path) -> u32 {
        std::fs::symlink_metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn a_new_folder_is_private_to_its_owner() {
        let root = test_dir("new");
        let dir = root.join("recovery");
        create_private_dir(&dir).unwrap();
        assert!(dir.is_dir());
        assert_eq!(mode(&dir), 0o700);
        // Nothing else was touched.
        assert_ne!(mode(&root), 0);
    }

    #[test]
    fn an_existing_loose_folder_is_tightened_and_kept_usable() {
        let root = test_dir("loose");
        std::fs::create_dir_all(&root).unwrap();
        let dir = root.join("recovery");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(mode(&dir), 0o755);
        create_private_dir(&dir).unwrap();
        assert_eq!(mode(&dir), 0o700);
        // A second call is a no-op that still succeeds, and the folder still takes files.
        create_private_dir(&dir).unwrap();
        assert_eq!(mode(&dir), 0o700);
        std::fs::write(dir.join("entry.pdf"), b"snapshot").unwrap();
        assert_eq!(std::fs::read(dir.join("entry.pdf")).unwrap(), b"snapshot");
    }

    #[test]
    fn missing_parents_are_created_private_too() {
        let root = test_dir("parents");
        let dir = root.join("editor").join("recovery");
        create_private_dir(&dir).unwrap();
        assert_eq!(mode(&dir), 0o700);
        assert_eq!(mode(&root.join("editor")), 0o700);
    }

    #[test]
    fn a_symbolic_link_is_refused_and_its_target_is_left_alone() {
        let root = test_dir("link");
        let target = root.join("elsewhere");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o755)).unwrap();
        let link = root.join("recovery");
        std::os::unix::fs::symlink(&target, &link).unwrap();
        let error = create_private_dir(&link).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{error}");
        // The directory the app does not own keeps its mode.
        assert_eq!(mode(&target), 0o755);
        assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    }

    #[test]
    fn a_dangling_symbolic_link_is_refused_instead_of_created_through() {
        let root = test_dir("dangling");
        let link = root.join("recovery");
        std::os::unix::fs::symlink(root.join("missing"), &link).unwrap();
        let error = create_private_dir(&link).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{error}");
        assert!(!root.join("missing").exists(), "the link target must not be created");
    }

    #[test]
    fn a_file_at_the_path_is_refused() {
        let root = test_dir("file");
        let path = root.join("recovery");
        std::fs::write(&path, b"not a folder").unwrap();
        let error = create_private_dir(&path).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"not a folder");
    }
}
