use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, position) = path.into_inner();
    match state
        .projects
        .project_link(project_id)
        .stages()
        .detailed_stage(position, &state.pool)
        .await
    {
        Ok(stage) => HttpResponse::Ok().json(stage),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}