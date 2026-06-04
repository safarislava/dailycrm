use crate::endpoint::api_error::ApiError;
use crate::model::project::comment_summaries::CommentSummaries;
use crate::model::project::contract::list::List;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, parent_position, position) = path.into_inner();
    let stage = Stage::new_substage(Project::new(project_id), parent_position, position);
    let items = CommentSummaries::new(state.pool.clone(), stage)
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(items))
}