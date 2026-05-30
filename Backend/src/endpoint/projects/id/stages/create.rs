use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_appending::StageAppending;
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
    let project = Project::new(path.into_inner());
    let title = body.title.clone();
    let task = StageAppending::new(state.pool.clone(), project, title);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
