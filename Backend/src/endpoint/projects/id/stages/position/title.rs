use crate::endpoint::api_error::ApiError;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::stage_rename::StageRename;
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateTitleDto {
    title: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<UpdateTitleDto>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), position);
    StageRename::new(state.pool.clone(), stage, body.title.clone())
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}
