use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_reordering::StageReordering;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReorderSubStageDto {
    to: i32,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, i32)>,
    body: Json<ReorderSubStageDto>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, parent_position, position) = path.into_inner();
    StageReordering::sub(
        state.pool.clone(),
        Project::new(project_id),
        parent_position,
        position,
        body.to,
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}