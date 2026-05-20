use actix_web::{post, HttpResponse, Responder};
use actix_web::web::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    id: String,
}

#[post("/")]
async fn approx_service(request: Json<Request>) -> impl Responder {
    HttpResponse::Ok()
}
