use std::io;

use super::launch::launch;
use super::info::info;
use super::parse::{Arguments, Commands};

pub async fn execute(arguments: &Arguments) -> io::Result<()> {
    match &arguments.command {
        Commands::Launch { path } => {
            let server = launch(path, arguments)
                .await
                .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

            server.await
        },
        Commands::Info { path } => {
            info(path, arguments)
                .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

            Ok(())
        }
    }
}
