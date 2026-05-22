use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn get(state: web::Data<AppState>) -> impl Responder {
    match state.projects.projects().await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
