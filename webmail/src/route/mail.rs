use log::info;
use rocket::post;
use rocket::serde::json::Json;
use common::model::Letter;

#[post("/", data = "<data>")]
pub async fn receive_letter(data: Json<Letter>) -> Result<(), ()> {
    let letter = data.into_inner();

    info!("{:?}", letter);

    Ok(())
}
