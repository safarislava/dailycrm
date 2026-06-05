use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::list::List;
use crate::model::project::project_summaries::ProjectSummaries;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let data = ProjectSummaries::new(state.pool.clone())
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(data))
}