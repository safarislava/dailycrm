use crate::model::stage_completed::StageCompleted;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateCompletedDto {
    completed: bool,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateCompletedDto>,
) -> impl Responder {
    let (project_id, position) = path.into_inner();
    match StageCompleted::new(project_id, position, body.completed, state.pool.clone())
        .save()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
