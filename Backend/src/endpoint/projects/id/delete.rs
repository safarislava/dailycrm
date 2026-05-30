use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::project_removal::ProjectRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn delete(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let project = Project::new(path.into_inner());
    let task = ProjectRemoval::new(state.pool.clone(), project);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
