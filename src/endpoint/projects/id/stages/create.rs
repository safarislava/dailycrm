use actix_web::{HttpResponse, Responder};

pub async fn create() -> impl Responder {
    HttpResponse::Ok()
}
