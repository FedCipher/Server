use clap::{Parser, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = false)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[command(flatten)]
    pub verbosity: Verbosity
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Verbosity {
    #[arg(short = 'v', long = "verbose", help = "Enable verbose output", global = true)]
    pub verbose: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress informational messages", global = true)]
    pub quiet: bool
}
