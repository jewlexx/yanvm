use parking_lot::{const_mutex, Mutex};

use crate::{config::Config, installer::ArchiveType};

pub const OS_STR: (&str, ArchiveType) = {
    if cfg!(windows) {
        ("win", ArchiveType::Zip)
    } else if cfg!(target_os = "macos") {
        ("darwin", ArchiveType::TarGz)
    } else if cfg!(target_os = "linux") {
        ("linux", ArchiveType::TarXz)
    } else {
        panic!("Unsupported OS");
    }
};

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub static CONFIG: Mutex<Config> = const_mutex(Config::blank());
