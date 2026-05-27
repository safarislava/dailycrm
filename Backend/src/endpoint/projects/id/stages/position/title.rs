use crate::model::stage_title::StageTitle;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateTitleDto {
    title: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateTitleDto>,
) -> impl Responder {
    let (project_id, position) = path.into_inner();
    match StageTitle::new(project_id, position, body.title.clone(), state.pool.clone())
        .save()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
