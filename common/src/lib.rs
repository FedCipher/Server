pub mod command;
pub mod model;

use clap::Parser;
use command::Arguments;
use log::{LevelFilter::{Warn, Trace}, debug, error};
use log4rs::config::{init_config, load_config_file};

pub fn setup_environment() -> Result<(), ()> {
    let arguments = Arguments::parse();
    let config_path = "config/log4rs.yaml";

    let config = load_config_file(config_path, Default::default());

    match config {
        Ok(mut value) => {
            debug!("Loaded logging configuration file at {}", config_path);

            if arguments.verbosity.verbose {
                value.root_mut().set_level(Trace);
            }

            if arguments.verbosity.quiet {
                value.root_mut().set_level(Warn);
            }

            match init_config(value) {
                Ok(_) => {
                    debug!("Initialized logging configuration");

                    Ok(())
                }
                Err(_) => {
                    error!("Failed to initialize logging configuration");

                    Err(())
                }
            }
        }
        Err(_) => {
            error!("Failed to load logging configuration file at {}", config_path);

            Err(())
        }
    }


}
