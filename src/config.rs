use std::fs::{create_dir_all, read_to_string};

use serde::{Deserialize, Serialize};

use crate::helpers::ToError;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub versions: Vec<Version>,
    pub current: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Config {
    pub fn init() -> anyhow::Result<Self> {
        let dirs = directories::ProjectDirs::from("com", "jewelexx", "yanvm").to_error()?;

        let prefs_path = dirs.preference_dir();

        let config_path = prefs_path.join("config.toml");

        if !prefs_path.exists() {
            create_dir_all(prefs_path)?;
        }

        if config_path.exists() {
            let config = toml::from_str(&read_to_string(&config_path)?)?;

            Ok(config)
        } else {
            let config = Self {
                versions: vec![],
                current: None,
            };

            std::fs::write(&config_path, toml::to_string(&config)?)?;

            Ok(config)
        }
    }
}
