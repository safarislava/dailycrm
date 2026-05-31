use crate::endpoint::api_error::ApiError;
use crate::model::task::contract::task::Task;
use crate::model::task::project::project_registration::ProjectRegistration;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct CreateProjectDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateProjectDto>,
) -> Result<HttpResponse, ApiError> {
    ProjectRegistration::new(state.pool.clone(), body.title.clone())
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Created().finish())
}
