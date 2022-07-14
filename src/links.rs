use std::io;
use std::path::Path;

// NOTE: These should only be used to symlink one level directories

#[cfg(windows)]
pub fn symlink_dir<P>(original: P, target: P)
where
    P: AsRef<Path>,
{
}

#[cfg(not(windows))]
pub fn symlink_dir<P>(original: P, target: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    use std::fs::read_dir;
    use std::os::unix::fs::symlink;

    let og_dir = read_dir(original);

    Ok(())
}
