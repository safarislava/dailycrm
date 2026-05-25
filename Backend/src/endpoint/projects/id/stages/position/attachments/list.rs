use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, stage_position) = path.into_inner();
    match state.attachments.list(project_id, stage_position).await {
        Ok(attachments) => HttpResponse::Ok().json(attachments),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
