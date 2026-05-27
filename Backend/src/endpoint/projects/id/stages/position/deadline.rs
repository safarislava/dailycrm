use crate::model::stage_deadline::StageDeadline;
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
    match StageDeadline::new(project_id, position, body.deadline, state.pool.clone())
        .save()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}
