use std::{env::current_dir, fs::read_dir};

#[cfg(windows)]
fn main() {
    use std::path::PathBuf;

    let cwd = current_dir().expect("failed to get current directory");

    // The directory containing the C files for Windows specific code
    let win_dir = cwd.join("lib/win");

    let dir = read_dir(win_dir).expect("failed to read windows directory");

    let dir_entries: Vec<PathBuf> = dir.map(|x| x.unwrap().path()).collect();

    let mut builder = cc::Build::new();

    builder.files(dir_entries).compile("lowwin");
}

#[cfg(not(windows))]
fn main() {}
