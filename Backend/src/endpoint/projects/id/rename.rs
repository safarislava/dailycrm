use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::task::contract::task::Task;
use crate::model::task::project::project_rename::ProjectRename;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RenameProjectDto {
    title: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: Json<RenameProjectDto>,
) -> Result<HttpResponse, ApiError> {
    let title = body.title.trim().to_string();
    if title.is_empty() {
        return Err(ApiError::BadRequest("Title cannot be empty".to_string()));
    }
    ProjectRename::new(state.pool.clone(), Project::new(path.into_inner()), title)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
