use std::fs::read_to_string;
use std::fmt;
use log::Level;
use serde::{Serialize, Deserialize};
use toml::from_str;

type Bind = (String, u16);

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    /// The location of the logging configuration file.
    pub path: Option<String>,

    /// The logging level to use.
    pub level: Option<Level>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Http {
    pub bind: Bind
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Configuration {
    pub logging: Logging,

    pub http: Http
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            path: Option::None,
            level: Option::Some(Level::Info)
        }
    }
}

impl Default for Http {
    fn default() -> Self {
        Self {
            bind: ("localhost".to_string(), 8100)
        }
    }
}

#[derive(Debug)]
pub enum ConfigurationError {
    Read(std::io::Error),
    Deserialize(toml::de::Error)
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationError::Read(error) => write!(formatter, "{}", error),
            ConfigurationError::Deserialize(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfigurationError::Read(ref error) => Some(error),
            ConfigurationError::Deserialize(ref error) => Some(error)
        }
    }
}

fn load_configuration_file(path: &String) -> Result<Configuration, ConfigurationError> {
    let value = read_to_string(path).map_err(ConfigurationError::Read)?;
    let configuration = from_str::<Configuration>(&value).map_err(ConfigurationError::Deserialize)?;

    Ok(configuration)
}

pub fn configure(path: &Option<String>) -> Result<Configuration, ConfigurationError> {
    let configuration = match path {
        Some(value) => load_configuration_file(value)?,
        None => Configuration::default()
    };

    Ok(configuration)
}
