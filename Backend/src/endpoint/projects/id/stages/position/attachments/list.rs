use crate::endpoint::api_error::ApiError;
use crate::model::project::contract::json::Json;
use crate::model::project::attachments::Attachments;
use crate::model::project::contract::list::List;
use crate::model::project::detailed_attachment::DetailedAttachment;
use crate::model::project::project::Project;
use crate::model::project::stage::Stage;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use futures_util::future::try_join_all;
use uuid::Uuid;

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (project_id, stage_position) = path.into_inner();
    let stage = Stage::new(Project::new(project_id), stage_position);
    let list = Attachments::new(state.pool.clone(), stage)
        .items()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let futures = list.into_iter().map(|attachment| {
        let detailed = DetailedAttachment::new(state.pool.clone(), attachment);
        async move { detailed.json().await }
    });
    let items = try_join_all(futures)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(items))
}
