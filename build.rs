use std::{env::current_dir, fs::read_dir};

#[cfg(windows)]
fn main() {
    let cwd = current_dir().expect("failed to get current directory");

    // The directory containing the C files for Windows specific code
    let win_dir = cwd.join("lib/win");
}

#[cfg(not(windows))]
fn main() {}
