use crate::endpoint::api_error::ApiError;
use crate::endpoint::projects::id::stages::media::JsonStageMedia;
use crate::model::contract::printer::Printer;
use crate::model::project::archive::stage_summaries::ArchivedStageSummaries;
use crate::model::project::id::ProjectId;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<ProjectId>,
) -> Result<HttpResponse, ApiError> {
    let project_id = path.into_inner();
    let mut media = JsonStageMedia::default();
    ArchivedStageSummaries::new(state.storage.clone(), project_id)
        .print(&mut media)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(media))
}
