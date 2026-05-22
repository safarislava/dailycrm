use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn get(state: web::Data<AppState>) -> impl Responder {
    match state.stages.deadlines().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}