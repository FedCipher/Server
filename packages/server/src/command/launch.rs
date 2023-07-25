use std::fmt;
use actix_web::{HttpServer, App};
use actix_web::middleware::{Compress, Logger, NormalizePath, TrailingSlash};
use actix_web::web::{scope, Data};
use actix_server::Server;
use common::state::CommonState;
use log::info;
use mail::route::{receive_mail, send_mail};
use mail::state::MailState;
use common::database::redis::{create_pool, RedisDatabaseError};

use crate::route::{healthcheck, id};
use crate::command::parse::Arguments;
use crate::configuration::configure::{configure, ConfigurationError};
use crate::configuration::init::{init_logging, InitializeError};

#[derive(Debug)]
pub enum LaunchCommandError {
    Configure(ConfigurationError),
    Initialize(InitializeError),
    IO(std::io::Error),
    Redis(RedisDatabaseError)
}

impl fmt::Display for LaunchCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LaunchCommandError::Configure(error) => write!(formatter, "{}", error),
            LaunchCommandError::Initialize(error) => write!(formatter, "{}", error),
            LaunchCommandError::IO(error) => write!(formatter, "{}", error),
            LaunchCommandError::Redis(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for LaunchCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            LaunchCommandError::Configure(ref error) => Some(error),
            LaunchCommandError::Initialize(ref error) => Some(error),
            LaunchCommandError::IO(ref error) => Some(error),
            LaunchCommandError::Redis(ref error) => Some(error)
        }
    }
}

const LOG_FORMAT: &str = "%t %{r}a %r %s %bB %Dms";
const API_VERSION: &str = "v1";

pub async fn launch(path: &Option<String>, arguments: &Arguments) -> Result<Server, LaunchCommandError> {
    let configuration = configure(path)
        .map_err(LaunchCommandError::Configure)?;

    init_logging(&configuration.logging.path, &arguments.verbosity)
        .map_err(LaunchCommandError::Initialize)?;

    let pool = create_pool("redis://localhost:6379")
        .map_err(LaunchCommandError::Redis)?;

    info!("Starting HTTP server at {}:{}", configuration.http.bind.0, configuration.http.bind.1);

    let root = match configuration.http.directory {
        Some(value) => format!("{}/{}", value, API_VERSION),
        None => String::from(API_VERSION)
    };

    let pool_data = Data::new(pool);
    let mail_state_data = Data::new(MailState::default());
    let common_state_data = Data::new(CommonState::default());
    let mail_configuration_data = Data::new(configuration.mail.clone());

    let bind = configuration.http.bind;
    let server = HttpServer::new(move || {
        let mail_scope = scope("mail")
            .app_data(pool_data.clone())
            .app_data(mail_state_data.clone())
            .app_data(mail_configuration_data.clone())
            .service(receive_mail)
            .service(send_mail);

        let root_scope = scope(&root)
            .app_data(common_state_data.clone())
            .service(healthcheck)
            .service(id)
            .service(mail_scope);

        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::new(LOG_FORMAT))
            .wrap(Compress::default())
            .service(root_scope)
    })
    .bind(bind)
    .map_err(LaunchCommandError::IO)?
    .run();

    Ok(server)
}
