mod configuration;
mod command;
mod route;

use std::io::Result;
use actix_web::main;
use command::parse::parse;
use command::execute::execute;

#[main]
async fn main() -> Result<()> {
    let arguments = parse();

    execute(&arguments).await
}
