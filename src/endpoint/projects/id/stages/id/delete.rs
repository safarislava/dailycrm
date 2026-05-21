use actix_web::{HttpResponse, Responder};

pub async fn delete() -> impl Responder {
    HttpResponse::Ok()
}
