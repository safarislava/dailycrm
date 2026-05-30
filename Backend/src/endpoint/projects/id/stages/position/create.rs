use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_insertion::StageInsertion;
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
    let project = Project::new(project_id);
    let title = body.title.clone();
    let task = StageInsertion::new(state.pool.clone(), project, position, title);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
