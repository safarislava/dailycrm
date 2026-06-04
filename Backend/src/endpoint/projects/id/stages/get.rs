use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use crate::model::project::stage_summaries::StageSummaries;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let data = StageSummaries::new(state.pool.clone(), Project::new(path.into_inner()))
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}