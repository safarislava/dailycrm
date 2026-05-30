use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::detailed_stage::DetailedStage;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub async fn get(state: web::Data<AppState>, path: web::Path<(Uuid, i32)>) -> impl Responder {
    let (project_id, position) = path.into_inner();
    let project = Project::new(project_id);
    let stage = DetailedStage::new(state.pool.clone(), Stage::new(project, position));
    match stage.content().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
