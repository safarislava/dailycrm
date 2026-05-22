use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(state: web::Data<AppState>, path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let (project_id, stage_id) = path.into_inner();
    match state.stages.remove(project_id, stage_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
