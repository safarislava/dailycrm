use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RenameProjectDto {
    title: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: Json<RenameProjectDto>,
) -> impl Responder {
    let id = path.into_inner();
    if body.title.trim().is_empty() {
        return HttpResponse::BadRequest().body("Title cannot be empty");
    }
    let project = match state.projects.project_by_id(id).await {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().body("Project not found"),
    };
    match project.rename(body.title.trim()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
