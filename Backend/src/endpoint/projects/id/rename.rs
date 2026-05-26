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
    match state
        .projects
        .project_link(id)
        .rename(body.title.trim(), &state.pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Project not found"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
