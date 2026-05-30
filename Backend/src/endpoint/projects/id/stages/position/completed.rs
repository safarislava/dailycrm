use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::completion_update::CompletionUpdate;
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
    let project = Project::new(project_id);
    let stage = Stage::new(project, position);
    let completed = body.completed;
    let task = CompletionUpdate::new(state.pool.clone(), stage, completed);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
