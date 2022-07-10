use std::fs::{create_dir_all, read_to_string};

use serde::{Deserialize, Serialize};

use crate::{
    helpers::{NoneError, ToError},
    versions::Version,
};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{0}")]
    NoneError(#[from] NoneError),
    #[error("Failed to interact with IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to serialize toml: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Failed to deserilize toml: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub versions: Vec<Version>,
    pub current: Option<Version>,
}

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
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
