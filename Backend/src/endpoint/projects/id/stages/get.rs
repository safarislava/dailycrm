use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let project_id = path.into_inner();
    match state
        .projects
        .project_link(project_id)
        .stages()
        .list(&state.pool)
        .await
    {
        Ok(stages) => HttpResponse::Ok().json(stages),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
