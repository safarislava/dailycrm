use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::deadline_update::DeadlineUpdate;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateDeadlineDto {
    deadline: Option<DateTime<Utc>>,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateDeadlineDto>,
) -> impl Responder {
    let (project_id, position) = path.into_inner();
    let project = Project::new(project_id);
    let stage = Stage::new(project, position);
    let deadline = body.deadline;
    let task = DeadlineUpdate::new(state.pool.clone(), stage, deadline);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
