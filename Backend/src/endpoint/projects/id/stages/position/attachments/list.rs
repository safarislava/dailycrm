use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, stage_position) = path.into_inner();
    match state
        .projects
        .project(project_id)
        .stages()
        .stage(stage_position)
        .attachments()
        .list()
        .await
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
