use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, position) = path.into_inner();
    match state.stages.detailed_stage(project_id, position).await {
        Ok(stage) => HttpResponse::Ok().json(stage),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}