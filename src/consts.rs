pub const OS_STR: (&str, &str) = {
    if cfg!(windows) {
        ("win", "zip")
    } else if cfg!(target_os = "macos") {
        ("darwin", "tar.gz")
    } else if cfg!(target_os = "linux") {
        ("linux", "tar.xz")
    } else {
        panic!("Unsupported OS");
    }
};

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
