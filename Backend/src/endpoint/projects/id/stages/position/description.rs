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
    match state
        .stages
        .stage_link(project_id, position)
        .update_description(body.description.clone())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}