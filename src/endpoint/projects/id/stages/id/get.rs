use actix_web::{HttpResponse, Responder, web};

pub async fn get(path: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok()
}
