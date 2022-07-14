use std::io;
use std::path::PathBuf;

#[cfg(not(windows))]
fn symlink_dir_unix(original: PathBuf, target: PathBuf) -> io::Result<()> {
    use std::fs::read_dir;
    use std::os::unix::fs::symlink;

    let og_dir = read_dir(original)?;

    for entry in og_dir {
        let entry = entry?;

        let name = entry.file_name();

        let base_path = entry.path();
        let target_path = target.join(name);

        symlink(base_path, target_path)?;
    }

    Ok(())
}

/// Should only be used to symlink one level directories
pub fn symlink_dir(original: PathBuf, target: PathBuf) -> io::Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            std::os::windows::fs::symlink_dir(original, target)?;
        } else {
            symlink_dir_unix(original, target)?;
        }
    }

    Ok(())
}
