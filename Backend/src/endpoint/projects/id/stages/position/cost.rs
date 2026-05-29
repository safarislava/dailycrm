use crate::model::project::contract::details::Details;
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
    let stage = state.projects.project(project_id).stages().stage(position);
    let result = match stage.details().await {
        Ok(details) => details.update_cost(body.cost).await,
        Err(error) => Err(error),
    };
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
