use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let project_id = path.into_inner();
    let project = match state.projects.project_by_id(project_id).await {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().body("Project not found"),
    };
    match project.remove().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
