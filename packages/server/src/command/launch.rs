use std::fmt;
use actix_web::{HttpServer, App};
use actix_web::middleware::{Compress, Logger};
use actix_server::Server;
use mail::route::{receive_mail, send_mail};

use crate::route::{healthcheck, identifier};
use crate::command::parse::Arguments;
use crate::configuration::configure::{configure, ConfigurationError};
use crate::configuration::init::{init_logging, InitializeError};

#[derive(Debug)]
pub enum LaunchCommandError {
    Configure(ConfigurationError),
    Initialize(InitializeError),
    IO(std::io::Error)
}

impl fmt::Display for LaunchCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LaunchCommandError::Configure(error) => write!(formatter, "{}", error),
            LaunchCommandError::Initialize(error) => write!(formatter, "{}", error),
            LaunchCommandError::IO(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for LaunchCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            LaunchCommandError::Configure(ref error) => Some(error),
            LaunchCommandError::Initialize(ref error) => Some(error),
            LaunchCommandError::IO(ref error) => Some(error)
        }
    }
}

const LOG_FORMAT: &str = "%t %{r}a %r %s %bB %Dms";

pub fn launch(path: &Option<String>, arguments: &Arguments) -> Result<Server, LaunchCommandError> {
    let configuration = configure(path).map_err(LaunchCommandError::Configure)?;

    init_logging(&configuration.logging.path, &arguments.verbosity).map_err(LaunchCommandError::Initialize)?;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(LOG_FORMAT))
            .wrap(Compress::default())
            .service(receive_mail)
            .service(healthcheck)
            .service(identifier)
            .service(send_mail)
    })
    .bind(configuration.http.bind)
    .map_err(LaunchCommandError::IO)?
    .run();

    Ok(server)
}
