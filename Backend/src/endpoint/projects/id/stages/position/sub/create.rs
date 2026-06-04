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
pub struct Body {
    title: String,
}

pub async fn post(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    body: Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, parent_position) = path.into_inner();
    StageAppending::sub(
        state.pool.clone(),
        Project::new(project_id),
        parent_position,
        body.title.clone(),
    )
    .done()
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}