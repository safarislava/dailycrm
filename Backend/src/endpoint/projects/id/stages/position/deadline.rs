use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::deadline_update::DeadlineUpdate;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
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
) -> Result<HttpResponse, ApiError> {
    let (project_id, position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), position);
    DeadlineUpdate::new(state.pool.clone(), stage, body.deadline)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
