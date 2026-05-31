use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::project_removal::ProjectRemoval;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    ProjectRemoval::new(state.pool.clone(), Project::new(path.into_inner()))
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
