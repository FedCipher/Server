mod route;

use log::{error, debug};
use rocket::{launch, routes, Build, Rocket};
use common::setup_environment;
use route::receive_letter;

#[launch]
fn rocket() -> Rocket<Build> {
    match setup_environment() {
        Ok(_) => {
            debug!("Setup program envrionment")
        }
        Err(_) => {
            error!("Failed to setup program environment")
        }
    }

    rocket::build()
        .mount("/mail", routes![receive_letter])
}
