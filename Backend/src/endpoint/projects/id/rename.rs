use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::project_rename::ProjectRename;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RenameProjectDto {
    title: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: Json<RenameProjectDto>,
) -> impl Responder {
    let project = Project::new(path.into_inner());
    let title = String::from(body.title.trim());
    if title.is_empty() {
        return HttpResponse::BadRequest().body("Title cannot be empty");
    }
    let task = ProjectRename::new(state.pool.clone(), project, title.clone());
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
