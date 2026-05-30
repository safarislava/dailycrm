use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::description_update::DescriptionUpdate;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateDescriptionDto {
    description: Option<String>,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateDescriptionDto>,
) -> impl Responder {
    let (project_id, position) = path.into_inner();
    let project = Project::new(project_id);
    let stage = Stage::new(project, position);
    let description = body.description.clone();
    let task = DescriptionUpdate::new(state.pool.clone(), stage, description);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
