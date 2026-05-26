use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct InsertStageDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<InsertStageDto>,
) -> impl Responder {
    let (project_id, position) = path.into_inner();
    match state
        .projects
        .project_link(project_id)
        .stages()
        .register(position, body.title.clone(), &state.pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}