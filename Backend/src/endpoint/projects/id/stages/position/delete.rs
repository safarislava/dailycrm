use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, position) = path.into_inner();
    match state
        .projects
        .project_link(project_id)
        .stages()
        .stage_link(position)
        .remove()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
