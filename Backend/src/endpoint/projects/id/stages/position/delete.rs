use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_removal::StageRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), position);
    StageRemoval::new(state.pool.clone(), stage)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
