use std::fmt;

use crate::command::parse::Arguments;
use crate::configuration::configure::{configure, ConfigurationError};
use crate::configuration::init::{init_logging, InitializeError};

#[derive(Debug)]
pub enum InfoCommandError {
    Configure(ConfigurationError),
    Initialize(InitializeError)
}

impl fmt::Display for InfoCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoCommandError::Configure(error) => write!(formatter, "{}", error),
            InfoCommandError::Initialize(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for InfoCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            InfoCommandError::Configure(ref error) => Some(error),
            InfoCommandError::Initialize(ref error) => Some(error)
        }
    }
}

pub fn info(path: &Option<String>, arguments: &Arguments) -> Result<(), InfoCommandError> {
    let configuration = configure(path).map_err(InfoCommandError::Configure)?;

    init_logging(&configuration.logging.path, &arguments.verbosity).map_err(InfoCommandError::Initialize)?;

    println!("{:#?}", configuration);

    Ok(())
}
