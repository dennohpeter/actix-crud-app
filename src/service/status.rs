use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn status() -> HttpResponse {
    HttpResponse::Ok().json("Service is running")
}
