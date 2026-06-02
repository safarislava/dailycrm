use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::project::contract::list::List;
use crate::model::project::detailed_project::DetailedProject;
use crate::model::project::projects::Projects;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use futures_util::future::try_join_all;

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let projects = Projects::new(state.pool.clone())
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let futures = projects.into_iter().map(|project| {
        let detailed = DetailedProject::new(state.pool.clone(), project);
        async move { detailed.json().await }
    });
    let data = try_join_all(futures)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}
