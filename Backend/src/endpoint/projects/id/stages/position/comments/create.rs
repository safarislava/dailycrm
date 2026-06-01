use crate::endpoint::api_error::ApiError;
use crate::endpoint::auth_header::UserHeader;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::model::task::contract::task::Task;
use crate::model::task::project::comment_creation::CommentCreation;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    text: String,
}

pub async fn post(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<(Uuid, i32)>,
    body: web::Json<Body>,
) -> Result<HttpResponse, ApiError> {
    let user = request
        .user()
        .ok_or(ApiError::Unauthorized("Unauthorized".to_string()))?;
    let (project_id, stage_position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), stage_position);
    let text = body.into_inner().text;
    if text.trim().is_empty() {
        return Err(ApiError::BadRequest("Text must not be empty".to_string()));
    }
    CommentCreation::new(state.pool.clone(), stage, user, text)
        .done()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Created().finish())
}