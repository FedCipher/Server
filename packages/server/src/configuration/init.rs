use std::fmt;
use log::{LevelFilter, debug, SetLoggerError, Record};
use log::LevelFilter::{Warn, Info, Debug};
use log4rs::filter::{Filter, Response};
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::{Config, init_file};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{init_config, Root, Appender};
use log4rs::config::runtime::ConfigErrors;
use log4rs::encode::pattern::PatternEncoder;

use crate::command::parse::Verbosity;

const PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} {h({l})} {m}{n}";

fn stdout() -> ConsoleAppender {
    let target = Target::Stdout;
    let encoder = Box::new(
        PatternEncoder::new(PATTERN)
    );

    ConsoleAppender::builder()
        .target(target)
        .encoder(encoder)
        .build()
}

fn stderr() -> ConsoleAppender {
    let target = Target::Stderr;
    let encoder = Box::new(
        PatternEncoder::new(PATTERN)
    );

    ConsoleAppender::builder()
        .target(target)
        .encoder(encoder)
        .build()
}

#[derive(Debug)]
pub enum InitializeError {
    File(anyhow::Error),
    Config(ConfigErrors),
    SetLogger(SetLoggerError)
}

impl fmt::Display for InitializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitializeError::File(error) => write!(formatter, "{}", error),
            InitializeError::Config(error) => write!(formatter, "{}", error),
            InitializeError::SetLogger(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for InitializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            InitializeError::File(ref error) => error.source(),
            InitializeError::Config(ref error) => Some(error),
            InitializeError::SetLogger(ref error) => Some(error)
        }
    }
}

#[derive(Debug)]
pub struct InverseThresholdFilter {
    level: LevelFilter
}

impl InverseThresholdFilter {
    pub fn new(level: LevelFilter) -> Self {
        InverseThresholdFilter { level }
    }
}

impl Filter for InverseThresholdFilter {
    fn filter(&self, record: &Record) -> Response {
        if record.level() > self.level {
            Response::Neutral
        }
        else {
            Response::Reject
        }
    }
}

pub fn init_logging(path: &Option<String>, verbosity: &Verbosity) -> Result<(), InitializeError> {
    match path {
        Some(value) => {
            init_file(value, Default::default()).map_err(InitializeError::File)?;

            debug!("Initialized log4rs configuration from file at {}", value);
        },
        None => {
            let stdout = Appender::builder()
                .filter(
                    Box::new(
                        InverseThresholdFilter::new(LevelFilter::Error)
                    )
                )
                .build(
                    "stdout",
                    Box::new(
                        stdout()
                    )
                );
            let stderr = Appender::builder()
                .filter(
                    Box::new(
                        ThresholdFilter::new(LevelFilter::Error)
                    )
                )
                .build(
                    "stderr",
                    Box::new(
                        stderr()
                    )
                );

            let factory = |level: LevelFilter| {
                Root::builder().appenders(["stdout", "stderr"]).build(level)
            };

            let root = if verbosity.verbose {
                factory(Debug)
            }
            else if verbosity.quiet {
                factory(Warn)
            }
            else {
                factory(Info)
            };

            let config = Config::builder()
                .appenders([stdout, stderr])
                .build(root)
                .map_err(InitializeError::Config)?;

            init_config(config).map_err(InitializeError::SetLogger)?;

            debug!("Initialized default logging configuration");
        }
    };

    Ok(())
}
