use crate::model::positioned_stage::PositionedStage;
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
    match PositionedStage::new(project_id, position, body.title.clone(), state.pool.clone())
        .save()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
