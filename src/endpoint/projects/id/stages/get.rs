use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let project_id = path.into_inner();
    match state.stage_service.stages(project_id).await {
        Ok(stages) => HttpResponse::Ok().json(stages),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
