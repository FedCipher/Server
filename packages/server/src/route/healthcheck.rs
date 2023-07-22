use actix_web::{get, HttpResponse, Responder};

#[get("/api/v1/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}
