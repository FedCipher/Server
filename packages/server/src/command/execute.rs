use std::io::{Result, Error, ErrorKind};

use super::launch::launch;
use super::info::info;
use super::parse::{Arguments, Commands};

pub async fn execute(arguments: &Arguments) -> Result<()> {
    match &arguments.command {
        Commands::Launch { path } => {
            let server = launch(path, arguments).map_err(|error| {
                Error::new(ErrorKind::Other, error)
            })?;

            server.await
        },
        Commands::Info { path } => {
            info(path, arguments).map_err(|error| {
                Error::new(ErrorKind::Other, error)
            })?;

            Ok(())
        }
    }
}
