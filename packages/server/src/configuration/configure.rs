use std::fs::read_to_string;
use std::fmt;
use log::Level;
use mail::configuration::MailConfiguration;
use serde::{Serialize, Deserialize};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logging {
    /// The location of the logging configuration file.
    pub path: Option<String>,

    /// The logging level to use.
    pub level: Option<Level>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    /// The local IP address & port to bind the HTTP server to.
    pub bind: (String, u16),

    /// The host name of this instance.
    pub host: String,

    /// An optional path prefix to serve the API on.
    pub directory: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Configuration {
    /// The logging configuration.
    pub logging: Logging,

    /// The HTTP server configuration.
    pub http: Http,

    /// The mail service configuration.
    pub mail: MailConfiguration
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
            bind: (String::from("localhost"), 8100),
            host: String::from("localhost:8100"),
            directory: None
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
