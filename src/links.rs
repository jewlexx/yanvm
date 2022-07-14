use std::path::Path;

#[cfg(windows)]
pub fn symlink_dir<P>(original: P, target: P)
where
    P: AsRef<Path>,
{
}

#[cfg(not(windows))]
pub fn symlink_dir<P>(original: P, target: P)
where
    P: AsRef<Path>,
{
}
