use crate::contract::Details;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, position) = path.into_inner();
    let details = match state
        .projects
        .project(project_id)
        .stages()
        .stage(position)
        .details()
        .await
    {
        Ok(details) => details,
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };
    match details.data().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Stage not found"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
