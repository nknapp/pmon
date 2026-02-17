use crate::core::config::{Config, create_default_config};
use std::path::Path;
use std::{fmt, fs};

pub fn read_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    if !fs::exists(path.as_ref())? {
        write_config(&path, &create_default_config())?
    }
    log::info!("Reading config from {}", path.as_ref().display());
    let contents = fs::read_to_string(path)?;
    let raw_config: serde_yaml::Value = serde_yaml::from_str(&contents)?;
    let config = serde_yaml::from_value(raw_config)?;
    log::debug!("Config: {config:?}");
    Ok(config)
}

pub fn write_config(path: impl AsRef<Path>, config: &Config) -> Result<(), ConfigError> {
    let raw_config = serde_yaml::to_value(config)?;
    fs::write(path, serde_yaml::to_string(&raw_config)?)?;
    Ok(())
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "Failed to read config file: {}", err),
            ConfigError::Parse(err) => write!(f, "Failed to parse config file: {}", err),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::Parse(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Parse(error)
    }
}
