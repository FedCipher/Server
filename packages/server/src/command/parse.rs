use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true, propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbosity: Verbosity,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the HTTP server and wait
    Launch {
        #[arg(short = 'p', long = "config-path", help = "Path to configuration file")]
        path: Option<String>
    },
    /// Print application configuration information and exit
    Info {
        #[arg(short = 'p', long = "config-path", help = "Path to configuration file")]
        path: Option<String>
    }
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Verbosity {
    #[arg(short = 'v', long = "verbose", help = "Enable verbose output", global = true)]
    pub verbose: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress informational messages", global = true)]
    pub quiet: bool
}

pub fn parse() -> Arguments {
    Arguments::parse()
}
