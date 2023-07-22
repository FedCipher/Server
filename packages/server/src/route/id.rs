use actix_web::{get, Responder, Result};
use actix_web::web::Json;
use common::model::Identifier;

#[get("/api/v1/id")]
pub async fn identifier() -> Result<impl Responder> {
    let identifier = Identifier::new();
    let response = Json(identifier);

    Ok(response)
}
