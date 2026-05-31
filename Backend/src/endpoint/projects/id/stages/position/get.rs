use crate::endpoint::api_error::ApiError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::project::detailed_stage::DetailedStage;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, position) = path.into_inner();
    let stage = DetailedStage::new(
        state.pool.clone(),
        Stage::new(Project::new(project_id), position),
    );
    let data = stage
        .content()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}
