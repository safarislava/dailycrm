use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_appending::StageAppending;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateStageDto {
    title: String,
}

pub async fn create(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: Json<CreateStageDto>,
) -> Result<HttpResponse, ApiError> {
    StageAppending::new(
        state.pool.clone(),
        Project::new(path.into_inner()),
        body.title.clone(),
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
