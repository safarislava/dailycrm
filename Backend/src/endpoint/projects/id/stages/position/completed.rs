use crate::model::project::contract::details::Details;
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
    let stage = state.projects.project(project_id).stages().stage(position);
    let result = match stage.details().await {
        Ok(details) => details.update_completed(body.completed).await,
        Err(error) => Err(error),
    };
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
