use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::cost_update::CostUpdate;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateCostDto {
    cost: Option<i32>,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateCostDto>,
) -> impl Responder {
    let (project_id, position) = path.into_inner();
    let project = Project::new(project_id);
    let stage = Stage::new(project, position);
    let cost = body.cost;
    let task = CostUpdate::new(state.pool.clone(), stage, cost);
    match task.done().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
