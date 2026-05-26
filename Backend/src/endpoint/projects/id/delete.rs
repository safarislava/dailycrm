use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let project_id = path.into_inner();
    match state.projects.project_link(project_id).remove().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Project not found"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
