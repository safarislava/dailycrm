use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use crate::model::project::stage_summary::StageSummary;
use crate::model::project::stages::Stages;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use futures_util::future::try_join_all;
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let stages = Stages::new(state.pool.clone(), Project::new(path.into_inner()))
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let futures = stages.into_iter().map(|stage| {
        let detailed = StageSummary::new(state.pool.clone(), stage);
        async move { detailed.json().await }
    });
    let data = try_join_all(futures)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}
