use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateStageDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: Json<CreateStageDto>,
) -> impl Responder {
    let project_id = path.into_inner();
    match state.stages.append(project_id, body.title.clone()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}